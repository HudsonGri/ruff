use ruff_db::files::File;
use ruff_db::parsed::parsed_module;
use ruff_python_ast as ast;
use ruff_python_ast::name::Name;
use rustc_hash::FxHashMap;
use ty_module_resolver::{KnownModule, ModuleName, resolve_module_confident};

use crate::{
    Db, FxIndexMap,
    place::{imported_symbol, known_module_symbol},
    semantic_index::{definition::DefinitionKind, global_scope, semantic_index},
    types::{
        ClassLiteral, KnownClass, Parameter, Parameters, Signature, Type, UnionType, binding_type,
        class::StaticClassLiteral, member::class_member,
    },
};

/// Classifies Django field types for mapping to Python types in `get_type`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, salsa::Update, get_size2::GetSize)]
pub(super) enum DjangoFieldKind {
    Char,
    Integer,
    Float,
    Bool,
    Date,
    DateTime,
    Time,
    Decimal,
    Uuid,
    Json,
    Binary,
    Auto,
    ForeignKey,
    OneToOne,
    ManyToMany,
    Unknown,
}

/// Metadata for a single field declaration in a Django model class body.
#[derive(Debug, Clone, PartialEq, Eq, Hash, salsa::Update, get_size2::GetSize)]
pub(super) struct DjangoFieldInfo<'db> {
    pub name: Name,
    pub kind: DjangoFieldKind,
    pub nullable: bool,
    pub primary_key: bool,
    pub related_model: Option<Type<'db>>,
}

/// Returns the instance type of a stdlib class, or `Any` if the class can't be found.
fn resolve_stdlib_instance<'db>(
    db: &'db dyn Db,
    module: KnownModule,
    class_name: &str,
) -> Type<'db> {
    known_module_symbol(db, module, class_name)
        .place
        .ignore_possibly_undefined()
        .and_then(|ty| ty.to_instance(db))
        .unwrap_or_else(Type::unknown)
}

/// Build a `Manager[model_ty]` instance type by resolving the actual
/// `django.db.models.manager.Manager` class and specializing it.
///
/// We can't go through `KnownClass::DjangoManager.to_specialized_instance` because
/// `canonical_module` returns a Builtins placeholder for third-party classes, so the
/// lookup never finds `Manager`.
fn resolve_specialized_manager<'db>(db: &'db dyn Db, model_ty: Type<'db>) -> Type<'db> {
    let Some(module_name) = ModuleName::new("django.db.models.manager") else {
        return Type::unknown();
    };
    let Some(module) = resolve_module_confident(db, &module_name) else {
        return Type::unknown();
    };
    let Some(file) = module.file(db) else {
        return Type::unknown();
    };
    let scope = global_scope(db, file);
    let Some(ty) = class_member(db, scope, "Manager").ignore_possibly_undefined() else {
        return Type::unknown();
    };
    let Type::ClassLiteral(ClassLiteral::Static(manager_lit)) = ty else {
        return Type::unknown();
    };
    let Some(generic_context) = manager_lit.generic_context(db) else {
        return Type::unknown();
    };
    if generic_context.len(db) != 1 {
        return Type::unknown();
    }
    let class_type =
        manager_lit.apply_specialization(db, |_| generic_context.specialize(db, &[model_ty]));
    Type::from(class_type)
        .to_instance(db)
        .unwrap_or_else(Type::unknown)
}

/// Given a related model's instance type, find its primary key type.
/// Falls back to `int` if the model can't be inspected.
fn resolve_related_pk_type<'db>(db: &'db dyn Db, related_instance_ty: Type<'db>) -> Type<'db> {
    let fallback = || KnownClass::Int.to_instance(db);
    let Some(instance) = related_instance_ty.as_nominal_instance() else {
        return fallback();
    };
    let Some((lit, _)) = instance.class(db).static_class_literal(db) else {
        return fallback();
    };
    let all_fields = collect_all_django_fields(db, lit);
    resolve_pk_type(db, &all_fields)
}

