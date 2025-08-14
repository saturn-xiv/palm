import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"
import * as portal_pb from './portal_pb'; // proto import: "portal.proto"


export class SiteLayoutRequest extends jspb.Message {
  getLang(): string;
  setLang(value: string): SiteLayoutRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteLayoutRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SiteLayoutRequest): SiteLayoutRequest.AsObject;
  static serializeBinaryToWriter(message: SiteLayoutRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteLayoutRequest;
  static deserializeBinaryFromReader(message: SiteLayoutRequest, reader: jspb.BinaryReader): SiteLayoutRequest;
}

export namespace SiteLayoutRequest {
  export type AsObject = {
    lang: string,
  }
}

export class SiteLayoutResponse extends jspb.Message {
  getUser(): SiteLayoutResponse.CurrentUser | undefined;
  setUser(value?: SiteLayoutResponse.CurrentUser): SiteLayoutResponse;
  hasUser(): boolean;
  clearUser(): SiteLayoutResponse;

  getLocale(): string;
  setLocale(value: string): SiteLayoutResponse;

  getAvailableLanguagesList(): Array<string>;
  setAvailableLanguagesList(value: Array<string>): SiteLayoutResponse;
  clearAvailableLanguagesList(): SiteLayoutResponse;
  addAvailableLanguages(value: string, index?: number): SiteLayoutResponse;

  getGitVersion(): string;
  setGitVersion(value: string): SiteLayoutResponse;

  getBuildTime(): string;
  setBuildTime(value: string): SiteLayoutResponse;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): SiteLayoutResponse;
  hasCreatedAt(): boolean;
  clearCreatedAt(): SiteLayoutResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteLayoutResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SiteLayoutResponse): SiteLayoutResponse.AsObject;
  static serializeBinaryToWriter(message: SiteLayoutResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteLayoutResponse;
  static deserializeBinaryFromReader(message: SiteLayoutResponse, reader: jspb.BinaryReader): SiteLayoutResponse;
}

export namespace SiteLayoutResponse {
  export type AsObject = {
    user?: SiteLayoutResponse.CurrentUser.AsObject,
    locale: string,
    availableLanguagesList: Array<string>,
    gitVersion: string,
    buildTime: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }

  export class CurrentUser extends jspb.Message {
    getName(): string;
    setName(value: string): CurrentUser;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): CurrentUser.AsObject;
    static toObject(includeInstance: boolean, msg: CurrentUser): CurrentUser.AsObject;
    static serializeBinaryToWriter(message: CurrentUser, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): CurrentUser;
    static deserializeBinaryFromReader(message: CurrentUser, reader: jspb.BinaryReader): CurrentUser;
  }

  export namespace CurrentUser {
    export type AsObject = {
      name: string,
    }
  }


  export enum UserCase { 
    _USER_NOT_SET = 0,
    USER = 1,
  }
}

export class Heartbeat extends jspb.Message {
  getHttp(): Heartbeat.Http | undefined;
  setHttp(value?: Heartbeat.Http): Heartbeat;
  hasHttp(): boolean;
  clearHttp(): Heartbeat;

  getMysql(): Heartbeat.MySql | undefined;
  setMysql(value?: Heartbeat.MySql): Heartbeat;
  hasMysql(): boolean;
  clearMysql(): Heartbeat;

  getPostgresql(): Heartbeat.PostgreSql | undefined;
  setPostgresql(value?: Heartbeat.PostgreSql): Heartbeat;
  hasPostgresql(): boolean;
  clearPostgresql(): Heartbeat;

  getRabbitmq(): Heartbeat.RabbitMQ | undefined;
  setRabbitmq(value?: Heartbeat.RabbitMQ): Heartbeat;
  hasRabbitmq(): boolean;
  clearRabbitmq(): Heartbeat;

  getRedis(): Heartbeat.Redis | undefined;
  setRedis(value?: Heartbeat.Redis): Heartbeat;
  hasRedis(): boolean;
  clearRedis(): Heartbeat;

