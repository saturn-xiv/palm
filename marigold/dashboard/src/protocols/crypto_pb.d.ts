import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"
import * as google_protobuf_duration_pb from 'google-protobuf/google/protobuf/duration_pb'; // proto import: "google/protobuf/duration.proto"


export class AeadEncryptRequest extends jspb.Message {
  getPlain(): Uint8Array | string;
  getPlain_asU8(): Uint8Array;
  getPlain_asB64(): string;
  setPlain(value: Uint8Array | string): AeadEncryptRequest;

  getAssociated(): Uint8Array | string;
  getAssociated_asU8(): Uint8Array;
  getAssociated_asB64(): string;
  setAssociated(value: Uint8Array | string): AeadEncryptRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AeadEncryptRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AeadEncryptRequest): AeadEncryptRequest.AsObject;
  static serializeBinaryToWriter(message: AeadEncryptRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AeadEncryptRequest;
  static deserializeBinaryFromReader(message: AeadEncryptRequest, reader: jspb.BinaryReader): AeadEncryptRequest;
}

export namespace AeadEncryptRequest {
  export type AsObject = {
    plain: Uint8Array | string;
    associated: Uint8Array | string;
  };
}

export class AeadEncryptResponse extends jspb.Message {
  getCipher(): Uint8Array | string;
  getCipher_asU8(): Uint8Array;
  getCipher_asB64(): string;
  setCipher(value: Uint8Array | string): AeadEncryptResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AeadEncryptResponse.AsObject;
  static toObject(includeInstance: boolean, msg: AeadEncryptResponse): AeadEncryptResponse.AsObject;
  static serializeBinaryToWriter(message: AeadEncryptResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AeadEncryptResponse;
  static deserializeBinaryFromReader(message: AeadEncryptResponse, reader: jspb.BinaryReader): AeadEncryptResponse;
}

export namespace AeadEncryptResponse {
  export type AsObject = {
    cipher: Uint8Array | string;
  };
}

export class AeadDecryptRequest extends jspb.Message {
  getCipher(): Uint8Array | string;
  getCipher_asU8(): Uint8Array;
  getCipher_asB64(): string;
  setCipher(value: Uint8Array | string): AeadDecryptRequest;

  getAssociated(): Uint8Array | string;
  getAssociated_asU8(): Uint8Array;
  getAssociated_asB64(): string;
  setAssociated(value: Uint8Array | string): AeadDecryptRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AeadDecryptRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AeadDecryptRequest): AeadDecryptRequest.AsObject;
  static serializeBinaryToWriter(message: AeadDecryptRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AeadDecryptRequest;
  static deserializeBinaryFromReader(message: AeadDecryptRequest, reader: jspb.BinaryReader): AeadDecryptRequest;
}

export namespace AeadDecryptRequest {
  export type AsObject = {
    cipher: Uint8Array | string;
    associated: Uint8Array | string;
  };
}

export class AeadDecryptResponse extends jspb.Message {
  getPlain(): Uint8Array | string;
  getPlain_asU8(): Uint8Array;
  getPlain_asB64(): string;
  setPlain(value: Uint8Array | string): AeadDecryptResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AeadDecryptResponse.AsObject;
  static toObject(includeInstance: boolean, msg: AeadDecryptResponse): AeadDecryptResponse.AsObject;
  static serializeBinaryToWriter(message: AeadDecryptResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AeadDecryptResponse;
  static deserializeBinaryFromReader(message: AeadDecryptResponse, reader: jspb.BinaryReader): AeadDecryptResponse;
}

export namespace AeadDecryptResponse {
  export type AsObject = {
    plain: Uint8Array | string;
  };
}

export class HMacComputeRequest extends jspb.Message {
  getData(): Uint8Array | string;
  getData_asU8(): Uint8Array;
  getData_asB64(): string;
  setData(value: Uint8Array | string): HMacComputeRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HMacComputeRequest.AsObject;
  static toObject(includeInstance: boolean, msg: HMacComputeRequest): HMacComputeRequest.AsObject;
  static serializeBinaryToWriter(message: HMacComputeRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HMacComputeRequest;
  static deserializeBinaryFromReader(message: HMacComputeRequest, reader: jspb.BinaryReader): HMacComputeRequest;
}

export namespace HMacComputeRequest {
  export type AsObject = {
    data: Uint8Array | string;
  };
}

export class HMacComputeResponse extends jspb.Message {
  getMac(): Uint8Array | string;
  getMac_asU8(): Uint8Array;
  getMac_asB64(): string;
  setMac(value: Uint8Array | string): HMacComputeResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HMacComputeResponse.AsObject;
  static toObject(includeInstance: boolean, msg: HMacComputeResponse): HMacComputeResponse.AsObject;
  static serializeBinaryToWriter(message: HMacComputeResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HMacComputeResponse;
  static deserializeBinaryFromReader(message: HMacComputeResponse, reader: jspb.BinaryReader): HMacComputeResponse;
}

export namespace HMacComputeResponse {
  export type AsObject = {
    mac: Uint8Array | string;
  };
}

export class HMacVerifyRequest extends jspb.Message {
  getMac(): Uint8Array | string;
  getMac_asU8(): Uint8Array;
  getMac_asB64(): string;
  setMac(value: Uint8Array | string): HMacVerifyRequest;

