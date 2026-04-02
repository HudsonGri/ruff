# Django Synthesized Model Attributes

```toml
[analysis]
django = true

[environment]
python-version = "3.11"
python = "/.venv"
```

## objects manager is available on model class

Django synthesizes an `objects` class attribute of type `Manager` on every model. Calling `.all()`
on it returns a `QuerySet`.

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.manager import Manager, QuerySet
from django.db.models.fields import CharField, TextField, IntegerField, FloatField, BooleanField, AutoField
from django.db.models.fields.related import ForeignKey
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/manager.py`:

```py
from typing import Generic, TypeVar

_ModelT = TypeVar("_ModelT")

class Manager(Generic[_ModelT]):
    def all(self) -> "QuerySet[_ModelT]": ...
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def get(self, **kwargs) -> _ModelT: ...
    def create(self, **kwargs) -> _ModelT: ...

class QuerySet(Generic[_ModelT]):
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def first(self) -> _ModelT | None: ...
    def __iter__(self): ...
```

`/.venv/<path-to-site-packages>/django/db/models/fields/__init__.py`:

```py
from typing import Generic, TypeVar, overload

_ST = TypeVar("_ST")
_GT = TypeVar("_GT")

class Field(Generic[_ST, _GT]):
    @overload
    def __get__(self, instance: None, owner: type) -> "Field[_ST, _GT]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _GT: ...
    def __get__(self, instance, owner): ...
    def __set__(self, instance: object, value: _ST) -> None: ...

class CharField(Field[str, str]):
    def __init__(self, *, max_length: int = 255, null: bool = False, blank: bool = False, default=None): ...

