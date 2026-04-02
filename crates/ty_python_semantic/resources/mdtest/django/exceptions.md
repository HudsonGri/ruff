# Django Model Exception Types

```toml
[analysis]
django = true

[environment]
python-version = "3.11"
python = "/.venv"
```

## DoesNotExist is catchable as ObjectDoesNotExist

The synthesized `Model.DoesNotExist` should be a subclass of
`django.core.exceptions.ObjectDoesNotExist`, so catching the base class works across models.

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

`/.venv/<path-to-site-packages>/django/core/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/core/exceptions.py`:

```py
class ObjectDoesNotExist(Exception):
    pass

class MultipleObjectsReturned(Exception):
    pass
```

```py
from django.db.models import Model, CharField
from django.core.exceptions import ObjectDoesNotExist

class Product(Model):
    name = CharField(max_length=50)

try:
    p = Product.objects.get(name="foo")
except ObjectDoesNotExist:
    pass
```

## DoesNotExist is usable in except clause on the model

The per-model `DoesNotExist` class synthesized by Django can be caught directly off the model class.

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

`/.venv/<path-to-site-packages>/django/core/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/core/exceptions.py`:

```py
class ObjectDoesNotExist(Exception):
    pass

class MultipleObjectsReturned(Exception):
    pass
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

## MultipleObjectsReturned is catchable on the model

The per-model `MultipleObjectsReturned` class should be accessible on the model and usable in an
`except` clause without a type error.

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

`/.venv/<path-to-site-packages>/django/core/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/core/exceptions.py`:

```py
class ObjectDoesNotExist(Exception):
    pass

class MultipleObjectsReturned(Exception):
    pass
```

```py
from django.db.models import Model, CharField

class Product(Model):
    name = CharField(max_length=50)

try:
    p = Product.objects.get(name="foo")
except Product.MultipleObjectsReturned:
    pass
```
