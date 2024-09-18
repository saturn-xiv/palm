from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class PolicyHasRequest(_message.Message):
    __slots__ = ("user", "role")
    USER_FIELD_NUMBER: _ClassVar[int]
    ROLE_FIELD_NUMBER: _ClassVar[int]
    user: PolicyUsersResponse.Item
    role: PolicyRolesResponse.Item
    def __init__(self, user: _Optional[_Union[PolicyUsersResponse.Item, _Mapping]] = ..., role: _Optional[_Union[PolicyRolesResponse.Item, _Mapping]] = ...) -> None: ...

class PolicyCanRequest(_message.Message):
    __slots__ = ("user", "resource", "operation")
    USER_FIELD_NUMBER: _ClassVar[int]
    RESOURCE_FIELD_NUMBER: _ClassVar[int]
    OPERATION_FIELD_NUMBER: _ClassVar[int]
    user: PolicyUsersResponse.Item
    resource: PolicyPermissionsResponse.Item.Resource
    operation: PolicyPermissionsResponse.Item.Operation
    def __init__(self, user: _Optional[_Union[PolicyUsersResponse.Item, _Mapping]] = ..., resource: _Optional[_Union[PolicyPermissionsResponse.Item.Resource, _Mapping]] = ..., operation: _Optional[_Union[PolicyPermissionsResponse.Item.Operation, _Mapping]] = ...) -> None: ...

class PolicyUsersResponse(_message.Message):
    __slots__ = ("items",)
    class Item(_message.Message):
        __slots__ = ("i", "s")
        I_FIELD_NUMBER: _ClassVar[int]
        S_FIELD_NUMBER: _ClassVar[int]
        i: int
        s: str
        def __init__(self, i: _Optional[int] = ..., s: _Optional[str] = ...) -> None: ...
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[PolicyUsersResponse.Item]
    def __init__(self, items: _Optional[_Iterable[_Union[PolicyUsersResponse.Item, _Mapping]]] = ...) -> None: ...

class PolicyRolesResponse(_message.Message):
    __slots__ = ("items",)
    class Item(_message.Message):
        __slots__ = ("root", "administrator", "code")
        class Administrator(_message.Message):
            __slots__ = ()
            def __init__(self) -> None: ...
        class Root(_message.Message):
            __slots__ = ()
            def __init__(self) -> None: ...
        ROOT_FIELD_NUMBER: _ClassVar[int]
        ADMINISTRATOR_FIELD_NUMBER: _ClassVar[int]
        CODE_FIELD_NUMBER: _ClassVar[int]
        root: PolicyRolesResponse.Item.Root
        administrator: PolicyRolesResponse.Item.Administrator
        code: str
        def __init__(self, root: _Optional[_Union[PolicyRolesResponse.Item.Root, _Mapping]] = ..., administrator: _Optional[_Union[PolicyRolesResponse.Item.Administrator, _Mapping]] = ..., code: _Optional[str] = ...) -> None: ...
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[PolicyRolesResponse.Item]
    def __init__(self, items: _Optional[_Iterable[_Union[PolicyRolesResponse.Item, _Mapping]]] = ...) -> None: ...

class PolicyRolesForUserRequest(_message.Message):
    __slots__ = ("user", "roles")
    USER_FIELD_NUMBER: _ClassVar[int]
    ROLES_FIELD_NUMBER: _ClassVar[int]
    user: PolicyUsersResponse.Item
    roles: _containers.RepeatedCompositeFieldContainer[PolicyRolesResponse.Item]
    def __init__(self, user: _Optional[_Union[PolicyUsersResponse.Item, _Mapping]] = ..., roles: _Optional[_Iterable[_Union[PolicyRolesResponse.Item, _Mapping]]] = ...) -> None: ...

