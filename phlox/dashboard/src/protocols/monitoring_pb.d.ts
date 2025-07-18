import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"
import * as portal_pb from './portal_pb'; // proto import: "portal.proto"


export class FileSystemQueryRequest extends jspb.Message {
  getAll(): FileSystemQueryRequest.All | undefined;
  setAll(value?: FileSystemQueryRequest.All): FileSystemQueryRequest;
  hasAll(): boolean;
  clearAll(): FileSystemQueryRequest;

  getHost(): string;
  setHost(value: string): FileSystemQueryRequest;

  getFile(): string;
  setFile(value: string): FileSystemQueryRequest;

  getPage(): portal_pb.Page | undefined;
  setPage(value?: portal_pb.Page): FileSystemQueryRequest;
  hasPage(): boolean;
  clearPage(): FileSystemQueryRequest;

  getByCase(): FileSystemQueryRequest.ByCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FileSystemQueryRequest.AsObject;
  static toObject(includeInstance: boolean, msg: FileSystemQueryRequest): FileSystemQueryRequest.AsObject;
  static serializeBinaryToWriter(message: FileSystemQueryRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FileSystemQueryRequest;
  static deserializeBinaryFromReader(message: FileSystemQueryRequest, reader: jspb.BinaryReader): FileSystemQueryRequest;
}

export namespace FileSystemQueryRequest {
  export type AsObject = {
    all?: FileSystemQueryRequest.All.AsObject,
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

export class FileLogsResponse extends jspb.Message {
  getItemsList(): Array<FileLogsResponse.Item>;
  setItemsList(value: Array<FileLogsResponse.Item>): FileLogsResponse;
  clearItemsList(): FileLogsResponse;
  addItems(value?: FileLogsResponse.Item, index?: number): FileLogsResponse.Item;

  getPagination(): portal_pb.Pagination | undefined;
  setPagination(value?: portal_pb.Pagination): FileLogsResponse;
  hasPagination(): boolean;
  clearPagination(): FileLogsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FileLogsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: FileLogsResponse): FileLogsResponse.AsObject;
  static serializeBinaryToWriter(message: FileLogsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FileLogsResponse;
  static deserializeBinaryFromReader(message: FileLogsResponse, reader: jspb.BinaryReader): FileLogsResponse;
}

export namespace FileLogsResponse {
  export type AsObject = {
    itemsList: Array<FileLogsResponse.Item.AsObject>,
    pagination?: portal_pb.Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getHost(): string;
    setHost(value: string): Item;

    getFile(): string;
    setFile(value: string): Item;

    getMessage(): string;
    setMessage(value: string): Item;

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
      message: string,
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

  getContainerId(): string;
  setContainerId(value: string): PodmanQueryRequest;

  getContainerName(): string;
  setContainerName(value: string): PodmanQueryRequest;

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
    containerId: string,
    containerName: string,
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
    CONTAINER_ID = 3,
    CONTAINER_NAME = 4,
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

    getContainerId(): string;
    setContainerId(value: string): Item;

    getMessage(): string;
    setMessage(value: string): Item;

    getCreatedAt(): string;
    setCreatedAt(value: string): Item;

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
      containerId: string,
      message: string,
      createdAt: string,
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

    getExited(): boolean;
    setExited(value: boolean): Item;

    getExitedAt(): number;
    setExitedAt(value: number): Item;

    getExitCode(): number;
    setExitCode(value: number): Item;

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
      image: string,
      imageId: string,
      labelsMap: Array<[string, string]>,
      mountsList: Array<string>,
      namesList: Array<string>,
      commandList: Array<string>,
      pid: number,
      state: string,
      startedAt: number,
      exited: boolean,
      exitedAt: number,
      exitCode: number,
      host: string,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
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

