from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GoogleSignInUrlRequest(_message.Message):
    __slots__ = ("redirect_uri", "scopes")
    REDIRECT_URI_FIELD_NUMBER: _ClassVar[int]
    SCOPES_FIELD_NUMBER: _ClassVar[int]
    redirect_uri: str
    scopes: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, redirect_uri: _Optional[str] = ..., scopes: _Optional[_Iterable[str]] = ...) -> None: ...

class GoogleSignInUrlResponse(_message.Message):
    __slots__ = ("authorization_url", "state")
    AUTHORIZATION_URL_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    authorization_url: str
    state: str
    def __init__(self, authorization_url: _Optional[str] = ..., state: _Optional[str] = ...) -> None: ...

class GoogleFetchTokenRequest(_message.Message):
    __slots__ = ("authorization_response", "state", "scopes")
    AUTHORIZATION_RESPONSE_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    SCOPES_FIELD_NUMBER: _ClassVar[int]
    authorization_response: str
    state: str
    scopes: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, authorization_response: _Optional[str] = ..., state: _Optional[str] = ..., scopes: _Optional[_Iterable[str]] = ...) -> None: ...

class GoogleFetchTokenResponse(_message.Message):
    __slots__ = ("credentials",)
    class Credentials(_message.Message):
        __slots__ = ("token", "refresh_token", "granted_scopes")
        TOKEN_FIELD_NUMBER: _ClassVar[int]
        REFRESH_TOKEN_FIELD_NUMBER: _ClassVar[int]
        GRANTED_SCOPES_FIELD_NUMBER: _ClassVar[int]
        token: str
        refresh_token: str
        granted_scopes: _containers.RepeatedScalarFieldContainer[str]
        def __init__(self, token: _Optional[str] = ..., refresh_token: _Optional[str] = ..., granted_scopes: _Optional[_Iterable[str]] = ...) -> None: ...
    CREDENTIALS_FIELD_NUMBER: _ClassVar[int]
    credentials: GoogleFetchTokenResponse.Credentials
    def __init__(self, credentials: _Optional[_Union[GoogleFetchTokenResponse.Credentials, _Mapping]] = ...) -> None: ...

class GoogleRevokeResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class GoogleUserInfoResponse(_message.Message):
    __slots__ = ("sub", "email", "verified_email", "name", "given_name", "family_name", "picture", "locale", "hosted_domain", "gender", "link")
    SUB_FIELD_NUMBER: _ClassVar[int]
    EMAIL_FIELD_NUMBER: _ClassVar[int]
    VERIFIED_EMAIL_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    GIVEN_NAME_FIELD_NUMBER: _ClassVar[int]
    FAMILY_NAME_FIELD_NUMBER: _ClassVar[int]
    PICTURE_FIELD_NUMBER: _ClassVar[int]
    LOCALE_FIELD_NUMBER: _ClassVar[int]
    HOSTED_DOMAIN_FIELD_NUMBER: _ClassVar[int]
    GENDER_FIELD_NUMBER: _ClassVar[int]
    LINK_FIELD_NUMBER: _ClassVar[int]
    sub: str
    email: str
    verified_email: bool
    name: str
    given_name: str
    family_name: str
    picture: str
    locale: str
    hosted_domain: str
    gender: str
    link: str
    def __init__(self, sub: _Optional[str] = ..., email: _Optional[str] = ..., verified_email: _Optional[bool] = ..., name: _Optional[str] = ..., given_name: _Optional[str] = ..., family_name: _Optional[str] = ..., picture: _Optional[str] = ..., locale: _Optional[str] = ..., hosted_domain: _Optional[str] = ..., gender: _Optional[str] = ..., link: _Optional[str] = ...) -> None: ...
