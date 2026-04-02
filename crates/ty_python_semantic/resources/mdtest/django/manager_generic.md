# Django Manager[Self] Specialization

```toml
[analysis]
django = true

[environment]
python-version = "3.11"
python = "/.venv"
```

## objects manager is specialized to Manager[ModelClass]

When accessing `Model.objects`, the synthesized manager should be `Manager[ModelClass]`, so that
calls like `.get()` return the concrete model type and `.all()` returns `QuerySet[ModelClass]`.

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

class Article(Model):
    title = CharField(max_length=100)

reveal_type(Article.objects)  # revealed: Manager[Article]
```

## objects.get() returns the concrete model type

Calling `.get()` on the synthesized manager should return the concrete model, not a bare TypeVar.

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

class Article(Model):
    title = CharField(max_length=100)

result = Article.objects.get(title="hello")
reveal_type(result)  # revealed: Article
```

## objects.all() returns QuerySet specialized to model

Calling `.all()` on the synthesized manager should return `QuerySet[ModelClass]`.

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

class Article(Model):
    title = CharField(max_length=100)

qs = Article.objects.all()
reveal_type(qs)  # revealed: QuerySet[Article]
```

## objects.filter() returns QuerySet specialized to model

Calling `.filter()` on the synthesized manager should return `QuerySet[ModelClass]`.

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
from django.db.models import Model, IntegerField

class Order(Model):
    amount = IntegerField()

qs = Order.objects.filter(amount=10)
reveal_type(qs)  # revealed: QuerySet[Order]
```
