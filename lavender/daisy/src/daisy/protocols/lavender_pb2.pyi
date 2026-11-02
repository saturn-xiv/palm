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
    __slots__ = ("node", "pod", "container", "message", "owners", "created_at")
    class Owner(_message.Message):
        __slots__ = ("kind", "name", "uid")
        KIND_FIELD_NUMBER: _ClassVar[int]
        NAME_FIELD_NUMBER: _ClassVar[int]
        UID_FIELD_NUMBER: _ClassVar[int]
        kind: str
        name: str
        uid: str
        def __init__(self, kind: _Optional[str] = ..., name: _Optional[str] = ..., uid: _Optional[str] = ...) -> None: ...
    NODE_FIELD_NUMBER: _ClassVar[int]
    POD_FIELD_NUMBER: _ClassVar[int]
    CONTAINER_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    OWNERS_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    node: str
    pod: str
    container: str
    message: str
    owners: _containers.RepeatedCompositeFieldContainer[KubernetesRequest.Owner]
    created_at: Timestamp
    def __init__(self, node: _Optional[str] = ..., pod: _Optional[str] = ..., container: _Optional[str] = ..., message: _Optional[str] = ..., owners: _Optional[_Iterable[_Union[KubernetesRequest.Owner, _Mapping]]] = ..., created_at: _Optional[_Union[Timestamp, _Mapping]] = ...) -> None: ...

class KubernetesResponse(_message.Message):
    __slots__ = ("chunk_count",)
    CHUNK_COUNT_FIELD_NUMBER: _ClassVar[int]
    chunk_count: int
    def __init__(self, chunk_count: _Optional[int] = ...) -> None: ...

