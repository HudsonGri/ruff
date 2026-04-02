# Django ForeignKey Field Types

```toml
[analysis]
django = true

[environment]
python-version = "3.11"
python = "/.venv"
```

## ForeignKey on instance returns related model instance

Accessing a ForeignKey field on a model instance should return an instance of the related model, not
a `ForeignKey` descriptor.

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
from django.db.models import Model, CharField, ForeignKey

class Author(Model):
    name = CharField(max_length=100)

class Book(Model):
    author = ForeignKey(Author, on_delete=None)

b = Book()
reveal_type(b.author)  # revealed: Author
reveal_type(b.author.name)  # revealed: str
```

## Nullable ForeignKey resolves to related model | None

When `null=True` is passed to `ForeignKey`, the instance access type becomes `RelatedModel | None`.

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
from django.db.models import Model, CharField, ForeignKey

class Tag(Model):
    label = CharField(max_length=50)

class Article(Model):
    primary_tag = ForeignKey(Tag, null=True, on_delete=None)

a = Article()
reveal_type(a.primary_tag)  # revealed: Tag | None
```