  getMinio(): Heartbeat.Minio | undefined;
  setMinio(value?: Heartbeat.Minio): Heartbeat;
  hasMinio(): boolean;
  clearMinio(): Heartbeat;

  getByCase(): Heartbeat.ByCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Heartbeat.AsObject;
  static toObject(includeInstance: boolean, msg: Heartbeat): Heartbeat.AsObject;
  static serializeBinaryToWriter(message: Heartbeat, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Heartbeat;
  static deserializeBinaryFromReader(message: Heartbeat, reader: jspb.BinaryReader): Heartbeat;
}

export namespace Heartbeat {
  export type AsObject = {
    http?: Heartbeat.Http.AsObject,
    mysql?: Heartbeat.MySql.AsObject,
    postgresql?: Heartbeat.PostgreSql.AsObject,
    rabbitmq?: Heartbeat.RabbitMQ.AsObject,
    redis?: Heartbeat.Redis.AsObject,
    minio?: Heartbeat.Minio.AsObject,
  }

  export class Http extends jspb.Message {
    getUrl(): string;
    setUrl(value: string): Http;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Http.AsObject;
    static toObject(includeInstance: boolean, msg: Http): Http.AsObject;
    static serializeBinaryToWriter(message: Http, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Http;
    static deserializeBinaryFromReader(message: Http, reader: jspb.BinaryReader): Http;
  }

  export namespace Http {
    export type AsObject = {
      url: string,
    }
  }


  export class MySql extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): MySql.AsObject;
    static toObject(includeInstance: boolean, msg: MySql): MySql.AsObject;
    static serializeBinaryToWriter(message: MySql, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): MySql;
    static deserializeBinaryFromReader(message: MySql, reader: jspb.BinaryReader): MySql;
  }

  export namespace MySql {
    export type AsObject = {
    }
  }


  export class PostgreSql extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): PostgreSql.AsObject;
    static toObject(includeInstance: boolean, msg: PostgreSql): PostgreSql.AsObject;
    static serializeBinaryToWriter(message: PostgreSql, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): PostgreSql;
    static deserializeBinaryFromReader(message: PostgreSql, reader: jspb.BinaryReader): PostgreSql;
  }

  export namespace PostgreSql {
    export type AsObject = {
    }
  }


  export class RabbitMQ extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RabbitMQ.AsObject;
    static toObject(includeInstance: boolean, msg: RabbitMQ): RabbitMQ.AsObject;
    static serializeBinaryToWriter(message: RabbitMQ, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RabbitMQ;
    static deserializeBinaryFromReader(message: RabbitMQ, reader: jspb.BinaryReader): RabbitMQ;
  }

  export namespace RabbitMQ {
    export type AsObject = {
    }
  }


  export class Redis extends jspb.Message {
    getHost(): string;
    setHost(value: string): Redis;

    getPort(): number;
    setPort(value: number): Redis;

    getCluster(): boolean;
    setCluster(value: boolean): Redis;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Redis.AsObject;
    static toObject(includeInstance: boolean, msg: Redis): Redis.AsObject;
    static serializeBinaryToWriter(message: Redis, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Redis;
    static deserializeBinaryFromReader(message: Redis, reader: jspb.BinaryReader): Redis;
  }

  export namespace Redis {
    export type AsObject = {
      host: string,
      port: number,
      cluster: boolean,
    }
  }


  export class Minio extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Minio.AsObject;
    static toObject(includeInstance: boolean, msg: Minio): Minio.AsObject;
    static serializeBinaryToWriter(message: Minio, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Minio;
    static deserializeBinaryFromReader(message: Minio, reader: jspb.BinaryReader): Minio;
  }

  export namespace Minio {
    export type AsObject = {
    }
  }


  export enum ByCase { 
    BY_NOT_SET = 0,
    HTTP = 1,
    MYSQL = 2,
    POSTGRESQL = 3,
    RABBITMQ = 4,
    REDIS = 5,
    MINIO = 6,
  }
}