class ClawRequest(_message.Message):
    __slots__ = ("elapsed", "created_at", "http", "postgresql", "mysql", "redis", "opensearch", "php_fpm", "snmp")
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
    class PhpFpm(_message.Message):
        __slots__ = ("x_powered_by", "expires", "cache_control", "content_type", "body")
        class Body(_message.Message):
            __slots__ = ("pool", "process_manager", "start_time", "start_since", "accepted_conn", "listen_queue", "max_listen_queue", "listen_queue_len", "idle_processes", "active_process", "total_process", "max_active_processes", "max_children_reached", "slow_requests", "memory_peak", "processes")
            class Process(_message.Message):
                __slots__ = ("pid", "state", "start_time", "start_since", "requests", "request_duration", "request_method", "request_uri", "content_length", "user", "script", "last_request_cpu", "last_request_memory")
                PID_FIELD_NUMBER: _ClassVar[int]
                STATE_FIELD_NUMBER: _ClassVar[int]
                START_TIME_FIELD_NUMBER: _ClassVar[int]
                START_SINCE_FIELD_NUMBER: _ClassVar[int]
                REQUESTS_FIELD_NUMBER: _ClassVar[int]
                REQUEST_DURATION_FIELD_NUMBER: _ClassVar[int]
                REQUEST_METHOD_FIELD_NUMBER: _ClassVar[int]
                REQUEST_URI_FIELD_NUMBER: _ClassVar[int]
                CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
                USER_FIELD_NUMBER: _ClassVar[int]
                SCRIPT_FIELD_NUMBER: _ClassVar[int]
                LAST_REQUEST_CPU_FIELD_NUMBER: _ClassVar[int]
                LAST_REQUEST_MEMORY_FIELD_NUMBER: _ClassVar[int]
                pid: int
                state: str
                start_time: int
                start_since: int
                requests: int
                request_duration: int
                request_method: str
                request_uri: str
                content_length: int
                user: str
                script: str
                last_request_cpu: float
                last_request_memory: int
                def __init__(self, pid: _Optional[int] = ..., state: _Optional[str] = ..., start_time: _Optional[int] = ..., start_since: _Optional[int] = ..., requests: _Optional[int] = ..., request_duration: _Optional[int] = ..., request_method: _Optional[str] = ..., request_uri: _Optional[str] = ..., content_length: _Optional[int] = ..., user: _Optional[str] = ..., script: _Optional[str] = ..., last_request_cpu: _Optional[float] = ..., last_request_memory: _Optional[int] = ...) -> None: ...
            POOL_FIELD_NUMBER: _ClassVar[int]
            PROCESS_MANAGER_FIELD_NUMBER: _ClassVar[int]
            START_TIME_FIELD_NUMBER: _ClassVar[int]
            START_SINCE_FIELD_NUMBER: _ClassVar[int]
            ACCEPTED_CONN_FIELD_NUMBER: _ClassVar[int]
            LISTEN_QUEUE_FIELD_NUMBER: _ClassVar[int]
            MAX_LISTEN_QUEUE_FIELD_NUMBER: _ClassVar[int]
            LISTEN_QUEUE_LEN_FIELD_NUMBER: _ClassVar[int]
            IDLE_PROCESSES_FIELD_NUMBER: _ClassVar[int]
            ACTIVE_PROCESS_FIELD_NUMBER: _ClassVar[int]
            TOTAL_PROCESS_FIELD_NUMBER: _ClassVar[int]
            MAX_ACTIVE_PROCESSES_FIELD_NUMBER: _ClassVar[int]
            MAX_CHILDREN_REACHED_FIELD_NUMBER: _ClassVar[int]
            SLOW_REQUESTS_FIELD_NUMBER: _ClassVar[int]
            MEMORY_PEAK_FIELD_NUMBER: _ClassVar[int]
            PROCESSES_FIELD_NUMBER: _ClassVar[int]
            pool: str
            process_manager: str
            start_time: int
            start_since: int
            accepted_conn: int
            listen_queue: int
            max_listen_queue: int
            listen_queue_len: int
            idle_processes: int
            active_process: int
            total_process: int
            max_active_processes: int
            max_children_reached: int
            slow_requests: int
            memory_peak: int
            processes: _containers.RepeatedCompositeFieldContainer[ClawRequest.PhpFpm.Body.Process]
            def __init__(self, pool: _Optional[str] = ..., process_manager: _Optional[str] = ..., start_time: _Optional[int] = ..., start_since: _Optional[int] = ..., accepted_conn: _Optional[int] = ..., listen_queue: _Optional[int] = ..., max_listen_queue: _Optional[int] = ..., listen_queue_len: _Optional[int] = ..., idle_processes: _Optional[int] = ..., active_process: _Optional[int] = ..., total_process: _Optional[int] = ..., max_active_processes: _Optional[int] = ..., max_children_reached: _Optional[int] = ..., slow_requests: _Optional[int] = ..., memory_peak: _Optional[int] = ..., processes: _Optional[_Iterable[_Union[ClawRequest.PhpFpm.Body.Process, _Mapping]]] = ...) -> None: ...
        X_POWERED_BY_FIELD_NUMBER: _ClassVar[int]
        EXPIRES_FIELD_NUMBER: _ClassVar[int]
        CACHE_CONTROL_FIELD_NUMBER: _ClassVar[int]
        CONTENT_TYPE_FIELD_NUMBER: _ClassVar[int]
        BODY_FIELD_NUMBER: _ClassVar[int]
        x_powered_by: str
        expires: str
        cache_control: str
        content_type: str
        body: ClawRequest.PhpFpm.Body
        def __init__(self, x_powered_by: _Optional[str] = ..., expires: _Optional[str] = ..., cache_control: _Optional[str] = ..., content_type: _Optional[str] = ..., body: _Optional[_Union[ClawRequest.PhpFpm.Body, _Mapping]] = ...) -> None: ...
    FROM_FIELD_NUMBER: _ClassVar[int]
    ELAPSED_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    HTTP_FIELD_NUMBER: _ClassVar[int]
    POSTGRESQL_FIELD_NUMBER: _ClassVar[int]
    MYSQL_FIELD_NUMBER: _ClassVar[int]
    REDIS_FIELD_NUMBER: _ClassVar[int]
    OPENSEARCH_FIELD_NUMBER: _ClassVar[int]
    PHP_FPM_FIELD_NUMBER: _ClassVar[int]
    SNMP_FIELD_NUMBER: _ClassVar[int]
    elapsed: Duration
    created_at: Timestamp
    http: ClawRequest.Http
    postgresql: ClawRequest.PostgreSql
    mysql: ClawRequest.MySql
    redis: ClawRequest.Redis
    opensearch: ClawRequest.OpenSearch
    php_fpm: ClawRequest.PhpFpm
    snmp: ClawRequest.Snmp
    def __init__(self, elapsed: _Optional[_Union[Duration, _Mapping]] = ..., created_at: _Optional[_Union[Timestamp, _Mapping]] = ..., http: _Optional[_Union[ClawRequest.Http, _Mapping]] = ..., postgresql: _Optional[_Union[ClawRequest.PostgreSql, _Mapping]] = ..., mysql: _Optional[_Union[ClawRequest.MySql, _Mapping]] = ..., redis: _Optional[_Union[ClawRequest.Redis, _Mapping]] = ..., opensearch: _Optional[_Union[ClawRequest.OpenSearch, _Mapping]] = ..., php_fpm: _Optional[_Union[ClawRequest.PhpFpm, _Mapping]] = ..., snmp: _Optional[_Union[ClawRequest.Snmp, _Mapping]] = ..., **kwargs) -> None: ...