impl<'db> DjangoFieldInfo<'db> {
    /// Returns the Python type for this field on an instance (e.g. `CharField` → `str`).
    pub(super) fn get_type(&self, db: &'db dyn Db) -> Type<'db> {
        let base = match self.kind {
            DjangoFieldKind::Char => KnownClass::Str.to_instance(db),
            DjangoFieldKind::Integer | DjangoFieldKind::Auto => KnownClass::Int.to_instance(db),
            DjangoFieldKind::Float => KnownClass::Float.to_instance(db),
            DjangoFieldKind::Bool => KnownClass::Bool.to_instance(db),
            DjangoFieldKind::Date => resolve_stdlib_instance(db, KnownModule::Datetime, "date"),
            DjangoFieldKind::DateTime => {
                resolve_stdlib_instance(db, KnownModule::Datetime, "datetime")
            }
            DjangoFieldKind::Time => resolve_stdlib_instance(db, KnownModule::Datetime, "time"),
            DjangoFieldKind::Decimal => {
                resolve_stdlib_instance(db, KnownModule::Decimal, "Decimal")
            }
            DjangoFieldKind::Uuid => resolve_stdlib_instance(db, KnownModule::Uuid, "UUID"),
            DjangoFieldKind::Binary => KnownClass::Bytes.to_instance(db),
            DjangoFieldKind::Json | DjangoFieldKind::Unknown => Type::any(),
            DjangoFieldKind::ForeignKey | DjangoFieldKind::OneToOne => {
                self.related_model.unwrap_or_else(Type::unknown)
            }
            DjangoFieldKind::ManyToMany => {
                if let Some(related) = self.related_model {
                    resolve_specialized_manager(db, related)
                } else {
                    Type::unknown()
                }
            }
        };
        if self.nullable {
            UnionType::from_two_elements(db, base, Type::none(db))
        } else {
            base
        }
    }
}

/// Map a field class name (e.g. `"CharField"`) to a `DjangoFieldKind`.
fn field_name_to_kind(field_class_name: &str) -> DjangoFieldKind {
    match field_class_name {
        "CharField"
        | "TextField"
        | "SlugField"
        | "URLField"
        | "EmailField"
        | "GenericIPAddressField"
        | "IPAddressField"
        | "FilePathField" => DjangoFieldKind::Char,
        // FieldFile/ImageFieldFile aren't modeled yet, so these fall back to Any.
        "FileField" | "ImageField" => DjangoFieldKind::Json,
        "IntegerField"
        | "SmallIntegerField"
        | "BigIntegerField"
        | "PositiveIntegerField"
        | "PositiveSmallIntegerField"
        | "PositiveBigIntegerField" => DjangoFieldKind::Integer,
        "FloatField" => DjangoFieldKind::Float,
        "BooleanField" | "NullBooleanField" => DjangoFieldKind::Bool,
        "DateField" => DjangoFieldKind::Date,
        "DateTimeField" => DjangoFieldKind::DateTime,
        "TimeField" => DjangoFieldKind::Time,
        "DecimalField" => DjangoFieldKind::Decimal,
        "UUIDField" => DjangoFieldKind::Uuid,
        "JSONField" => DjangoFieldKind::Json,
        "BinaryField" => DjangoFieldKind::Binary,
        "AutoField" | "BigAutoField" | "SmallAutoField" => DjangoFieldKind::Auto,
        "ForeignKey" | "ForeignObject" => DjangoFieldKind::ForeignKey,
        "OneToOneField" => DjangoFieldKind::OneToOne,
        "ManyToManyField" => DjangoFieldKind::ManyToMany,
        _ => DjangoFieldKind::Unknown,
    }
}

/// Attempt to resolve an unknown field class name to a known Django field kind by walking
/// the field class's MRO. For example, `PhoneNumberField(CharField)` resolves to `Char`
/// because `CharField` appears in its MRO.
fn resolve_custom_field_kind(db: &dyn Db, file: File, class_name: &str) -> DjangoFieldKind {
    let module_scope = global_scope(db, file);
    let member = class_member(db, module_scope, class_name);
    let Some(ty) = member.ignore_possibly_undefined() else {
        return DjangoFieldKind::Unknown;
    };
    let Type::ClassLiteral(ClassLiteral::Static(lit)) = ty else {
        return DjangoFieldKind::Unknown;
    };
    for base in lit.iter_mro(db, None) {
        let Some(class_type) = base.into_class() else {
            continue;
        };
        let Some((base_lit, _)) = class_type.static_class_literal(db) else {
            continue;
        };
        let kind = field_name_to_kind(base_lit.name(db).as_str());
        if !matches!(kind, DjangoFieldKind::Unknown) {
            return kind;
        }
    }
    DjangoFieldKind::Unknown
}

/// Look up a Django exception class, falling back to `type[Exception]`.
fn resolve_django_exception<'db>(db: &'db dyn Db, class_name: &str) -> Type<'db> {
    let fallback = KnownClass::Exception.to_class_literal(db);
    let Some(module_name) = ModuleName::new("django.core.exceptions") else {
        return fallback;
    };
    let Some(module) = resolve_module_confident(db, &module_name) else {
        return fallback;
    };
    let Some(file) = module.file(db) else {
        return fallback;
    };
    imported_symbol(db, Some(file), class_name, None)
        .place
        .ignore_possibly_undefined()
        .unwrap_or(fallback)
}

