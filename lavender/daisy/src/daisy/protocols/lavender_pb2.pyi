from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Empty(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class Timestamp(_message.Message):
    __slots__ = ("seconds", "nanos")
    SECONDS_FIELD_NUMBER: _ClassVar[int]
    NANOS_FIELD_NUMBER: _ClassVar[int]
    seconds: int
    nanos: int
    def __init__(self, seconds: _Optional[int] = ..., nanos: _Optional[int] = ...) -> None: ...

class Duration(_message.Message):
    __slots__ = ("seconds", "nanos")
    SECONDS_FIELD_NUMBER: _ClassVar[int]
    NANOS_FIELD_NUMBER: _ClassVar[int]
    seconds: int
    nanos: int
    def __init__(self, seconds: _Optional[int] = ..., nanos: _Optional[int] = ...) -> None: ...

class SystemdRequest(_message.Message):
    __slots__ = ("host", "unit", "priority", "message", "created_at")
    HOST_FIELD_NUMBER: _ClassVar[int]
    UNIT_FIELD_NUMBER: _ClassVar[int]
    PRIORITY_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    host: str
    unit: str
    priority: int
    message: str
    created_at: Timestamp
    def __init__(self, host: _Optional[str] = ..., unit: _Optional[str] = ..., priority: _Optional[int] = ..., message: _Optional[str] = ..., created_at: _Optional[_Union[Timestamp, _Mapping]] = ...) -> None: ...

class SystemdResponse(_message.Message):
    __slots__ = ("chunk_count",)
    CHUNK_COUNT_FIELD_NUMBER: _ClassVar[int]
    chunk_count: int
    def __init__(self, chunk_count: _Optional[int] = ...) -> None: ...

class KubernetesRequest(_message.Message):
    __slots__ = ("node", "pod", "container", "message", "created_at")
    NODE_FIELD_NUMBER: _ClassVar[int]
    POD_FIELD_NUMBER: _ClassVar[int]
    CONTAINER_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    node: str
    pod: str
    container: str
    message: str
    created_at: Timestamp
    def __init__(self, node: _Optional[str] = ..., pod: _Optional[str] = ..., container: _Optional[str] = ..., message: _Optional[str] = ..., created_at: _Optional[_Union[Timestamp, _Mapping]] = ...) -> None: ...

class KubernetesResponse(_message.Message):
    __slots__ = ("chunk_count",)
    CHUNK_COUNT_FIELD_NUMBER: _ClassVar[int]
    chunk_count: int
    def __init__(self, chunk_count: _Optional[int] = ...) -> None: ...

class ClawRequest(_message.Message):
    __slots__ = ("elapsed", "created_at", "error", "http", "postgresql", "mysql", "redis", "opensearch", "snmp")
    class Error(_message.Message):
        __slots__ = ("reason",)
        REASON_FIELD_NUMBER: _ClassVar[int]
        reason: str
        def __init__(self, reason: _Optional[str] = ...) -> None: ...
    class Http(_message.Message):
        __slots__ = ("url", "status_code", "content_type", "body")
        URL_FIELD_NUMBER: _ClassVar[int]
        STATUS_CODE_FIELD_NUMBER: _ClassVar[int]
        CONTENT_TYPE_FIELD_NUMBER: _ClassVar[int]
        BODY_FIELD_NUMBER: _ClassVar[int]
        url: str
        status_code: int
        content_type: str
        body: str
        def __init__(self, url: _Optional[str] = ..., status_code: _Optional[int] = ..., content_type: _Optional[str] = ..., body: _Optional[str] = ...) -> None: ...
    class PostgreSql(_message.Message):
        __slots__ = ("version",)
        VERSION_FIELD_NUMBER: _ClassVar[int]
        version: str
        def __init__(self, version: _Optional[str] = ...) -> None: ...
    class MySql(_message.Message):
        __slots__ = ("version",)
        VERSION_FIELD_NUMBER: _ClassVar[int]
        version: str
        def __init__(self, version: _Optional[str] = ...) -> None: ...
    class Redis(_message.Message):
        __slots__ = ("info", "cluster")
        INFO_FIELD_NUMBER: _ClassVar[int]
        CLUSTER_FIELD_NUMBER: _ClassVar[int]
        info: str
        cluster: str
        def __init__(self, info: _Optional[str] = ..., cluster: _Optional[str] = ...) -> None: ...
    class Snmp(_message.Message):
        __slots__ = ("items",)
        class Item(_message.Message):
            __slots__ = ("oid", "i", "s", "d")
            OID_FIELD_NUMBER: _ClassVar[int]
            I_FIELD_NUMBER: _ClassVar[int]
            S_FIELD_NUMBER: _ClassVar[int]
            D_FIELD_NUMBER: _ClassVar[int]
            oid: str
            i: int
            s: str
            d: float
            def __init__(self, oid: _Optional[str] = ..., i: _Optional[int] = ..., s: _Optional[str] = ..., d: _Optional[float] = ...) -> None: ...
        ITEMS_FIELD_NUMBER: _ClassVar[int]
        items: _containers.RepeatedCompositeFieldContainer[ClawRequest.Snmp.Item]
        def __init__(self, items: _Optional[_Iterable[_Union[ClawRequest.Snmp.Item, _Mapping]]] = ...) -> None: ...
    class OpenSearch(_message.Message):
        __slots__ = ("cluster_name", "status", "timed_out", "number_of_nodes", "number_of_data_nodes", "discovered_master", "discovered_cluster_manager", "active_primary_shards", "active_shards", "relocating_shards", "initializing_shards", "unassigned_shards", "delayed_unassigned_shards", "number_of_pending_tasks", "number_of_in_flight_fetch", "task_max_waiting_in_queue_millis", "active_shards_percent_as_number")
        CLUSTER_NAME_FIELD_NUMBER: _ClassVar[int]
        STATUS_FIELD_NUMBER: _ClassVar[int]
        TIMED_OUT_FIELD_NUMBER: _ClassVar[int]
        NUMBER_OF_NODES_FIELD_NUMBER: _ClassVar[int]
        NUMBER_OF_DATA_NODES_FIELD_NUMBER: _ClassVar[int]
        DISCOVERED_MASTER_FIELD_NUMBER: _ClassVar[int]
        DISCOVERED_CLUSTER_MANAGER_FIELD_NUMBER: _ClassVar[int]
        ACTIVE_PRIMARY_SHARDS_FIELD_NUMBER: _ClassVar[int]
        ACTIVE_SHARDS_FIELD_NUMBER: _ClassVar[int]
        RELOCATING_SHARDS_FIELD_NUMBER: _ClassVar[int]
        INITIALIZING_SHARDS_FIELD_NUMBER: _ClassVar[int]
        UNASSIGNED_SHARDS_FIELD_NUMBER: _ClassVar[int]
        DELAYED_UNASSIGNED_SHARDS_FIELD_NUMBER: _ClassVar[int]
        NUMBER_OF_PENDING_TASKS_FIELD_NUMBER: _ClassVar[int]
        NUMBER_OF_IN_FLIGHT_FETCH_FIELD_NUMBER: _ClassVar[int]
        TASK_MAX_WAITING_IN_QUEUE_MILLIS_FIELD_NUMBER: _ClassVar[int]
        ACTIVE_SHARDS_PERCENT_AS_NUMBER_FIELD_NUMBER: _ClassVar[int]
        cluster_name: str
        status: str
        timed_out: bool
        number_of_nodes: int
        number_of_data_nodes: int
        discovered_master: bool
        discovered_cluster_manager: bool
        active_primary_shards: int
        active_shards: int
        relocating_shards: int
        initializing_shards: int
        unassigned_shards: int
        delayed_unassigned_shards: int
        number_of_pending_tasks: int
        number_of_in_flight_fetch: int
        task_max_waiting_in_queue_millis: int
        active_shards_percent_as_number: float
        def __init__(self, cluster_name: _Optional[str] = ..., status: _Optional[str] = ..., timed_out: _Optional[bool] = ..., number_of_nodes: _Optional[int] = ..., number_of_data_nodes: _Optional[int] = ..., discovered_master: _Optional[bool] = ..., discovered_cluster_manager: _Optional[bool] = ..., active_primary_shards: _Optional[int] = ..., active_shards: _Optional[int] = ..., relocating_shards: _Optional[int] = ..., initializing_shards: _Optional[int] = ..., unassigned_shards: _Optional[int] = ..., delayed_unassigned_shards: _Optional[int] = ..., number_of_pending_tasks: _Optional[int] = ..., number_of_in_flight_fetch: _Optional[int] = ..., task_max_waiting_in_queue_millis: _Optional[int] = ..., active_shards_percent_as_number: _Optional[float] = ...) -> None: ...
    FROM_FIELD_NUMBER: _ClassVar[int]
    ELAPSED_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    ERROR_FIELD_NUMBER: _ClassVar[int]
    HTTP_FIELD_NUMBER: _ClassVar[int]
    POSTGRESQL_FIELD_NUMBER: _ClassVar[int]
    MYSQL_FIELD_NUMBER: _ClassVar[int]
    REDIS_FIELD_NUMBER: _ClassVar[int]
    OPENSEARCH_FIELD_NUMBER: _ClassVar[int]
    SNMP_FIELD_NUMBER: _ClassVar[int]
    elapsed: Duration
    created_at: Timestamp
    error: ClawRequest.Error
    http: ClawRequest.Http
    postgresql: ClawRequest.PostgreSql
    mysql: ClawRequest.MySql
    redis: ClawRequest.Redis
    opensearch: ClawRequest.OpenSearch
    snmp: ClawRequest.Snmp
    def __init__(self, elapsed: _Optional[_Union[Duration, _Mapping]] = ..., created_at: _Optional[_Union[Timestamp, _Mapping]] = ..., error: _Optional[_Union[ClawRequest.Error, _Mapping]] = ..., http: _Optional[_Union[ClawRequest.Http, _Mapping]] = ..., postgresql: _Optional[_Union[ClawRequest.PostgreSql, _Mapping]] = ..., mysql: _Optional[_Union[ClawRequest.MySql, _Mapping]] = ..., redis: _Optional[_Union[ClawRequest.Redis, _Mapping]] = ..., opensearch: _Optional[_Union[ClawRequest.OpenSearch, _Mapping]] = ..., snmp: _Optional[_Union[ClawRequest.Snmp, _Mapping]] = ..., **kwargs) -> None: ...
