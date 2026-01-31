import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"
import * as google_protobuf_duration_pb from 'google-protobuf/google/protobuf/duration_pb'; // proto import: "google/protobuf/duration.proto"


export class RemoveObjectRequest extends jspb.Message {
  getBucket(): string;
  setBucket(value: string): RemoveObjectRequest;

  getObject(): string;
  setObject(value: string): RemoveObjectRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RemoveObjectRequest.AsObject;
  static toObject(includeInstance: boolean, msg: RemoveObjectRequest): RemoveObjectRequest.AsObject;
  static serializeBinaryToWriter(message: RemoveObjectRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RemoveObjectRequest;
  static deserializeBinaryFromReader(message: RemoveObjectRequest, reader: jspb.BinaryReader): RemoveObjectRequest;
}

export namespace RemoveObjectRequest {
  export type AsObject = {
    bucket: string;
    object: string;
  };
}

export class MakeBucketRequest extends jspb.Message {
  getName(): string;
  setName(value: string): MakeBucketRequest;

  getPublic(): boolean;
  setPublic(value: boolean): MakeBucketRequest;

  getExpireAfterDays(): number;
  setExpireAfterDays(value: number): MakeBucketRequest;
  hasExpireAfterDays(): boolean;
  clearExpireAfterDays(): MakeBucketRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MakeBucketRequest.AsObject;
  static toObject(includeInstance: boolean, msg: MakeBucketRequest): MakeBucketRequest.AsObject;
  static serializeBinaryToWriter(message: MakeBucketRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MakeBucketRequest;
  static deserializeBinaryFromReader(message: MakeBucketRequest, reader: jspb.BinaryReader): MakeBucketRequest;
}

export namespace MakeBucketRequest {
  export type AsObject = {
    name: string;
    pb_public: boolean;
    expireAfterDays?: number;
  };

  export enum ExpireAfterDaysCase {
    _EXPIRE_AFTER_DAYS_NOT_SET = 0,
    EXPIRE_AFTER_DAYS = 3,
  }
}

export class BucketExistsRequest extends jspb.Message {
  getName(): string;
  setName(value: string): BucketExistsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BucketExistsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: BucketExistsRequest): BucketExistsRequest.AsObject;
  static serializeBinaryToWriter(message: BucketExistsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BucketExistsRequest;
  static deserializeBinaryFromReader(message: BucketExistsRequest, reader: jspb.BinaryReader): BucketExistsRequest;
}

export namespace BucketExistsRequest {
  export type AsObject = {
    name: string;
  };
}

export class BucketExistsResponse extends jspb.Message {
  getExists(): boolean;
  setExists(value: boolean): BucketExistsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BucketExistsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: BucketExistsResponse): BucketExistsResponse.AsObject;
  static serializeBinaryToWriter(message: BucketExistsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BucketExistsResponse;
  static deserializeBinaryFromReader(message: BucketExistsResponse, reader: jspb.BinaryReader): BucketExistsResponse;
}

export namespace BucketExistsResponse {
  export type AsObject = {
    exists: boolean;
  };
}

export class PutObjectRequest extends jspb.Message {
  getBucket(): string;
  setBucket(value: string): PutObjectRequest;

  getObject(): string;
  setObject(value: string): PutObjectRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): PutObjectRequest;
  hasTtl(): boolean;
  clearTtl(): PutObjectRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PutObjectRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PutObjectRequest): PutObjectRequest.AsObject;
  static serializeBinaryToWriter(message: PutObjectRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PutObjectRequest;
  static deserializeBinaryFromReader(message: PutObjectRequest, reader: jspb.BinaryReader): PutObjectRequest;
}

export namespace PutObjectRequest {
  export type AsObject = {
    bucket: string;
    object: string;
    ttl?: google_protobuf_duration_pb.Duration.AsObject;
  };
}

export class PutObjectResponse extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): PutObjectResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PutObjectResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PutObjectResponse): PutObjectResponse.AsObject;
  static serializeBinaryToWriter(message: PutObjectResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PutObjectResponse;
  static deserializeBinaryFromReader(message: PutObjectResponse, reader: jspb.BinaryReader): PutObjectResponse;
}

export namespace PutObjectResponse {
  export type AsObject = {
    url: string;
  };
}

export class ListBucketResponse extends jspb.Message {
  getItemsList(): Array<ListBucketResponse.Item>;
  setItemsList(value: Array<ListBucketResponse.Item>): ListBucketResponse;
  clearItemsList(): ListBucketResponse;
  addItems(value?: ListBucketResponse.Item, index?: number): ListBucketResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListBucketResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListBucketResponse): ListBucketResponse.AsObject;
  static serializeBinaryToWriter(message: ListBucketResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListBucketResponse;
  static deserializeBinaryFromReader(message: ListBucketResponse, reader: jspb.BinaryReader): ListBucketResponse;
}

export namespace ListBucketResponse {
  export type AsObject = {
    itemsList: Array<ListBucketResponse.Item.AsObject>;
  };

  export class Item extends jspb.Message {
    getName(): string;
    setName(value: string): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      name: string;
    };
  }

}

