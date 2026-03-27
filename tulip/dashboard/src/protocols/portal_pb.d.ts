import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"


export class IdRequest extends jspb.Message {
  getId(): number;
  setId(value: number): IdRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IdRequest.AsObject;
  static toObject(includeInstance: boolean, msg: IdRequest): IdRequest.AsObject;
  static serializeBinaryToWriter(message: IdRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IdRequest;
  static deserializeBinaryFromReader(message: IdRequest, reader: jspb.BinaryReader): IdRequest;
}

export namespace IdRequest {
  export type AsObject = {
    id: number;
  };
}

export class Page extends jspb.Message {
  getIndex(): number;
  setIndex(value: number): Page;

  getSize(): number;
  setSize(value: number): Page;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Page.AsObject;
  static toObject(includeInstance: boolean, msg: Page): Page.AsObject;
  static serializeBinaryToWriter(message: Page, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Page;
  static deserializeBinaryFromReader(message: Page, reader: jspb.BinaryReader): Page;
}

export namespace Page {
  export type AsObject = {
    index: number;
    size: number;
  };
}

export class Pagination extends jspb.Message {
  getCurrent(): Page | undefined;
  setCurrent(value?: Page): Pagination;
  hasCurrent(): boolean;
  clearCurrent(): Pagination;

  getHasPrevious(): boolean;
  setHasPrevious(value: boolean): Pagination;

  getHasNext(): boolean;
  setHasNext(value: boolean): Pagination;

  getPages(): number;
  setPages(value: number): Pagination;

  getTotal(): number;
  setTotal(value: number): Pagination;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Pagination.AsObject;
  static toObject(includeInstance: boolean, msg: Pagination): Pagination.AsObject;
  static serializeBinaryToWriter(message: Pagination, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Pagination;
  static deserializeBinaryFromReader(message: Pagination, reader: jspb.BinaryReader): Pagination;
}

export namespace Pagination {
  export type AsObject = {
    current?: Page.AsObject;
    hasPrevious: boolean;
    hasNext: boolean;
    pages: number;
    total: number;
  };
}

export class File extends jspb.Message {
  getBucket(): string;
  setBucket(value: string): File;

  getObject(): string;
  setObject(value: string): File;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): File.AsObject;
  static toObject(includeInstance: boolean, msg: File): File.AsObject;
  static serializeBinaryToWriter(message: File, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): File;
  static deserializeBinaryFromReader(message: File, reader: jspb.BinaryReader): File;
}

export namespace File {
  export type AsObject = {
    bucket: string;
    object: string;
  };
}

export class RichText extends jspb.Message {
  getEditor(): RichText.Editor;
  setEditor(value: RichText.Editor): RichText;

  getBody(): string;
  setBody(value: string): RichText;

  getAttachmentsList(): Array<File>;
  setAttachmentsList(value: Array<File>): RichText;
  clearAttachmentsList(): RichText;
  addAttachments(value?: File, index?: number): File;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RichText.AsObject;
  static toObject(includeInstance: boolean, msg: RichText): RichText.AsObject;
  static serializeBinaryToWriter(message: RichText, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RichText;
  static deserializeBinaryFromReader(message: RichText, reader: jspb.BinaryReader): RichText;
}

export namespace RichText {
  export type AsObject = {
    editor: RichText.Editor;
    body: string;
    attachmentsList: Array<File.AsObject>;
  };

  export enum Editor {
    CK_EDITOR = 0,
  }
}

export class Location extends jspb.Message {
  getMap(): Location.Map;
  setMap(value: Location.Map): Location;

  getAddress(): string;
  setAddress(value: string): Location;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Location.AsObject;
  static toObject(includeInstance: boolean, msg: Location): Location.AsObject;
  static serializeBinaryToWriter(message: Location, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Location;
  static deserializeBinaryFromReader(message: Location, reader: jspb.BinaryReader): Location;
}

export namespace Location {
  export type AsObject = {
    map: Location.Map;
    address: string;
  };