/// A single reverse-relation entry in the project-wide index.
#[derive(Debug, Clone, PartialEq, Eq, Hash, salsa::Update, get_size2::GetSize)]
struct ReverseRelationEntry {
    source_model_name: Name,
    source_file: File,
    field_name: Name,
    kind: DjangoFieldKind,
    related_name: Option<String>,
    nullable: bool,
}

/// A relation declaration extracted from a single file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, salsa::Update, get_size2::GetSize)]
struct RelationDecl {
    source_model_name: Name,
    source_file: File,
    field_name: Name,
    kind: DjangoFieldKind,
    related_name: Option<String>,
    nullable: bool,
    target_model_name: Name,
}

/// Per-file extraction of FK/O2O/M2M declarations from Django model classes.
#[salsa::tracked(returns(ref), cycle_initial = |_, _, _| Box::default(), heap_size = ruff_memory_usage::heap_size)]
fn django_relations_in_file<'db>(db: &'db dyn Db, file: File) -> Box<[RelationDecl]> {
    if !db.analysis_settings(file).django {
        return Box::default();
    }
    let index = semantic_index(db, file);
    let module = parsed_module(db, file).load(db);
    let mut decls = Vec::new();

    for scope_id in index.scope_ids() {
        let scope_node = scope_id.node(db);
        let Some(class_node) = scope_node.as_class() else {
            continue;
        };

        let def = index.expect_single_definition(class_node);
        if !matches!(def.kind(db), DefinitionKind::Class(_)) {
            continue;
        }

        let file_scope_id = scope_id.file_scope_id(db);
        if !index.is_scope_reachable(db, file_scope_id) {
            continue;
        }

        let ty = binding_type(db, def);
        let Some(ClassLiteral::Static(static_lit)) = ty.as_class_literal() else {
            continue;
        };

        if !static_lit.is_django_model(db) {
            continue;
        }

        let model_name = static_lit.name(db).clone();
        let class_stmt = class_node.node(&module);

        for stmt in &class_stmt.body {
            let (field_name, call_expr) = match stmt {
                ast::Stmt::Assign(assign) => {
                    if assign.targets.len() != 1 {
                        continue;
                    }
                    let name = match &assign.targets[0] {
                        ast::Expr::Name(n) => n.id.clone(),
                        _ => continue,
                    };
                    let ast::Expr::Call(call) = assign.value.as_ref() else {
                        continue;
                    };
                    (name, call)
                }
                ast::Stmt::AnnAssign(ann) => {
                    let name = match ann.target.as_ref() {
                        ast::Expr::Name(n) => n.id.clone(),
                        _ => continue,
                    };
                    let Some(val) = ann.value.as_deref() else {
                        continue;
                    };
                    let ast::Expr::Call(call) = val else {
                        continue;
                    };
                    (name, call)
                }
                _ => continue,
            };

            let field_class_name = match call_expr.func.as_ref() {
                ast::Expr::Name(n) => n.id.to_string(),
                ast::Expr::Attribute(a) => a.attr.to_string(),
                _ => continue,
            };

            let relation_kind = match field_class_name.as_str() {
                "ForeignKey" | "ForeignObject" => DjangoFieldKind::ForeignKey,
                "OneToOneField" => DjangoFieldKind::OneToOne,
                "ManyToManyField" => DjangoFieldKind::ManyToMany,
                _ => continue,
            };

            let to_kwarg = call_expr.arguments.keywords.iter().find_map(|kw| {
                if kw.arg.as_deref() == Some("to") {
                    Some(&kw.value)
                } else {
                    None
                }
            });
            let first_pos = call_expr.arguments.args.first();
            let target_expr = to_kwarg.or(first_pos);

            let target_name = match target_expr {
                Some(ast::Expr::Name(n)) => Name::new(n.id.as_str()),
                Some(ast::Expr::StringLiteral(s)) => {
                    let val = s.value.to_str();
                    if val == "self" {
                        model_name.clone()
                    } else if val.contains('.') {
                        Name::new(val.rsplit('.').next().unwrap_or(val))
                    } else {
                        Name::new(val)
                    }
                }
                // Treat settings.AUTH_USER_MODEL as "User"
                Some(ast::Expr::Attribute(attr)) if attr.attr.as_str() == "AUTH_USER_MODEL" => {
                    Name::new_static("User")
                }
                _ => continue,
            };

            let mut related_name = None;
            let mut nullable = false;
            for kw in &call_expr.arguments.keywords {
                let Some(arg) = kw.arg.as_deref() else {
                    continue;
                };
                match arg {
                    "related_name" => {
                        if let ast::Expr::StringLiteral(s) = &kw.value {
                            let val = s.value.to_str();
                            if val != "+" {
                                related_name = Some(val.to_string());
                            }
                        }
                    }
                    "null" => {
                        if matches!(
                            &kw.value,
                            ast::Expr::BooleanLiteral(ast::ExprBooleanLiteral { value: true, .. })
                        ) {
                            nullable = true;
                        }
                    }
                    _ => {}
                }
            }

            decls.push(RelationDecl {
                source_model_name: model_name.clone(),
                source_file: file,
                field_name: Name::new(field_name.as_str()),
                kind: relation_kind,
                related_name,
                nullable,
                target_model_name: target_name,
            });
        }
    }

    decls.into_boxed_slice()
}