export class SystemdJournalRequest extends jspb.Message {
  getAll(): SystemdJournalRequest.All | undefined;
  setAll(value?: SystemdJournalRequest.All): SystemdJournalRequest;
  hasAll(): boolean;
  clearAll(): SystemdJournalRequest;

  getHost(): string;
  setHost(value: string): SystemdJournalRequest;

  getName(): string;
  setName(value: string): SystemdJournalRequest;

  getPage(): portal_pb.Page | undefined;
  setPage(value?: portal_pb.Page): SystemdJournalRequest;
  hasPage(): boolean;
  clearPage(): SystemdJournalRequest;

  getByCase(): SystemdJournalRequest.ByCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SystemdJournalRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SystemdJournalRequest): SystemdJournalRequest.AsObject;
  static serializeBinaryToWriter(message: SystemdJournalRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SystemdJournalRequest;
  static deserializeBinaryFromReader(message: SystemdJournalRequest, reader: jspb.BinaryReader): SystemdJournalRequest;
}

export namespace SystemdJournalRequest {
  export type AsObject = {
    all?: SystemdJournalRequest.All.AsObject,
    host: string,
    name: string,
    page?: portal_pb.Page.AsObject,
  }

  export class All extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): All.AsObject;
    static toObject(includeInstance: boolean, msg: All): All.AsObject;
    static serializeBinaryToWriter(message: All, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): All;
    static deserializeBinaryFromReader(message: All, reader: jspb.BinaryReader): All;
  }

  export namespace All {
    export type AsObject = {
    }
  }


  export enum ByCase { 
    BY_NOT_SET = 0,
    ALL = 1,
    HOST = 2,
    NAME = 3,
  }
}

export class SystemdJournalResponse extends jspb.Message {
  getItemsList(): Array<SystemdJournalResponse.Item>;
  setItemsList(value: Array<SystemdJournalResponse.Item>): SystemdJournalResponse;
  clearItemsList(): SystemdJournalResponse;
  addItems(value?: SystemdJournalResponse.Item, index?: number): SystemdJournalResponse.Item;

  getPagination(): portal_pb.Pagination | undefined;
  setPagination(value?: portal_pb.Pagination): SystemdJournalResponse;
  hasPagination(): boolean;
  clearPagination(): SystemdJournalResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SystemdJournalResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SystemdJournalResponse): SystemdJournalResponse.AsObject;
  static serializeBinaryToWriter(message: SystemdJournalResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SystemdJournalResponse;
  static deserializeBinaryFromReader(message: SystemdJournalResponse, reader: jspb.BinaryReader): SystemdJournalResponse;
}

export namespace SystemdJournalResponse {
  export type AsObject = {
    itemsList: Array<SystemdJournalResponse.Item.AsObject>,
    pagination?: portal_pb.Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getHost(): string;
    setHost(value: string): Item;

    getName(): string;
    setName(value: string): Item;

    getMessage(): string;
    setMessage(value: string): Item;

    getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasCreatedAt(): boolean;
    clearCreatedAt(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      host: string,
      name: string,
      message: string,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
  }

}

export class FileSystemLogsRequest extends jspb.Message {
  getAll(): FileSystemLogsRequest.All | undefined;
  setAll(value?: FileSystemLogsRequest.All): FileSystemLogsRequest;
  hasAll(): boolean;
  clearAll(): FileSystemLogsRequest;

  getHost(): string;
  setHost(value: string): FileSystemLogsRequest;

  getFile(): string;
  setFile(value: string): FileSystemLogsRequest;

  getPage(): portal_pb.Page | undefined;
  setPage(value?: portal_pb.Page): FileSystemLogsRequest;
  hasPage(): boolean;
  clearPage(): FileSystemLogsRequest;

