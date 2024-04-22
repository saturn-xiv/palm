import * as jspb from 'google-protobuf'

import * as google_protobuf_duration_pb from 'google-protobuf/google/protobuf/duration_pb'; // proto import: "google/protobuf/duration.proto"


export class S3CreateBucketRequest extends jspb.Message {
  getName(): string;
  setName(value: string): S3CreateBucketRequest;

  getExpirationDays(): number;
  setExpirationDays(value: number): S3CreateBucketRequest;
  hasExpirationDays(): boolean;
  clearExpirationDays(): S3CreateBucketRequest;

  getPublic(): boolean;
  setPublic(value: boolean): S3CreateBucketRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): S3CreateBucketRequest.AsObject;
  static toObject(includeInstance: boolean, msg: S3CreateBucketRequest): S3CreateBucketRequest.AsObject;
  static serializeBinaryToWriter(message: S3CreateBucketRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): S3CreateBucketRequest;
  static deserializeBinaryFromReader(message: S3CreateBucketRequest, reader: jspb.BinaryReader): S3CreateBucketRequest;
}

export namespace S3CreateBucketRequest {
  export type AsObject = {
    name: string,
    expirationDays?: number,
    pb_public: boolean,
  }

  export enum ExpirationDaysCase { 
    _EXPIRATION_DAYS_NOT_SET = 0,
    EXPIRATION_DAYS = 2,
  }
}

export class S3CreateBucketResponse extends jspb.Message {
  getName(): string;
  setName(value: string): S3CreateBucketResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): S3CreateBucketResponse.AsObject;
  static toObject(includeInstance: boolean, msg: S3CreateBucketResponse): S3CreateBucketResponse.AsObject;
  static serializeBinaryToWriter(message: S3CreateBucketResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): S3CreateBucketResponse;
  static deserializeBinaryFromReader(message: S3CreateBucketResponse, reader: jspb.BinaryReader): S3CreateBucketResponse;
}

export namespace S3CreateBucketResponse {
  export type AsObject = {
    name: string,
  }
}

export class S3UploadFileRequest extends jspb.Message {
  getBucket(): string;
  setBucket(value: string): S3UploadFileRequest;

  getObject(): string;
  setObject(value: string): S3UploadFileRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): S3UploadFileRequest;
  hasTtl(): boolean;
  clearTtl(): S3UploadFileRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): S3UploadFileRequest.AsObject;
  static toObject(includeInstance: boolean, msg: S3UploadFileRequest): S3UploadFileRequest.AsObject;
  static serializeBinaryToWriter(message: S3UploadFileRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): S3UploadFileRequest;
  static deserializeBinaryFromReader(message: S3UploadFileRequest, reader: jspb.BinaryReader): S3UploadFileRequest;
}

export namespace S3UploadFileRequest {
  export type AsObject = {
    bucket: string,
    object: string,
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class S3UploadFileResponse extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): S3UploadFileResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): S3UploadFileResponse.AsObject;
  static toObject(includeInstance: boolean, msg: S3UploadFileResponse): S3UploadFileResponse.AsObject;
  static serializeBinaryToWriter(message: S3UploadFileResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): S3UploadFileResponse;
  static deserializeBinaryFromReader(message: S3UploadFileResponse, reader: jspb.BinaryReader): S3UploadFileResponse;
}

export namespace S3UploadFileResponse {
  export type AsObject = {
    url: string,
  }
}

export class S3UploadResponse extends jspb.Message {
  getBucket(): string;
  setBucket(value: string): S3UploadResponse;

  getName(): string;
  setName(value: string): S3UploadResponse;

  getContentType(): string;
  setContentType(value: string): S3UploadResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): S3UploadResponse.AsObject;
  static toObject(includeInstance: boolean, msg: S3UploadResponse): S3UploadResponse.AsObject;
  static serializeBinaryToWriter(message: S3UploadResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): S3UploadResponse;
  static deserializeBinaryFromReader(message: S3UploadResponse, reader: jspb.BinaryReader): S3UploadResponse;
}

export namespace S3UploadResponse {
  export type AsObject = {
    bucket: string,
    name: string,
    contentType: string,
  }
}

export class S3GetPresignedUrlRequest extends jspb.Message {
  getBucket(): string;
  setBucket(value: string): S3GetPresignedUrlRequest;

  getObject(): string;
  setObject(value: string): S3GetPresignedUrlRequest;

  getTitle(): string;
  setTitle(value: string): S3GetPresignedUrlRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): S3GetPresignedUrlRequest;
  hasTtl(): boolean;
  clearTtl(): S3GetPresignedUrlRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): S3GetPresignedUrlRequest.AsObject;
  static toObject(includeInstance: boolean, msg: S3GetPresignedUrlRequest): S3GetPresignedUrlRequest.AsObject;
  static serializeBinaryToWriter(message: S3GetPresignedUrlRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): S3GetPresignedUrlRequest;
  static deserializeBinaryFromReader(message: S3GetPresignedUrlRequest, reader: jspb.BinaryReader): S3GetPresignedUrlRequest;
}

export namespace S3GetPresignedUrlRequest {
  export type AsObject = {
    bucket: string,
    object: string,
    title: string,
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class S3GetPermanentUrlRequest extends jspb.Message {
  getBucket(): string;
  setBucket(value: string): S3GetPermanentUrlRequest;

  getObject(): string;
  setObject(value: string): S3GetPermanentUrlRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): S3GetPermanentUrlRequest.AsObject;
  static toObject(includeInstance: boolean, msg: S3GetPermanentUrlRequest): S3GetPermanentUrlRequest.AsObject;
  static serializeBinaryToWriter(message: S3GetPermanentUrlRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): S3GetPermanentUrlRequest;
  static deserializeBinaryFromReader(message: S3GetPermanentUrlRequest, reader: jspb.BinaryReader): S3GetPermanentUrlRequest;
}

export namespace S3GetPermanentUrlRequest {
  export type AsObject = {
    bucket: string,
    object: string,
  }
}

export class S3UrlResponse extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): S3UrlResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): S3UrlResponse.AsObject;
  static toObject(includeInstance: boolean, msg: S3UrlResponse): S3UrlResponse.AsObject;
  static serializeBinaryToWriter(message: S3UrlResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): S3UrlResponse;
  static deserializeBinaryFromReader(message: S3UrlResponse, reader: jspb.BinaryReader): S3UrlResponse;
}

export namespace S3UrlResponse {
  export type AsObject = {
    url: string,
  }
}