/// Aggregates `django_relations_in_file` across all first-party modules into
/// a lookup table keyed by target model name.
#[salsa::tracked(returns(ref), cycle_initial = |_, _| FxHashMap::default(), heap_size = ruff_memory_usage::heap_size)]
fn django_reverse_relation_index<'db>(
    db: &'db dyn Db,
) -> FxHashMap<Name, Vec<ReverseRelationEntry>> {
    let mut index: FxHashMap<Name, Vec<ReverseRelationEntry>> = FxHashMap::default();

    for module in ty_module_resolver::all_modules(db) {
        let Some(file) = module.file(db) else {
            continue;
        };
        if !db.analysis_settings(file).django {
            continue;
        }
        let is_first_party = module
            .search_path(db)
            .is_some_and(ty_module_resolver::SearchPath::is_first_party);
        if !is_first_party {
            continue;
        }

        for decl in django_relations_in_file(db, file) {
            index
                .entry(decl.target_model_name.clone())
                .or_default()
                .push(ReverseRelationEntry {
                    source_model_name: decl.source_model_name.clone(),
                    source_file: decl.source_file,
                    field_name: decl.field_name.clone(),
                    kind: decl.kind.clone(),
                    related_name: decl.related_name.clone(),
                    nullable: decl.nullable,
                });
        }
    }

    index
}