  getByCase(): FileSystemLogsRequest.ByCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FileSystemLogsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: FileSystemLogsRequest): FileSystemLogsRequest.AsObject;
  static serializeBinaryToWriter(message: FileSystemLogsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FileSystemLogsRequest;
  static deserializeBinaryFromReader(message: FileSystemLogsRequest, reader: jspb.BinaryReader): FileSystemLogsRequest;
}

export namespace FileSystemLogsRequest {
  export type AsObject = {
    all?: FileSystemLogsRequest.All.AsObject,
    host: string,
    file: string,
    page?: portal_pb.Page.AsObject,
  }

  export class All extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): All.AsObject;
    static toObject(includeInstance: boolean, msg: All): All.AsObject;
    static serializeBinaryToWriter(message: All, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): All;
    static deserializeBinaryFromReader(message: All, reader: jspb.BinaryReader): All;
  }

  export namespace All {
    export type AsObject = {
    }
  }


  export enum ByCase { 
    BY_NOT_SET = 0,
    ALL = 1,
    HOST = 2,
    FILE = 3,
  }
}

export class FileSystemLogsResponse extends jspb.Message {
  getItemsList(): Array<FileSystemLogsResponse.Item>;
  setItemsList(value: Array<FileSystemLogsResponse.Item>): FileSystemLogsResponse;
  clearItemsList(): FileSystemLogsResponse;
  addItems(value?: FileSystemLogsResponse.Item, index?: number): FileSystemLogsResponse.Item;

  getPagination(): portal_pb.Pagination | undefined;
  setPagination(value?: portal_pb.Pagination): FileSystemLogsResponse;
  hasPagination(): boolean;
  clearPagination(): FileSystemLogsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FileSystemLogsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: FileSystemLogsResponse): FileSystemLogsResponse.AsObject;
  static serializeBinaryToWriter(message: FileSystemLogsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FileSystemLogsResponse;
  static deserializeBinaryFromReader(message: FileSystemLogsResponse, reader: jspb.BinaryReader): FileSystemLogsResponse;
}

export namespace FileSystemLogsResponse {
  export type AsObject = {
    itemsList: Array<FileSystemLogsResponse.Item.AsObject>,
    pagination?: portal_pb.Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getHost(): string;
    setHost(value: string): Item;

    getFile(): string;
    setFile(value: string): Item;

    getLine(): string;
    setLine(value: string): Item;

    getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasCreatedAt(): boolean;
    clearCreatedAt(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      host: string,
      file: string,
      line: string,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
  }

}

export class DockerContainersResponse extends jspb.Message {
  getItemsList(): Array<DockerContainersResponse.Item>;
  setItemsList(value: Array<DockerContainersResponse.Item>): DockerContainersResponse;
  clearItemsList(): DockerContainersResponse;
  addItems(value?: DockerContainersResponse.Item, index?: number): DockerContainersResponse.Item;

