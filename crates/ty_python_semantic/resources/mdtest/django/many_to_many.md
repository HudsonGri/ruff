# Django ManyToManyField Types

```toml
[analysis]
django = true

[environment]
python-version = "3.11"
python = "/.venv"
```

## ManyToManyField on instance returns Manager[RelatedModel]

Accessing a ManyToManyField on a model instance should return `Manager[RelatedModel]`, allowing
typed queryset operations against the related model.

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
from django.db.models.fields.related import ForeignKey, ManyToManyField
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
from django.db.models.manager import Manager

_To = TypeVar("_To")

class ForeignKey(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ForeignKey[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _To: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, on_delete, null: bool = False, related_name: str = "", db_column: str = ""): ...

class ManyToManyField(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ManyToManyField[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> Manager[_To]: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, related_name: str = "", blank: bool = False): ...
```

```py
from django.db.models import Model, CharField
from django.db.models.fields.related import ManyToManyField

class Tag(Model):
    label = CharField(max_length=50)

class Article(Model):
    tags = ManyToManyField(Tag)

a = Article()
reveal_type(a.tags)  # revealed: Manager[Tag]
```

## ManyToManyField .all() returns QuerySet[RelatedModel]

Calling `.all()` on a ManyToManyField manager should return `QuerySet[RelatedModel]`.

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
from django.db.models.fields.related import ForeignKey, ManyToManyField
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
from django.db.models.manager import Manager

_To = TypeVar("_To")

class ForeignKey(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ForeignKey[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _To: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, on_delete, null: bool = False, related_name: str = "", db_column: str = ""): ...

class ManyToManyField(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ManyToManyField[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> Manager[_To]: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, related_name: str = "", blank: bool = False): ...
```

```py
from django.db.models import Model, CharField
from django.db.models.fields.related import ManyToManyField

class Tag(Model):
    label = CharField(max_length=50)

class Article(Model):
    tags = ManyToManyField(Tag)

a = Article()
qs = a.tags.all()
reveal_type(qs)  # revealed: QuerySet[Tag]
```

## ManyToManyField class access returns Manager

Accessing a ManyToManyField on the class returns `Manager[RelatedModel]` because ty's synthesis path
intercepts the lookup before the descriptor protocol runs. Both class and instance access go through
`synthesize_django_instance_member`.

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
from django.db.models.fields.related import ForeignKey, ManyToManyField
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
from django.db.models.manager import Manager

_To = TypeVar("_To")

class ForeignKey(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ForeignKey[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> _To: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, on_delete, null: bool = False, related_name: str = "", db_column: str = ""): ...

class ManyToManyField(Generic[_To]):
    @overload
    def __get__(self, instance: None, owner: type) -> "ManyToManyField[_To]": ...
    @overload
    def __get__(self, instance: object, owner: type) -> Manager[_To]: ...
    def __get__(self, instance, owner): ...
    def __init__(self, to: type, *, related_name: str = "", blank: bool = False): ...
```

```py
from django.db.models import Model, CharField
from django.db.models.fields.related import ManyToManyField

class Tag(Model):
    label = CharField(max_length=50)

class Article(Model):
    tags = ManyToManyField(Tag)

reveal_type(Article.tags)  # revealed: Manager[Tag]
```