/// Look up a model class by name in `file`'s global scope and return its instance type.
fn resolve_model_instance<'db>(db: &'db dyn Db, file: File, model_name: &str) -> Type<'db> {
    let scope = global_scope(db, file);
    class_member(db, scope, model_name)
        .ignore_possibly_undefined()
        .and_then(|ty| ty.to_instance(db))
        .unwrap_or_else(Type::unknown)
}

/// Try to match `name` against known reverse relations for this model.
fn lookup_reverse_relation<'db>(
    db: &'db dyn Db,
    class: StaticClassLiteral<'db>,
    name: &str,
) -> Option<Type<'db>> {
    let class_name = class.name(db).clone();
    let index = django_reverse_relation_index(db);
    let entries = index.get(&class_name)?;

    for entry in entries {
        let accessor = match &entry.related_name {
            Some(rn) => rn.clone(),
            None => match entry.kind {
                DjangoFieldKind::OneToOne => entry.source_model_name.to_lowercase(),
                DjangoFieldKind::ForeignKey | DjangoFieldKind::ManyToMany => {
                    format!("{}_set", entry.source_model_name.to_lowercase())
                }
                _ => continue,
            },
        };
        if accessor != name {
            continue;
        }

        let source_ty =
            resolve_model_instance(db, entry.source_file, entry.source_model_name.as_str());

        return Some(match entry.kind {
            DjangoFieldKind::OneToOne => {
                if entry.nullable {
                    UnionType::from_two_elements(db, source_ty, Type::none(db))
                } else {
                    source_ty
                }
            }
            DjangoFieldKind::ForeignKey | DjangoFieldKind::ManyToMany => {
                resolve_specialized_manager(db, source_ty)
            }
            _ => continue,
        });
    }

    None
}

/// Collect all fields for a model, including those inherited from ancestor models.
/// Walks the MRO base-first so child definitions override parent ones.
fn collect_all_django_fields<'db>(
    db: &'db dyn Db,
    class: StaticClassLiteral<'db>,
) -> Vec<DjangoFieldInfo<'db>> {
    let mut map: FxIndexMap<Name, DjangoFieldInfo<'db>> = FxIndexMap::default();

    for base in class.iter_mro(db, None).rev() {
        let Some(class_type) = base.into_class() else {
            continue;
        };
        let Some((base_lit, _)) = class_type.static_class_literal(db) else {
            continue;
        };
        // Skip the Model base class itself
        if base_lit.is_known(db, KnownClass::DjangoModel) {
            continue;
        }
        if !base_lit.is_django_model(db) {
            continue;
        }
        for field in base_lit.django_model_fields(db) {
            map.insert(field.name.clone(), field.clone());
        }
    }

    map.into_values().collect()
}