class TextField(Field[str, str]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class IntegerField(Field[int, int]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class FloatField(Field[float, float]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class BooleanField(Field[bool, bool]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class AutoField(Field[int, int]):
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/fields/related.py`:

```py
from typing import Generic, TypeVar, overload

_To = TypeVar("_To")

class ForeignKey(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ForeignKey[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _To: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, on_delete, null: bool = False, related_name: str = "", db_column: str = ""): ...
```

```py
from django.db.models import Model, CharField

class Product(Model):
    name = CharField(max_length=50)

reveal_type(Product.objects)  # revealed: Manager[Product]
products = Product.objects.all()
reveal_type(products)  # revealed: QuerySet[Product]
```

## DoesNotExist exception class is available

Django synthesizes a `DoesNotExist` exception class as a nested class on every model. It should be
usable in `except` clauses without error.

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.manager import Manager, QuerySet
from django.db.models.fields import CharField, TextField, IntegerField, FloatField, BooleanField, AutoField
from django.db.models.fields.related import ForeignKey
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/manager.py`:

```py
from typing import Generic, TypeVar

_ModelT = TypeVar("_ModelT")

class Manager(Generic[_ModelT]):
    def all(self) -> "QuerySet[_ModelT]": ...
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def get(self, **kwargs) -> _ModelT: ...
    def create(self, **kwargs) -> _ModelT: ...

class QuerySet(Generic[_ModelT]):
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def first(self) -> _ModelT | None: ...
    def __iter__(self): ...
```

`/.venv/<path-to-site-packages>/django/db/models/fields/__init__.py`:

```py
from typing import Generic, TypeVar, overload

_ST = TypeVar("_ST")
_GT = TypeVar("_GT")

class Field(Generic[_ST, _GT]):
    @overload
    def __get__(self, instance: None, owner: type) -> "Field[_ST, _GT]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _GT: ...
    def __get__(self, instance, owner): ...
    def __set__(self, instance: object, value: _ST) -> None: ...

class CharField(Field[str, str]):
    def __init__(self, *, max_length: int = 255, null: bool = False, blank: bool = False, default=None): ...

class TextField(Field[str, str]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class IntegerField(Field[int, int]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class FloatField(Field[float, float]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class BooleanField(Field[bool, bool]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class AutoField(Field[int, int]):
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/fields/related.py`:

```py
from typing import Generic, TypeVar, overload

_To = TypeVar("_To")

class ForeignKey(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ForeignKey[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _To: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, on_delete, null: bool = False, related_name: str = "", db_column: str = ""): ...
```

```py
from django.db.models import Model, CharField

class Product(Model):
    name = CharField(max_length=50)

try:
    p = Product.objects.get(name="foo")
except Product.DoesNotExist:
    pass
```

## pk and id are available as int

Django synthesizes `pk` and `id` attributes of type `int` on every model (corresponding to the
implicit auto-generated primary key).

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.manager import Manager, QuerySet
from django.db.models.fields import CharField, TextField, IntegerField, FloatField, BooleanField, AutoField
from django.db.models.fields.related import ForeignKey
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/manager.py`:

```py
from typing import Generic, TypeVar

_ModelT = TypeVar("_ModelT")

class Manager(Generic[_ModelT]):
    def all(self) -> "QuerySet[_ModelT]": ...
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def get(self, **kwargs) -> _ModelT: ...
    def create(self, **kwargs) -> _ModelT: ...

class QuerySet(Generic[_ModelT]):
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def first(self) -> _ModelT | None: ...
    def __iter__(self): ...
```

`/.venv/<path-to-site-packages>/django/db/models/fields/__init__.py`:

```py
from typing import Generic, TypeVar, overload

_ST = TypeVar("_ST")
_GT = TypeVar("_GT")

class Field(Generic[_ST, _GT]):
    @overload
    def __get__(self, instance: None, owner: type) -> "Field[_ST, _GT]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _GT: ...
    def __get__(self, instance, owner): ...
    def __set__(self, instance: object, value: _ST) -> None: ...

class CharField(Field[str, str]):
    def __init__(self, *, max_length: int = 255, null: bool = False, blank: bool = False, default=None): ...

class TextField(Field[str, str]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class IntegerField(Field[int, int]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class FloatField(Field[float, float]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class BooleanField(Field[bool, bool]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class AutoField(Field[int, int]):
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/fields/related.py`:

```py
from typing import Generic, TypeVar, overload

_To = TypeVar("_To")

class ForeignKey(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ForeignKey[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _To: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, on_delete, null: bool = False, related_name: str = "", db_column: str = ""): ...
```

```py
from django.db.models import Model, CharField

class Product(Model):
    name = CharField(max_length=50)

p = Product()
reveal_type(p.pk)  # revealed: int
reveal_type(p.id)  # revealed: int
```

## UUIDField(primary_key=True) makes pk return uuid.UUID

When a model explicitly sets a `UUIDField` as the primary key, `pk` should reflect that type.

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.manager import Manager, QuerySet
from django.db.models.fields import CharField, TextField, IntegerField, FloatField, BooleanField, AutoField, UUIDField
from django.db.models.fields.related import ForeignKey
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/manager.py`:

```py
from typing import Generic, TypeVar

_ModelT = TypeVar("_ModelT")

class Manager(Generic[_ModelT]):
    def all(self) -> "QuerySet[_ModelT]": ...
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def get(self, **kwargs) -> _ModelT: ...
    def create(self, **kwargs) -> _ModelT: ...

class QuerySet(Generic[_ModelT]):
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def first(self) -> _ModelT | None: ...
    def __iter__(self): ...
```

`/.venv/<path-to-site-packages>/django/db/models/fields/__init__.py`:

```py
import uuid
from typing import Generic, TypeVar, overload

_ST = TypeVar("_ST")
_GT = TypeVar("_GT")

class Field(Generic[_ST, _GT]):
    @overload
    def __get__(self, instance: None, owner: type) -> "Field[_ST, _GT]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _GT: ...
    def __get__(self, instance, owner): ...
    def __set__(self, instance: object, value: _ST) -> None: ...

class CharField(Field[str, str]):
    def __init__(
        self, *, max_length: int = 255, null: bool = False, blank: bool = False, default=None, primary_key: bool = False
    ): ...

class TextField(Field[str, str]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class IntegerField(Field[int, int]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class FloatField(Field[float, float]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class BooleanField(Field[bool, bool]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class AutoField(Field[int, int]):
    pass

class UUIDField(Field[uuid.UUID, uuid.UUID]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None, primary_key: bool = False): ...
```

`/.venv/<path-to-site-packages>/django/db/models/fields/related.py`:

```py
from typing import Generic, TypeVar, overload

_To = TypeVar("_To")

class ForeignKey(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ForeignKey[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _To: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, on_delete, null: bool = False, related_name: str = "", db_column: str = ""): ...
```

```py
from uuid import UUID
from django.db.models import Model, UUIDField

class Document(Model):
    id = UUIDField(primary_key=True)

d = Document()
reveal_type(d.pk)  # revealed: UUID
reveal_type(d.id)  # revealed: UUID
```

## CharField(primary_key=True) makes pk return str

When a model uses a `CharField` as the primary key, `pk` should return `str`.

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.manager import Manager, QuerySet
from django.db.models.fields import CharField, TextField, IntegerField, FloatField, BooleanField, AutoField
from django.db.models.fields.related import ForeignKey
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/manager.py`:

```py
from typing import Generic, TypeVar

_ModelT = TypeVar("_ModelT")

class Manager(Generic[_ModelT]):
    def all(self) -> "QuerySet[_ModelT]": ...
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def get(self, **kwargs) -> _ModelT: ...
    def create(self, **kwargs) -> _ModelT: ...

class QuerySet(Generic[_ModelT]):
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def first(self) -> _ModelT | None: ...
    def __iter__(self): ...
```

`/.venv/<path-to-site-packages>/django/db/models/fields/__init__.py`:

```py
from typing import Generic, TypeVar, overload

_ST = TypeVar("_ST")
_GT = TypeVar("_GT")

class Field(Generic[_ST, _GT]):
    @overload
    def __get__(self, instance: None, owner: type) -> "Field[_ST, _GT]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _GT: ...
    def __get__(self, instance, owner): ...
    def __set__(self, instance: object, value: _ST) -> None: ...

class CharField(Field[str, str]):
    def __init__(
        self, *, max_length: int = 255, null: bool = False, blank: bool = False, default=None, primary_key: bool = False
    ): ...

class TextField(Field[str, str]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class IntegerField(Field[int, int]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class FloatField(Field[float, float]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class BooleanField(Field[bool, bool]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class AutoField(Field[int, int]):
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/fields/related.py`:

```py
from typing import Generic, TypeVar, overload

_To = TypeVar("_To")

class ForeignKey(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ForeignKey[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _To: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, on_delete, null: bool = False, related_name: str = "", db_column: str = ""): ...
```

```py
from django.db.models import Model, CharField

class Country(Model):
    code = CharField(max_length=2, primary_key=True)

c = Country()
reveal_type(c.pk)  # revealed: str
reveal_type(c.code)  # revealed: str
```

## Model with no explicit PK defaults pk to int

When a model does not declare any `primary_key=True` field, Django auto-generates an `AutoField`
named `id`, so both `pk` and `id` should be `int`.

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.manager import Manager, QuerySet
from django.db.models.fields import CharField, TextField, IntegerField, FloatField, BooleanField, AutoField
from django.db.models.fields.related import ForeignKey
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/manager.py`:

```py
from typing import Generic, TypeVar

_ModelT = TypeVar("_ModelT")

class Manager(Generic[_ModelT]):
    def all(self) -> "QuerySet[_ModelT]": ...
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def get(self, **kwargs) -> _ModelT: ...
    def create(self, **kwargs) -> _ModelT: ...

class QuerySet(Generic[_ModelT]):
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def first(self) -> _ModelT | None: ...
    def __iter__(self): ...
```

`/.venv/<path-to-site-packages>/django/db/models/fields/__init__.py`:

```py
from typing import Generic, TypeVar, overload

_ST = TypeVar("_ST")
_GT = TypeVar("_GT")

class Field(Generic[_ST, _GT]):
    @overload
    def __get__(self, instance: None, owner: type) -> "Field[_ST, _GT]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _GT: ...
    def __get__(self, instance, owner): ...
    def __set__(self, instance: object, value: _ST) -> None: ...

class CharField(Field[str, str]):
    def __init__(self, *, max_length: int = 255, null: bool = False, blank: bool = False, default=None): ...

class TextField(Field[str, str]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class IntegerField(Field[int, int]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class FloatField(Field[float, float]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class BooleanField(Field[bool, bool]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class AutoField(Field[int, int]):
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/fields/related.py`:

```py
from typing import Generic, TypeVar, overload

_To = TypeVar("_To")

class ForeignKey(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ForeignKey[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _To: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, on_delete, null: bool = False, related_name: str = "", db_column: str = ""): ...
```

```py
from django.db.models import Model, CharField

class Tag(Model):
    name = CharField(max_length=50)

t = Tag()
reveal_type(t.pk)  # revealed: int
reveal_type(t.id)  # revealed: int
```

## Inherited custom PK propagates pk type to child

When a parent model defines a custom primary key, child models inherit it and `pk` on child
instances should return the same type.

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.manager import Manager, QuerySet
from django.db.models.fields import CharField, TextField, IntegerField, FloatField, BooleanField, AutoField, UUIDField
from django.db.models.fields.related import ForeignKey
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/manager.py`:

```py
from typing import Generic, TypeVar

_ModelT = TypeVar("_ModelT")

class Manager(Generic[_ModelT]):
    def all(self) -> "QuerySet[_ModelT]": ...
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def get(self, **kwargs) -> _ModelT: ...
    def create(self, **kwargs) -> _ModelT: ...

class QuerySet(Generic[_ModelT]):
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def first(self) -> _ModelT | None: ...
    def __iter__(self): ...
```

`/.venv/<path-to-site-packages>/django/db/models/fields/__init__.py`:

```py
import uuid
from typing import Generic, TypeVar, overload

_ST = TypeVar("_ST")
_GT = TypeVar("_GT")

class Field(Generic[_ST, _GT]):
    @overload
    def __get__(self, instance: None, owner: type) -> "Field[_ST, _GT]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _GT: ...
    def __get__(self, instance, owner): ...
    def __set__(self, instance: object, value: _ST) -> None: ...

class CharField(Field[str, str]):
    def __init__(
        self, *, max_length: int = 255, null: bool = False, blank: bool = False, default=None, primary_key: bool = False
    ): ...

class TextField(Field[str, str]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class IntegerField(Field[int, int]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class FloatField(Field[float, float]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class BooleanField(Field[bool, bool]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class AutoField(Field[int, int]):
    pass

class UUIDField(Field[uuid.UUID, uuid.UUID]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None, primary_key: bool = False): ...
```

`/.venv/<path-to-site-packages>/django/db/models/fields/related.py`:

```py
from typing import Generic, TypeVar, overload

_To = TypeVar("_To")

class ForeignKey(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ForeignKey[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _To: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, on_delete, null: bool = False, related_name: str = "", db_column: str = ""): ...
```

```py
from uuid import UUID
from django.db.models import Model, UUIDField, CharField

class BaseEntity(Model):
    id = UUIDField(primary_key=True)

class ChildEntity(BaseEntity):
    name = CharField(max_length=100)

c = ChildEntity()
reveal_type(c.pk)  # revealed: UUID
```

## CharField(primary_key=True) overrides the implicit auto pk

Explicitly setting `primary_key=True` on a `CharField` suppresses the auto-generated integer `id`,
so both `pk` and the named field should be `str`, not `int`.

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.manager import Manager, QuerySet
from django.db.models.fields import CharField, TextField, IntegerField, FloatField, BooleanField, AutoField
from django.db.models.fields.related import ForeignKey
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/manager.py`:

```py
from typing import Generic, TypeVar

_ModelT = TypeVar("_ModelT")

class Manager(Generic[_ModelT]):
    def all(self) -> "QuerySet[_ModelT]": ...
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def get(self, **kwargs) -> _ModelT: ...
    def create(self, **kwargs) -> _ModelT: ...

class QuerySet(Generic[_ModelT]):
    def filter(self, **kwargs) -> "QuerySet[_ModelT]": ...
    def first(self) -> _ModelT | None: ...
    def __iter__(self): ...
```

`/.venv/<path-to-site-packages>/django/db/models/fields/__init__.py`:

```py
from typing import Generic, TypeVar, overload

_ST = TypeVar("_ST")
_GT = TypeVar("_GT")

class Field(Generic[_ST, _GT]):
    @overload
    def __get__(self, instance: None, owner: type) -> "Field[_ST, _GT]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _GT: ...
    def __get__(self, instance, owner): ...
    def __set__(self, instance: object, value: _ST) -> None: ...

class CharField(Field[str, str]):
    def __init__(
        self, *, max_length: int = 255, null: bool = False, blank: bool = False, default=None, primary_key: bool = False
    ): ...

class TextField(Field[str, str]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class IntegerField(Field[int, int]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class FloatField(Field[float, float]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class BooleanField(Field[bool, bool]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class AutoField(Field[int, int]):
    pass
```

`/.venv/<path-to-site-packages>/django/db/models/fields/related.py`:

```py
from typing import Generic, TypeVar, overload

_To = TypeVar("_To")

class ForeignKey(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ForeignKey[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _To: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, on_delete, null: bool = False, related_name: str = "", db_column: str = ""): ...
```

```py
from django.db.models import Model, CharField

class Currency(Model):
    code = CharField(max_length=3, primary_key=True)
    name = CharField(max_length=50)

c = Currency()
reveal_type(c.pk)  # revealed: str
reveal_type(c.code)  # revealed: str
```