  getPagination(): portal_pb.Pagination | undefined;
  setPagination(value?: portal_pb.Pagination): DockerContainersResponse;
  hasPagination(): boolean;
  clearPagination(): DockerContainersResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DockerContainersResponse.AsObject;
  static toObject(includeInstance: boolean, msg: DockerContainersResponse): DockerContainersResponse.AsObject;
  static serializeBinaryToWriter(message: DockerContainersResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DockerContainersResponse;
  static deserializeBinaryFromReader(message: DockerContainersResponse, reader: jspb.BinaryReader): DockerContainersResponse;
}

export namespace DockerContainersResponse {
  export type AsObject = {
    itemsList: Array<DockerContainersResponse.Item.AsObject>,
    pagination?: portal_pb.Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getCommand(): string;
    setCommand(value: string): Item;

    getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasCreatedAt(): boolean;
    clearCreatedAt(): Item;

    getId(): string;
    setId(value: string): Item;

    getImage(): string;
    setImage(value: string): Item;

    getLabels(): string;
    setLabels(value: string): Item;

    getLocalVolumes(): string;
    setLocalVolumes(value: string): Item;

    getMounts(): string;
    setMounts(value: string): Item;

    getNames(): string;
    setNames(value: string): Item;

    getNetworks(): string;
    setNetworks(value: string): Item;

    getPlatform(): string;
    setPlatform(value: string): Item;
    hasPlatform(): boolean;
    clearPlatform(): Item;

    getPorts(): string;
    setPorts(value: string): Item;

    getRunningFor(): string;
    setRunningFor(value: string): Item;

    getSize(): string;
    setSize(value: string): Item;

    getState(): string;
    setState(value: string): Item;

    getStatus(): string;
    setStatus(value: string): Item;

    getHost(): string;
    setHost(value: string): Item;

    getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasUpdatedAt(): boolean;
    clearUpdatedAt(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      command: string,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      id: string,
      image: string,
      labels: string,
      localVolumes: string,
      mounts: string,
      names: string,
      networks: string,
      platform?: string,
      ports: string,
      runningFor: string,
      size: string,
      state: string,
      status: string,
      host: string,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum PlatformCase { 
      _PLATFORM_NOT_SET = 0,
      PLATFORM = 10,
    }
  }

}

export class DockerStatisticsResponse extends jspb.Message {
  getItemsList(): Array<DockerStatisticsResponse.Item>;
  setItemsList(value: Array<DockerStatisticsResponse.Item>): DockerStatisticsResponse;
  clearItemsList(): DockerStatisticsResponse;
  addItems(value?: DockerStatisticsResponse.Item, index?: number): DockerStatisticsResponse.Item;

  getPagination(): portal_pb.Pagination | undefined;
  setPagination(value?: portal_pb.Pagination): DockerStatisticsResponse;
  hasPagination(): boolean;
  clearPagination(): DockerStatisticsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DockerStatisticsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: DockerStatisticsResponse): DockerStatisticsResponse.AsObject;
  static serializeBinaryToWriter(message: DockerStatisticsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DockerStatisticsResponse;
  static deserializeBinaryFromReader(message: DockerStatisticsResponse, reader: jspb.BinaryReader): DockerStatisticsResponse;
}

export namespace DockerStatisticsResponse {
  export type AsObject = {
    itemsList: Array<DockerStatisticsResponse.Item.AsObject>,
    pagination?: portal_pb.Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getBlockIo(): string;
    setBlockIo(value: string): Item;

    getCpuPercent(): string;
    setCpuPercent(value: string): Item;

    getContainer(): string;
    setContainer(value: string): Item;

    getId(): string;
    setId(value: string): Item;

    getMemPercent(): string;
    setMemPercent(value: string): Item;

    getMemUsage(): string;
    setMemUsage(value: string): Item;

    getName(): string;
    setName(value: string): Item;

    getNetIo(): string;
    setNetIo(value: string): Item;

    getPids(): string;
    setPids(value: string): Item;

    getHost(): string;
    setHost(value: string): Item;

    getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasCreatedAt(): boolean;
    clearCreatedAt(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      blockIo: string,
      cpuPercent: string,
      container: string,
      id: string,
      memPercent: string,
      memUsage: string,
      name: string,
      netIo: string,
      pids: string,
      host: string,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
  }

}

export class PodmanQueryRequest extends jspb.Message {
  getAll(): PodmanQueryRequest.All | undefined;
  setAll(value?: PodmanQueryRequest.All): PodmanQueryRequest;
  hasAll(): boolean;
  clearAll(): PodmanQueryRequest;

  getHost(): string;
  setHost(value: string): PodmanQueryRequest;

  getId(): string;
  setId(value: string): PodmanQueryRequest;

  getName(): string;
  setName(value: string): PodmanQueryRequest;