class PolicyPermissionsForUserRequest(_message.Message):
    __slots__ = ("user", "permissions")
    USER_FIELD_NUMBER: _ClassVar[int]
    PERMISSIONS_FIELD_NUMBER: _ClassVar[int]
    user: PolicyUsersResponse.Item
    permissions: _containers.RepeatedCompositeFieldContainer[PolicyPermissionsResponse.Item]
    def __init__(self, user: _Optional[_Union[PolicyUsersResponse.Item, _Mapping]] = ..., permissions: _Optional[_Iterable[_Union[PolicyPermissionsResponse.Item, _Mapping]]] = ...) -> None: ...

class PolicyPermissionsForRoleRequest(_message.Message):
    __slots__ = ("role", "permissions")
    ROLE_FIELD_NUMBER: _ClassVar[int]
    PERMISSIONS_FIELD_NUMBER: _ClassVar[int]
    role: PolicyRolesResponse.Item
    permissions: _containers.RepeatedCompositeFieldContainer[PolicyPermissionsResponse.Item]
    def __init__(self, role: _Optional[_Union[PolicyRolesResponse.Item, _Mapping]] = ..., permissions: _Optional[_Iterable[_Union[PolicyPermissionsResponse.Item, _Mapping]]] = ...) -> None: ...

class PolicyPermissionsResponse(_message.Message):
    __slots__ = ("items",)
    class Item(_message.Message):
        __slots__ = ("resource", "operation")
        class Resource(_message.Message):
            __slots__ = ("type", "id")
            class Id(_message.Message):
                __slots__ = ("i", "s")
                I_FIELD_NUMBER: _ClassVar[int]
                S_FIELD_NUMBER: _ClassVar[int]
                i: int
                s: str
                def __init__(self, i: _Optional[int] = ..., s: _Optional[str] = ...) -> None: ...
            TYPE_FIELD_NUMBER: _ClassVar[int]
            ID_FIELD_NUMBER: _ClassVar[int]
            type: str
            id: PolicyPermissionsResponse.Item.Resource.Id
            def __init__(self, type: _Optional[str] = ..., id: _Optional[_Union[PolicyPermissionsResponse.Item.Resource.Id, _Mapping]] = ...) -> None: ...
        class Operation(_message.Message):
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
            read: PolicyPermissionsResponse.Item.Operation.Read
            write: PolicyPermissionsResponse.Item.Operation.Write
            append: PolicyPermissionsResponse.Item.Operation.Append
            execute: PolicyPermissionsResponse.Item.Operation.Execute
            credit: PolicyPermissionsResponse.Item.Operation.Credit
            debit: PolicyPermissionsResponse.Item.Operation.Debit
            inquiry: PolicyPermissionsResponse.Item.Operation.Inquiry
            code: str
            def __init__(self, read: _Optional[_Union[PolicyPermissionsResponse.Item.Operation.Read, _Mapping]] = ..., write: _Optional[_Union[PolicyPermissionsResponse.Item.Operation.Write, _Mapping]] = ..., append: _Optional[_Union[PolicyPermissionsResponse.Item.Operation.Append, _Mapping]] = ..., execute: _Optional[_Union[PolicyPermissionsResponse.Item.Operation.Execute, _Mapping]] = ..., credit: _Optional[_Union[PolicyPermissionsResponse.Item.Operation.Credit, _Mapping]] = ..., debit: _Optional[_Union[PolicyPermissionsResponse.Item.Operation.Debit, _Mapping]] = ..., inquiry: _Optional[_Union[PolicyPermissionsResponse.Item.Operation.Inquiry, _Mapping]] = ..., code: _Optional[str] = ...) -> None: ...
        RESOURCE_FIELD_NUMBER: _ClassVar[int]
        OPERATION_FIELD_NUMBER: _ClassVar[int]
        resource: PolicyPermissionsResponse.Item.Resource
        operation: PolicyPermissionsResponse.Item.Operation
        def __init__(self, resource: _Optional[_Union[PolicyPermissionsResponse.Item.Resource, _Mapping]] = ..., operation: _Optional[_Union[PolicyPermissionsResponse.Item.Operation, _Mapping]] = ...) -> None: ...
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[PolicyPermissionsResponse.Item]
    def __init__(self, items: _Optional[_Iterable[_Union[PolicyPermissionsResponse.Item, _Mapping]]] = ...) -> None: ...