  getData(): Uint8Array | string;
  getData_asU8(): Uint8Array;
  getData_asB64(): string;
  setData(value: Uint8Array | string): HMacVerifyRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HMacVerifyRequest.AsObject;
  static toObject(includeInstance: boolean, msg: HMacVerifyRequest): HMacVerifyRequest.AsObject;
  static serializeBinaryToWriter(message: HMacVerifyRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HMacVerifyRequest;
  static deserializeBinaryFromReader(message: HMacVerifyRequest, reader: jspb.BinaryReader): HMacVerifyRequest;
}

export namespace HMacVerifyRequest {
  export type AsObject = {
    mac: Uint8Array | string;
    data: Uint8Array | string;
  };
}

export class JwtSignRequest extends jspb.Message {
  getIssuer(): string;
  setIssuer(value: string): JwtSignRequest;

  getSubject(): string;
  setSubject(value: string): JwtSignRequest;

  getAudiencesList(): Array<string>;
  setAudiencesList(value: Array<string>): JwtSignRequest;
  clearAudiencesList(): JwtSignRequest;
  addAudiences(value: string, index?: number): JwtSignRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): JwtSignRequest;
  hasTtl(): boolean;
  clearTtl(): JwtSignRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JwtSignRequest.AsObject;
  static toObject(includeInstance: boolean, msg: JwtSignRequest): JwtSignRequest.AsObject;
  static serializeBinaryToWriter(message: JwtSignRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JwtSignRequest;
  static deserializeBinaryFromReader(message: JwtSignRequest, reader: jspb.BinaryReader): JwtSignRequest;
}

export namespace JwtSignRequest {
  export type AsObject = {
    issuer: string;
    subject: string;
    audiencesList: Array<string>;
    ttl?: google_protobuf_duration_pb.Duration.AsObject;
  };
}

export class JwtSignResponse extends jspb.Message {
  getToken(): string;
  setToken(value: string): JwtSignResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JwtSignResponse.AsObject;
  static toObject(includeInstance: boolean, msg: JwtSignResponse): JwtSignResponse.AsObject;
  static serializeBinaryToWriter(message: JwtSignResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JwtSignResponse;
  static deserializeBinaryFromReader(message: JwtSignResponse, reader: jspb.BinaryReader): JwtSignResponse;
}

export namespace JwtSignResponse {
  export type AsObject = {
    token: string;
  };
}

export class JwtVerifyRequest extends jspb.Message {
  getToken(): string;
  setToken(value: string): JwtVerifyRequest;

  getIssuer(): string;
  setIssuer(value: string): JwtVerifyRequest;

  getAudience(): string;
  setAudience(value: string): JwtVerifyRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JwtVerifyRequest.AsObject;
  static toObject(includeInstance: boolean, msg: JwtVerifyRequest): JwtVerifyRequest.AsObject;
  static serializeBinaryToWriter(message: JwtVerifyRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JwtVerifyRequest;
  static deserializeBinaryFromReader(message: JwtVerifyRequest, reader: jspb.BinaryReader): JwtVerifyRequest;
}

export namespace JwtVerifyRequest {
  export type AsObject = {
    token: string;
    issuer: string;
    audience: string;
  };
}

export class JwtVerifyResponse extends jspb.Message {
  getId(): string;
  setId(value: string): JwtVerifyResponse;

  getSubject(): string;
  setSubject(value: string): JwtVerifyResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JwtVerifyResponse.AsObject;
  static toObject(includeInstance: boolean, msg: JwtVerifyResponse): JwtVerifyResponse.AsObject;
  static serializeBinaryToWriter(message: JwtVerifyResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JwtVerifyResponse;
  static deserializeBinaryFromReader(message: JwtVerifyResponse, reader: jspb.BinaryReader): JwtVerifyResponse;
}

export namespace JwtVerifyResponse {
  export type AsObject = {
    id: string;
    subject: string;
  };
}