  export enum Map {
    GOOGLE = 0,
  }
}

export class UserIndexResponse extends jspb.Message {
  getItemsList(): Array<UserIndexResponse.Item>;
  setItemsList(value: Array<UserIndexResponse.Item>): UserIndexResponse;
  clearItemsList(): UserIndexResponse;
  addItems(value?: UserIndexResponse.Item, index?: number): UserIndexResponse.Item;

  getPagination(): Pagination | undefined;
  setPagination(value?: Pagination): UserIndexResponse;
  hasPagination(): boolean;
  clearPagination(): UserIndexResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserIndexResponse): UserIndexResponse.AsObject;
  static serializeBinaryToWriter(message: UserIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserIndexResponse;
  static deserializeBinaryFromReader(message: UserIndexResponse, reader: jspb.BinaryReader): UserIndexResponse;
}

export namespace UserIndexResponse {
  export type AsObject = {
    itemsList: Array<UserIndexResponse.Item.AsObject>;
    pagination?: Pagination.AsObject;
  };

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getType(): UserIndexResponse.Item.Type;
    setType(value: UserIndexResponse.Item.Type): Item;

    getSn(): string;
    setSn(value: string): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      id: number;
      type: UserIndexResponse.Item.Type;
      sn: string;
    };

    export enum Type {
      EMAIL = 0,
      SMS = 1,
      GOOGLEOAUTH2 = 2,
      WECHATOAUTH2 = 3,
      WECHATMINIPROGRAM = 4,
    }
  }

}

export class Session extends jspb.Message {
  getLocale(): string;
  setLocale(value: string): Session;

  getClientIp(): string;
  setClientIp(value: string): Session;
  hasClientIp(): boolean;
  clearClientIp(): Session;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Session;
  hasCreatedAt(): boolean;
  clearCreatedAt(): Session;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Session.AsObject;
  static toObject(includeInstance: boolean, msg: Session): Session.AsObject;
  static serializeBinaryToWriter(message: Session, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Session;
  static deserializeBinaryFromReader(message: Session, reader: jspb.BinaryReader): Session;
}

export namespace Session {
  export type AsObject = {
    locale: string;
    clientIp?: string;
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
  };

  export enum ClientIpCase {
    _CLIENT_IP_NOT_SET = 0,
    CLIENT_IP = 2,
  }
}

export class SiteHeartbeatResponse extends jspb.Message {
  getDb(): string;
  setDb(value: string): SiteHeartbeatResponse;
  hasDb(): boolean;
  clearDb(): SiteHeartbeatResponse;

  getCache(): string;
  setCache(value: string): SiteHeartbeatResponse;
  hasCache(): boolean;
  clearCache(): SiteHeartbeatResponse;

  getQueue(): string;
  setQueue(value: string): SiteHeartbeatResponse;
  hasQueue(): boolean;
  clearQueue(): SiteHeartbeatResponse;

  getS3(): string;
  setS3(value: string): SiteHeartbeatResponse;
  hasS3(): boolean;
  clearS3(): SiteHeartbeatResponse;

  getVersion(): string;
  setVersion(value: string): SiteHeartbeatResponse;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): SiteHeartbeatResponse;
  hasCreatedAt(): boolean;
  clearCreatedAt(): SiteHeartbeatResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteHeartbeatResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SiteHeartbeatResponse): SiteHeartbeatResponse.AsObject;
  static serializeBinaryToWriter(message: SiteHeartbeatResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteHeartbeatResponse;
  static deserializeBinaryFromReader(message: SiteHeartbeatResponse, reader: jspb.BinaryReader): SiteHeartbeatResponse;
}

export namespace SiteHeartbeatResponse {
  export type AsObject = {
    db?: string;
    cache?: string;
    queue?: string;
    s3?: string;
    version: string;
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
  };

  export enum DbCase {
    _DB_NOT_SET = 0,
    DB = 1,
  }

  export enum CacheCase {
    _CACHE_NOT_SET = 0,
    CACHE = 2,
  }

  export enum QueueCase {
    _QUEUE_NOT_SET = 0,
    QUEUE = 3,
  }

  export enum S3Case {
    _S3_NOT_SET = 0,
    S3 = 4,
  }
}