  getFrom(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setFrom(value?: google_protobuf_timestamp_pb.Timestamp): PodmanQueryRequest;
  hasFrom(): boolean;
  clearFrom(): PodmanQueryRequest;

  getTo(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTo(value?: google_protobuf_timestamp_pb.Timestamp): PodmanQueryRequest;
  hasTo(): boolean;
  clearTo(): PodmanQueryRequest;

  getPage(): portal_pb.Page | undefined;
  setPage(value?: portal_pb.Page): PodmanQueryRequest;
  hasPage(): boolean;
  clearPage(): PodmanQueryRequest;

  getByCase(): PodmanQueryRequest.ByCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PodmanQueryRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PodmanQueryRequest): PodmanQueryRequest.AsObject;
  static serializeBinaryToWriter(message: PodmanQueryRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PodmanQueryRequest;
  static deserializeBinaryFromReader(message: PodmanQueryRequest, reader: jspb.BinaryReader): PodmanQueryRequest;
}

export namespace PodmanQueryRequest {
  export type AsObject = {
    all?: PodmanQueryRequest.All.AsObject,
    host: string,
    id: string,
    name: string,
    from?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    to?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    page?: portal_pb.Page.AsObject,
  }

  export class All extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): All.AsObject;
    static toObject(includeInstance: boolean, msg: All): All.AsObject;
    static serializeBinaryToWriter(message: All, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): All;
    static deserializeBinaryFromReader(message: All, reader: jspb.BinaryReader): All;
  }

  export namespace All {
    export type AsObject = {
    }
  }


  export enum ByCase { 
    BY_NOT_SET = 0,
    ALL = 1,
    HOST = 2,
    ID = 3,
    NAME = 4,
  }
}

export class PodmanLogsResponse extends jspb.Message {
  getItemsList(): Array<PodmanLogsResponse.Item>;
  setItemsList(value: Array<PodmanLogsResponse.Item>): PodmanLogsResponse;
  clearItemsList(): PodmanLogsResponse;
  addItems(value?: PodmanLogsResponse.Item, index?: number): PodmanLogsResponse.Item;

  getPagination(): portal_pb.Pagination | undefined;
  setPagination(value?: portal_pb.Pagination): PodmanLogsResponse;
  hasPagination(): boolean;
  clearPagination(): PodmanLogsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PodmanLogsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PodmanLogsResponse): PodmanLogsResponse.AsObject;
  static serializeBinaryToWriter(message: PodmanLogsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PodmanLogsResponse;
  static deserializeBinaryFromReader(message: PodmanLogsResponse, reader: jspb.BinaryReader): PodmanLogsResponse;
}

export namespace PodmanLogsResponse {
  export type AsObject = {
    itemsList: Array<PodmanLogsResponse.Item.AsObject>,
    pagination?: portal_pb.Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getHost(): string;
    setHost(value: string): Item;

    getId(): string;
    setId(value: string): Item;

    getFullId(): string;
    setFullId(value: string): Item;

    getName(): string;
    setName(value: string): Item;

    getMessage(): string;
    setMessage(value: string): Item;

    getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasCreatedAt(): boolean;
    clearCreatedAt(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      host: string,
      id: string,
      fullId: string,
      name: string,
      message: string,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
  }

}

export class PodmanContainersResponse extends jspb.Message {
  getItemsList(): Array<PodmanContainersResponse.Item>;
  setItemsList(value: Array<PodmanContainersResponse.Item>): PodmanContainersResponse;
  clearItemsList(): PodmanContainersResponse;
  addItems(value?: PodmanContainersResponse.Item, index?: number): PodmanContainersResponse.Item;