/// Determine the PK type: explicit `primary_key=True` field, then `AutoField`, then `int`.
fn resolve_pk_type<'db>(db: &'db dyn Db, fields: &[DjangoFieldInfo<'db>]) -> Type<'db> {
    fields
        .iter()
        .find(|f| f.primary_key)
        .or_else(|| {
            fields
                .iter()
                .find(|f| matches!(f.kind, DjangoFieldKind::Auto))
        })
        .map(|f| f.get_type(db))
        .unwrap_or_else(|| KnownClass::Int.to_instance(db))
}

/// AST-reading methods that must be `#[salsa::tracked]` for incrementality.
#[salsa::tracked]
impl<'db> StaticClassLiteral<'db> {
    /// Returns field metadata for all Django field assignments in this class body.
    #[salsa::tracked(returns(deref), cycle_initial = |_, _, _| Box::default(), heap_size = ruff_memory_usage::heap_size)]
    pub(super) fn django_model_fields(self, db: &'db dyn Db) -> Box<[DjangoFieldInfo<'db>]> {
        let file = self.file(db);
        if !db.analysis_settings(file).django {
            return Box::default();
        }
        let module = parsed_module(db, file).load(db);
        let class_stmt = {
            let scope = self.body_scope(db);
            scope.node(db).expect_class().node(&module)
        };

        let mut fields = Vec::new();

        for stmt in &class_stmt.body {
            let (target_name, call_expr) = match stmt {
                ast::Stmt::Assign(assign) => {
                    if assign.targets.len() != 1 {
                        continue;
                    }
                    let target_name = match &assign.targets[0] {
                        ast::Expr::Name(name) => name.id.clone(),
                        _ => continue,
                    };
                    let ast::Expr::Call(call_expr) = assign.value.as_ref() else {
                        continue;
                    };
                    (target_name, call_expr)
                }
                ast::Stmt::AnnAssign(ann_assign) => {
                    let target_name = match ann_assign.target.as_ref() {
                        ast::Expr::Name(name) => name.id.clone(),
                        _ => continue,
                    };
                    let Some(value) = ann_assign.value.as_deref() else {
                        continue;
                    };
                    let ast::Expr::Call(call_expr) = value else {
                        continue;
                    };
                    (target_name, call_expr)
                }
                _ => continue,
            };

            let field_class_name = match call_expr.func.as_ref() {
                ast::Expr::Name(name) => name.id.to_string(),
                ast::Expr::Attribute(attr) => attr.attr.to_string(),
                _ => continue,
            };

            let mut kind = field_name_to_kind(&field_class_name);
            // For unknown field class names, try to resolve through MRO
            if matches!(kind, DjangoFieldKind::Unknown) {
                kind = resolve_custom_field_kind(db, file, &field_class_name);
                if matches!(kind, DjangoFieldKind::Unknown) {
                    continue;
                }
            }

            let mut nullable = false;
            let mut primary_key = false;
            for keyword in &call_expr.arguments.keywords {
                let Some(arg_name) = keyword.arg.as_deref() else {
                    continue;
                };
                match arg_name {
                    "null" => {
                        if matches!(
                            &keyword.value,
                            ast::Expr::BooleanLiteral(ast::ExprBooleanLiteral { value: true, .. })
                        ) {
                            nullable = true;
                        }
                    }
                    "primary_key" => {
                        if matches!(
                            &keyword.value,
                            ast::Expr::BooleanLiteral(ast::ExprBooleanLiteral { value: true, .. })
                        ) {
                            primary_key = true;
                        }
                    }
                    _ => {}
                }
            }

            if field_class_name == "NullBooleanField" {
                nullable = true;
            }

            let related_model = if matches!(
                kind,
                DjangoFieldKind::ForeignKey
                    | DjangoFieldKind::OneToOne
                    | DjangoFieldKind::ManyToMany
            ) {
                let to_kwarg = call_expr.arguments.keywords.iter().find_map(|kw| {
                    if kw.arg.as_deref() == Some("to") {
                        Some(&kw.value)
                    } else {
                        None
                    }
                });

                let first_positional = call_expr.arguments.args.first();

                let target_expr = to_kwarg.or(first_positional);

                let resolved = match target_expr {
                    Some(ast::Expr::Name(name_expr)) => {
                        let module_scope = global_scope(db, file);
                        let member = class_member(db, module_scope, name_expr.id.as_str());
                        member
                            .ignore_possibly_undefined()
                            .and_then(|ty| ty.to_instance(db))
                            .unwrap_or_else(Type::unknown)
                    }
                    Some(ast::Expr::StringLiteral(string_lit)) => {
                        let value = string_lit.value.to_str();
                        if value == "self" {
                            Type::instance(db, self.apply_optional_specialization(db, None))
                        } else if !value.contains('.') {
                            let module_scope = global_scope(db, file);
                            let member = class_member(db, module_scope, value);
                            member
                                .ignore_possibly_undefined()
                                .and_then(|ty| ty.to_instance(db))
                                .unwrap_or_else(Type::unknown)
                        } else {
                            Type::unknown()
                        }
                    }
                    // settings.AUTH_USER_MODEL → resolve to django.contrib.auth User
                    Some(ast::Expr::Attribute(attr)) if attr.attr.as_str() == "AUTH_USER_MODEL" => {
                        let auth_mod = ModuleName::new("django.contrib.auth.models");
                        auth_mod
                            .and_then(|mn| resolve_module_confident(db, &mn))
                            .and_then(|m| m.file(db))
                            .map(|f| {
                                let scope = global_scope(db, f);
                                class_member(db, scope, "User")
                                    .ignore_possibly_undefined()
                                    .and_then(|ty| ty.to_instance(db))
                                    .unwrap_or_else(Type::unknown)
                            })
                            .unwrap_or_else(Type::unknown)
                    }
                    _ => Type::unknown(),
                };
                Some(resolved)
            } else {
                None
            };

            fields.push(DjangoFieldInfo {
                name: Name::new(target_name.as_str()),
                kind,
                nullable,
                primary_key,
                related_model,
            });
        }

        fields.into_boxed_slice()
    }

    /// Detect if the model is abstract (has a `Meta` inner class with `abstract = True`).
    #[salsa::tracked]
    pub(super) fn is_django_model_abstract(self, db: &'db dyn Db) -> bool {
        let file = self.file(db);
        let module = parsed_module(db, file).load(db);
        let class_stmt = {
            let scope = self.body_scope(db);
            scope.node(db).expect_class().node(&module)
        };

        for stmt in &class_stmt.body {
            let ast::Stmt::ClassDef(meta_class) = stmt else {
                continue;
            };
            if meta_class.name.as_str() != "Meta" {
                continue;
            }
            for meta_stmt in &meta_class.body {
                if let ast::Stmt::Assign(assign) = meta_stmt
                    && assign.targets.len() == 1
                    && let ast::Expr::Name(name) = &assign.targets[0]
                    && name.id.as_str() == "abstract"
                    && let ast::Expr::BooleanLiteral(b) = assign.value.as_ref()
                {
                    return b.value;
                }
            }
            break;
        }
        false
    }
}

/// Synthesize an instance member for a Django model (field access, pk, _id, reverse relations).
pub(super) fn synthesize_django_instance_member<'db>(
    db: &'db dyn Db,
    class: StaticClassLiteral<'db>,
    name: &str,
) -> Option<Type<'db>> {
    if !class.is_django_model(db) {
        return None;
    }

    let fields = class.django_model_fields(db);

    match name {
        "pk" | "id" => {
            let all_fields = collect_all_django_fields(db, class);
            if name == "pk" {
                return Some(resolve_pk_type(db, &all_fields));
            }
            if !all_fields.iter().any(|f| f.name.as_str() == "id") {
                return Some(KnownClass::Int.to_instance(db));
            }
        }
        _ => {}
    }

    if let Some(base_name) = name.strip_suffix("_id") {
        let all_fields = collect_all_django_fields(db, class);
        if let Some(fk_field) = all_fields.iter().find(|f| {
            f.name.as_str() == base_name
                && matches!(
                    f.kind,
                    DjangoFieldKind::ForeignKey | DjangoFieldKind::OneToOne
                )
        }) {
            let pk_ty = fk_field
                .related_model
                .map(|rm| resolve_related_pk_type(db, rm))
                .unwrap_or_else(|| KnownClass::Int.to_instance(db));
            return Some(if fk_field.nullable {
                UnionType::from_two_elements(db, pk_ty, Type::none(db))
            } else {
                pk_ty
            });
        }
    }

    if let Some(field) = fields.iter().find(|f| f.name.as_str() == name) {
        return Some(field.get_type(db));
    }

    lookup_reverse_relation(db, class, name)
}

