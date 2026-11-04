from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Object(_message.Message):
    __slots__ = ("type", "id", "code", "all")
    TYPE_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    CODE_FIELD_NUMBER: _ClassVar[int]
    ALL_FIELD_NUMBER: _ClassVar[int]
    type: str
    id: int
    code: str
    all: _empty_pb2.Empty
    def __init__(self, type: _Optional[str] = ..., id: _Optional[int] = ..., code: _Optional[str] = ..., all: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ...) -> None: ...

class Subject(_message.Message):
    __slots__ = ("user", "role")
    class Role(_message.Message):
        __slots__ = ("root", "administrator", "id", "code")
        class Root(_message.Message):
            __slots__ = ()
            def __init__(self) -> None: ...
        class Administrator(_message.Message):
            __slots__ = ()
            def __init__(self) -> None: ...
        ROOT_FIELD_NUMBER: _ClassVar[int]
        ADMINISTRATOR_FIELD_NUMBER: _ClassVar[int]
        ID_FIELD_NUMBER: _ClassVar[int]
        CODE_FIELD_NUMBER: _ClassVar[int]
        root: Subject.Role.Root
        administrator: Subject.Role.Administrator
        id: int
        code: str
        def __init__(self, root: _Optional[_Union[Subject.Role.Root, _Mapping]] = ..., administrator: _Optional[_Union[Subject.Role.Administrator, _Mapping]] = ..., id: _Optional[int] = ..., code: _Optional[str] = ...) -> None: ...
    class User(_message.Message):
        __slots__ = ("id", "code")
        ID_FIELD_NUMBER: _ClassVar[int]
        CODE_FIELD_NUMBER: _ClassVar[int]
        id: int
        code: str
        def __init__(self, id: _Optional[int] = ..., code: _Optional[str] = ...) -> None: ...
    USER_FIELD_NUMBER: _ClassVar[int]
    ROLE_FIELD_NUMBER: _ClassVar[int]
    user: Subject.User
    role: Subject.Role
    def __init__(self, user: _Optional[_Union[Subject.User, _Mapping]] = ..., role: _Optional[_Union[Subject.Role, _Mapping]] = ...) -> None: ...

class Action(_message.Message):
    __slots__ = ("read", "write", "append", "execute", "credit", "debit", "inquiry", "code")
    class Read(_message.Message):
        __slots__ = ()
        def __init__(self) -> None: ...
    class Write(_message.Message):
        __slots__ = ()
        def __init__(self) -> None: ...
    class Append(_message.Message):
        __slots__ = ()
        def __init__(self) -> None: ...
    class Execute(_message.Message):
        __slots__ = ()
        def __init__(self) -> None: ...
    class Credit(_message.Message):
        __slots__ = ()
        def __init__(self) -> None: ...
    class Debit(_message.Message):
        __slots__ = ()
        def __init__(self) -> None: ...
    class Inquiry(_message.Message):
        __slots__ = ()
        def __init__(self) -> None: ...
    READ_FIELD_NUMBER: _ClassVar[int]
    WRITE_FIELD_NUMBER: _ClassVar[int]
    APPEND_FIELD_NUMBER: _ClassVar[int]
    EXECUTE_FIELD_NUMBER: _ClassVar[int]
    CREDIT_FIELD_NUMBER: _ClassVar[int]
    DEBIT_FIELD_NUMBER: _ClassVar[int]
    INQUIRY_FIELD_NUMBER: _ClassVar[int]
    CODE_FIELD_NUMBER: _ClassVar[int]
    read: Action.Read
    write: Action.Write
    append: Action.Append
    execute: Action.Execute
    credit: Action.Credit
    debit: Action.Debit
    inquiry: Action.Inquiry
    code: str
    def __init__(self, read: _Optional[_Union[Action.Read, _Mapping]] = ..., write: _Optional[_Union[Action.Write, _Mapping]] = ..., append: _Optional[_Union[Action.Append, _Mapping]] = ..., execute: _Optional[_Union[Action.Execute, _Mapping]] = ..., credit: _Optional[_Union[Action.Credit, _Mapping]] = ..., debit: _Optional[_Union[Action.Debit, _Mapping]] = ..., inquiry: _Optional[_Union[Action.Inquiry, _Mapping]] = ..., code: _Optional[str] = ...) -> None: ...

class Permission(_message.Message):
    __slots__ = ("subject", "object", "action")
    SUBJECT_FIELD_NUMBER: _ClassVar[int]
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    ACTION_FIELD_NUMBER: _ClassVar[int]
    subject: Subject
    object: Object
    action: Action
    def __init__(self, subject: _Optional[_Union[Subject, _Mapping]] = ..., object: _Optional[_Union[Object, _Mapping]] = ..., action: _Optional[_Union[Action, _Mapping]] = ...) -> None: ...

class UserRoleRequest(_message.Message):
    __slots__ = ("user", "role")
    USER_FIELD_NUMBER: _ClassVar[int]
    ROLE_FIELD_NUMBER: _ClassVar[int]
    user: Subject.User
    role: Subject.Role
    def __init__(self, user: _Optional[_Union[Subject.User, _Mapping]] = ..., role: _Optional[_Union[Subject.Role, _Mapping]] = ...) -> None: ...

class RolesResponse(_message.Message):
    __slots__ = ("items",)
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[Subject.Role]
    def __init__(self, items: _Optional[_Iterable[_Union[Subject.Role, _Mapping]]] = ...) -> None: ...

class UsersResponse(_message.Message):
    __slots__ = ("items",)
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[Subject.User]
    def __init__(self, items: _Optional[_Iterable[_Union[Subject.User, _Mapping]]] = ...) -> None: ...

class SubjectsResponse(_message.Message):
    __slots__ = ("items",)
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[Subject]
    def __init__(self, items: _Optional[_Iterable[_Union[Subject, _Mapping]]] = ...) -> None: ...

class ObjectsResponse(_message.Message):
    __slots__ = ("items",)
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[Object]
    def __init__(self, items: _Optional[_Iterable[_Union[Object, _Mapping]]] = ...) -> None: ...

class ActionsResponse(_message.Message):
    __slots__ = ("items",)
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[Action]
    def __init__(self, items: _Optional[_Iterable[_Union[Action, _Mapping]]] = ...) -> None: ...

class PermissionsResponse(_message.Message):
    __slots__ = ("items",)
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[Permission]
    def __init__(self, items: _Optional[_Iterable[_Union[Permission, _Mapping]]] = ...) -> None: ...