  getPagination(): portal_pb.Pagination | undefined;
  setPagination(value?: portal_pb.Pagination): PodmanContainersResponse;
  hasPagination(): boolean;
  clearPagination(): PodmanContainersResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PodmanContainersResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PodmanContainersResponse): PodmanContainersResponse.AsObject;
  static serializeBinaryToWriter(message: PodmanContainersResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PodmanContainersResponse;
  static deserializeBinaryFromReader(message: PodmanContainersResponse, reader: jspb.BinaryReader): PodmanContainersResponse;
}

export namespace PodmanContainersResponse {
  export type AsObject = {
    itemsList: Array<PodmanContainersResponse.Item.AsObject>,
    pagination?: portal_pb.Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getId(): string;
    setId(value: string): Item;

    getImage(): string;
    setImage(value: string): Item;

    getImageId(): string;
    setImageId(value: string): Item;

    getLabelsMap(): jspb.Map<string, string>;
    clearLabelsMap(): Item;

    getMountsList(): Array<string>;
    setMountsList(value: Array<string>): Item;
    clearMountsList(): Item;
    addMounts(value: string, index?: number): Item;

    getNamesList(): Array<string>;
    setNamesList(value: Array<string>): Item;
    clearNamesList(): Item;
    addNames(value: string, index?: number): Item;

    getCommandList(): Array<string>;
    setCommandList(value: Array<string>): Item;
    clearCommandList(): Item;
    addCommand(value: string, index?: number): Item;

    getPid(): number;
    setPid(value: number): Item;

    getState(): string;
    setState(value: string): Item;

    getStartedAt(): number;
    setStartedAt(value: number): Item;

    getCreatedAt(): string;
    setCreatedAt(value: string): Item;

    getCreated(): number;
    setCreated(value: number): Item;

    getStatus(): string;
    setStatus(value: string): Item;

    getExited(): boolean;
    setExited(value: boolean): Item;

    getExitedAt(): number;
    setExitedAt(value: number): Item;

    getExitCode(): number;
    setExitCode(value: number): Item;

    getHost(): string;
    setHost(value: string): Item;

    getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasUpdatedAt(): boolean;
    clearUpdatedAt(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      id: string,
      image: string,
      imageId: string,
      labelsMap: Array<[string, string]>,
      mountsList: Array<string>,
      namesList: Array<string>,
      commandList: Array<string>,
      pid: number,
      state: string,
      startedAt: number,
      createdAt: string,
      created: number,
      status: string,
      exited: boolean,
      exitedAt: number,
      exitCode: number,
      host: string,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
  }

}

export class PodmanStatisticsResponse extends jspb.Message {
  getItemsList(): Array<PodmanStatisticsResponse.Item>;
  setItemsList(value: Array<PodmanStatisticsResponse.Item>): PodmanStatisticsResponse;
  clearItemsList(): PodmanStatisticsResponse;
  addItems(value?: PodmanStatisticsResponse.Item, index?: number): PodmanStatisticsResponse.Item;

  getPagination(): portal_pb.Pagination | undefined;
  setPagination(value?: portal_pb.Pagination): PodmanStatisticsResponse;
  hasPagination(): boolean;
  clearPagination(): PodmanStatisticsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PodmanStatisticsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PodmanStatisticsResponse): PodmanStatisticsResponse.AsObject;
  static serializeBinaryToWriter(message: PodmanStatisticsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PodmanStatisticsResponse;
  static deserializeBinaryFromReader(message: PodmanStatisticsResponse, reader: jspb.BinaryReader): PodmanStatisticsResponse;
}

export namespace PodmanStatisticsResponse {
  export type AsObject = {
    itemsList: Array<PodmanStatisticsResponse.Item.AsObject>,
    pagination?: portal_pb.Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getId(): string;
    setId(value: string): Item;

    getName(): string;
    setName(value: string): Item;

    getCpuTime(): string;
    setCpuTime(value: string): Item;

    getCpuPercent(): string;
    setCpuPercent(value: string): Item;

    getAvgCpu(): string;
    setAvgCpu(value: string): Item;

    getMemUsage(): string;
    setMemUsage(value: string): Item;

    getMemPercent(): string;
    setMemPercent(value: string): Item;

    getNetIo(): string;
    setNetIo(value: string): Item;

    getBlockIo(): string;
    setBlockIo(value: string): Item;

    getPids(): string;
    setPids(value: string): Item;

    getHost(): string;
    setHost(value: string): Item;

    getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasCreatedAt(): boolean;
    clearCreatedAt(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      id: string,
      name: string,
      cpuTime: string,
      cpuPercent: string,
      avgCpu: string,
      memUsage: string,
      memPercent: string,
      netIo: string,
      blockIo: string,
      pids: string,
      host: string,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
  }

}