/// Synthesize a class-level model member (objects, exceptions, __init__, fields, reverse relations).
pub(super) fn synthesize_django_model_member<'db>(
    db: &'db dyn Db,
    class: StaticClassLiteral<'db>,
    name: &str,
) -> Option<Type<'db>> {
    if !class.is_django_model(db) {
        return None;
    }

    // Abstract models do not have an `objects` manager or exception classes,
    // but they still expose field access and `__init__` synthesis.
    if class.is_django_model_abstract(db)
        && matches!(name, "objects" | "DoesNotExist" | "MultipleObjectsReturned")
    {
        return None;
    }

    let fields = class.django_model_fields(db);

    match name {
        "objects" => {
            let model_ty = Type::instance(db, class.apply_optional_specialization(db, None));
            Some(resolve_specialized_manager(db, model_ty))
        }
        "DoesNotExist" => Some(resolve_django_exception(db, "ObjectDoesNotExist")),
        "MultipleObjectsReturned" => Some(resolve_django_exception(db, "MultipleObjectsReturned")),
        "pk" => {
            let all_fields = collect_all_django_fields(db, class);
            Some(resolve_pk_type(db, &all_fields))
        }
        "id" => {
            let all_fields = collect_all_django_fields(db, class);
            if all_fields.iter().any(|f| f.name.as_str() == "id") {
                None
            } else {
                Some(KnownClass::Int.to_instance(db))
            }
        }
        "__init__" => Some(synthesize_model_init(db, class)),
        name if name.ends_with("_id") => {
            let base_name = &name[..name.len() - 3];
            let all_fields = collect_all_django_fields(db, class);
            let fk_field = all_fields.iter().find(|f| {
                f.name.as_str() == base_name
                    && matches!(
                        f.kind,
                        DjangoFieldKind::ForeignKey | DjangoFieldKind::OneToOne
                    )
            })?;
            let pk_ty = fk_field
                .related_model
                .map(|rm| resolve_related_pk_type(db, rm))
                .unwrap_or_else(|| KnownClass::Int.to_instance(db));
            if fk_field.nullable {
                Some(UnionType::from_two_elements(db, pk_ty, Type::none(db)))
            } else {
                Some(pk_ty)
            }
        }
        _ => {
            if let Some(field) = fields.iter().find(|f| f.name.as_str() == name) {
                return Some(field.get_type(db));
            }
            lookup_reverse_relation(db, class, name)
        }
    }
}

