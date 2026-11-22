from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class PasswordBox(_message.Message):
    __slots__ = ("payload", "salt")
    PAYLOAD_FIELD_NUMBER: _ClassVar[int]
    SALT_FIELD_NUMBER: _ClassVar[int]
    payload: str
    salt: float
    def __init__(self, payload: _Optional[str] = ..., salt: _Optional[float] = ...) -> None: ...

class Oauth2State(_message.Message):
    __slots__ = ("type", "user")
    class Type(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = ()
        GOOGLE: _ClassVar[Oauth2State.Type]
        WECHAT: _ClassVar[Oauth2State.Type]
    GOOGLE: Oauth2State.Type
    WECHAT: Oauth2State.Type
    TYPE_FIELD_NUMBER: _ClassVar[int]
    USER_FIELD_NUMBER: _ClassVar[int]
    type: Oauth2State.Type
    user: str
    def __init__(self, type: _Optional[_Union[Oauth2State.Type, str]] = ..., user: _Optional[str] = ...) -> None: ...

class UserIndexResponse(_message.Message):
    __slots__ = ("items",)
    class Item(_message.Message):
        __slots__ = ("sn", "type")
        class Type(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
            __slots__ = ()
            EMAIL: _ClassVar[UserIndexResponse.Item.Type]
            SMS: _ClassVar[UserIndexResponse.Item.Type]
            GOOGLE_OAUTH2: _ClassVar[UserIndexResponse.Item.Type]
            WECHAT_OAUTH2: _ClassVar[UserIndexResponse.Item.Type]
            WECHAT_MINI_PROGRAM: _ClassVar[UserIndexResponse.Item.Type]
        EMAIL: UserIndexResponse.Item.Type
        SMS: UserIndexResponse.Item.Type
        GOOGLE_OAUTH2: UserIndexResponse.Item.Type
        WECHAT_OAUTH2: UserIndexResponse.Item.Type
        WECHAT_MINI_PROGRAM: UserIndexResponse.Item.Type
        SN_FIELD_NUMBER: _ClassVar[int]
        TYPE_FIELD_NUMBER: _ClassVar[int]
        sn: str
        type: UserIndexResponse.Item.Type
        def __init__(self, sn: _Optional[str] = ..., type: _Optional[_Union[UserIndexResponse.Item.Type, str]] = ...) -> None: ...
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[UserIndexResponse.Item]
    def __init__(self, items: _Optional[_Iterable[_Union[UserIndexResponse.Item, _Mapping]]] = ...) -> None: ...

class UserToken(_message.Message):
    __slots__ = ()
    class Audience(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = ()
        SIGN_IN: _ClassVar[UserToken.Audience]
        HYDRANGEA_JOB: _ClassVar[UserToken.Audience]
    SIGN_IN: UserToken.Audience
    HYDRANGEA_JOB: UserToken.Audience
    class Payload(_message.Message):
        __slots__ = ("type",)
        TYPE_FIELD_NUMBER: _ClassVar[int]
        type: UserIndexResponse.Item.Type
        def __init__(self, type: _Optional[_Union[UserIndexResponse.Item.Type, str]] = ...) -> None: ...
    def __init__(self) -> None: ...
