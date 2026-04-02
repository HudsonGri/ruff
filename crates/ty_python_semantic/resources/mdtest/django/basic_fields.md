# Django Model Basic Field Types

```toml
[analysis]
django = true

[environment]
python-version = "3.11"
python = "/.venv"
```

## Field access returns Python type (not descriptor)

Accessing a field on a model instance should return the Python value type (via `__get__`), not the
descriptor object itself.

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
from django.db.models import Model, CharField, IntegerField, FloatField, BooleanField

class Article(Model):
    title = CharField(max_length=100)
    view_count = IntegerField()
    rating = FloatField()
    published = BooleanField()

a = Article()
reveal_type(a.title)  # revealed: str
reveal_type(a.view_count)  # revealed: int
reveal_type(a.rating)  # revealed: float
reveal_type(a.published)  # revealed: bool
```

## DateField, DateTimeField, and TimeField resolve to stdlib datetime types

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.fields import DateField, DateTimeField, TimeField
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
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

class DateField(Field):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class DateTimeField(Field):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class TimeField(Field):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...
```

```py
from django.db.models import Model, DateField, DateTimeField, TimeField

class Event(Model):
    start_date = DateField()
    start_datetime = DateTimeField()
    start_time = TimeField()

e = Event()
reveal_type(e.start_date)  # revealed: date
reveal_type(e.start_datetime)  # revealed: datetime
reveal_type(e.start_time)  # revealed: time
```

## DecimalField resolves to decimal.Decimal

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.fields import DecimalField
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
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

class DecimalField(Field):
    def __init__(
        self, *, max_digits: int = 10, decimal_places: int = 2, null: bool = False, blank: bool = False, default=None
    ): ...
```

```py
from django.db.models import Model, DecimalField

class Invoice(Model):
    amount = DecimalField(max_digits=10, decimal_places=2)

i = Invoice()
reveal_type(i.amount)  # revealed: Decimal
```

## UUIDField resolves to uuid.UUID

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.fields import UUIDField
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
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

class UUIDField(Field):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...
```

```py
from django.db.models import Model, UUIDField

class Session(Model):
    session_id = UUIDField()

s = Session()
reveal_type(s.session_id)  # revealed: UUID
```

## Nullable variants of stdlib type fields resolve to T | None

When `null=True` is passed, each stdlib-typed field returns its type unioned with `None`.

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.fields import DateField, DateTimeField, TimeField, DecimalField, UUIDField
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
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

class DateField(Field):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class DateTimeField(Field):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class TimeField(Field):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...

class DecimalField(Field):
    def __init__(
        self, *, max_digits: int = 10, decimal_places: int = 2, null: bool = False, blank: bool = False, default=None
    ): ...

class UUIDField(Field):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...
```

```py
from django.db.models import Model, DateField, DateTimeField, TimeField, DecimalField, UUIDField

class Record(Model):
    due_date = DateField(null=True)
    end_datetime = DateTimeField(null=True)
    end_time = TimeField(null=True)
    amount = DecimalField(max_digits=10, decimal_places=2, null=True)
    token = UUIDField(null=True)

r = Record()
reveal_type(r.due_date)  # revealed: date | None
reveal_type(r.end_datetime)  # revealed: datetime | None
reveal_type(r.end_time)  # revealed: time | None
reveal_type(r.amount)  # revealed: Decimal | None
reveal_type(r.token)  # revealed: UUID | None
```

## BinaryField resolves to bytes

`BinaryField` stores raw binary data and returns `bytes` on instance access.

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.fields import BinaryField
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
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

class BinaryField(Field[bytes, bytes]):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...
```

```py
from django.db.models import Model, BinaryField

class Blob(Model):
    data = BinaryField()
    optional_data = BinaryField(null=True)

b = Blob()
reveal_type(b.data)  # revealed: bytes
reveal_type(b.optional_data)  # revealed: bytes | None
```

## FileField and ImageField fall back to Any

`FileField` and `ImageField` return `FieldFile` / `ImageFieldFile` descriptor objects at runtime,
not plain strings. Until those types are modeled, ty falls back to `Any`.

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.fields import FileField, ImageField
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
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

class FileField(Field):
    def __init__(self, *, upload_to: str = "", null: bool = False, blank: bool = False, default=None): ...

class ImageField(Field):
    def __init__(self, *, upload_to: str = "", null: bool = False, blank: bool = False, default=None): ...
```

```py
from django.db.models import Model, FileField, ImageField

class Media(Model):
    upload = FileField()
    photo = ImageField()

m = Media()
reveal_type(m.upload)  # revealed: Any
reveal_type(m.photo)  # revealed: Any
```

## Unrecognized field classes fall back to Unknown

Fields whose class is not recognized by ty's Django support should produce `Unknown` rather than
raising an error.

`/.venv/<path-to-site-packages>/django/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/__init__.py`:

```py
```

`/.venv/<path-to-site-packages>/django/db/models/__init__.py`:

```py
from django.db.models.base import Model
from django.db.models.fields import UnknownCustomField
```

`/.venv/<path-to-site-packages>/django/db/models/base.py`:

```py
class Model:
    pass
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

class UnknownCustomField(Field):
    def __init__(self, *, null: bool = False, blank: bool = False, default=None): ...
```

```py
from django.db.models import Model, UnknownCustomField

class Fallback(Model):
    data = UnknownCustomField()
    nullable_data = UnknownCustomField(null=True)

f = Fallback()
reveal_type(f.data)  # revealed: Unknown
reveal_type(f.nullable_data)  # revealed: Unknown
```