/// Build the `__init__` signature. All non-M2M fields become optional keyword-only params.
fn synthesize_model_init<'db>(db: &'db dyn Db, class: StaticClassLiteral<'db>) -> Type<'db> {
    let instance_ty = Type::instance(db, class.apply_optional_specialization(db, None));
    let all_fields = collect_all_django_fields(db, class);

    let mut parameters = vec![
        Parameter::positional_or_keyword(Name::new_static("self")).with_annotated_type(instance_ty),
    ];

    let has_explicit_pk = all_fields
        .iter()
        .any(|f| f.primary_key || f.name.as_str() == "id");
    if !has_explicit_pk {
        let int_ty = KnownClass::Int.to_instance(db);
        parameters.push(
            Parameter::keyword_only(Name::new_static("id"))
                .with_annotated_type(int_ty)
                .with_default_type(Type::none(db)),
        );
    }

    for field in &all_fields {
        if matches!(field.kind, DjangoFieldKind::ManyToMany) {
            continue;
        }
        let field_ty = field.get_type(db);
        let param = Parameter::keyword_only(field.name.clone())
            .with_annotated_type(field_ty)
            .with_default_type(Type::none(db));
        parameters.push(param);
    }

    let signature = Signature::new(Parameters::new(db, parameters), Type::none(db));
    Type::function_like_callable(db, signature)
}
