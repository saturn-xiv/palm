import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"
import * as google_protobuf_duration_pb from 'google-protobuf/google/protobuf/duration_pb'; // proto import: "google/protobuf/duration.proto"
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"


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
    index: number,
    size: number,
  }
}

export class Pagination extends jspb.Message {
  getTotal(): number;
  setTotal(value: number): Pagination;

  getIndex(): number;
  setIndex(value: number): Pagination;

  getSize(): number;
  setSize(value: number): Pagination;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Pagination.AsObject;
  static toObject(includeInstance: boolean, msg: Pagination): Pagination.AsObject;
  static serializeBinaryToWriter(message: Pagination, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Pagination;
  static deserializeBinaryFromReader(message: Pagination, reader: jspb.BinaryReader): Pagination;
}

export namespace Pagination {
  export type AsObject = {
    total: number,
    index: number,
    size: number,
  }
}

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
    id: number,
  }
}

export class LocaleByLangRequest extends jspb.Message {
  getLang(): string;
  setLang(value: string): LocaleByLangRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LocaleByLangRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LocaleByLangRequest): LocaleByLangRequest.AsObject;
  static serializeBinaryToWriter(message: LocaleByLangRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LocaleByLangRequest;
  static deserializeBinaryFromReader(message: LocaleByLangRequest, reader: jspb.BinaryReader): LocaleByLangRequest;
}

export namespace LocaleByLangRequest {
  export type AsObject = {
    lang: string,
  }
}

export class LocaleByLangResponse extends jspb.Message {
  getItemsList(): Array<LocaleIndexResponse.Item>;
  setItemsList(value: Array<LocaleIndexResponse.Item>): LocaleByLangResponse;
  clearItemsList(): LocaleByLangResponse;
  addItems(value?: LocaleIndexResponse.Item, index?: number): LocaleIndexResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LocaleByLangResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LocaleByLangResponse): LocaleByLangResponse.AsObject;
  static serializeBinaryToWriter(message: LocaleByLangResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LocaleByLangResponse;
  static deserializeBinaryFromReader(message: LocaleByLangResponse, reader: jspb.BinaryReader): LocaleByLangResponse;
}

export namespace LocaleByLangResponse {
  export type AsObject = {
    itemsList: Array<LocaleIndexResponse.Item.AsObject>,
  }
}

export class LocaleUpdateRequest extends jspb.Message {
  getId(): number;
  setId(value: number): LocaleUpdateRequest;

  getMessage(): string;
  setMessage(value: string): LocaleUpdateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LocaleUpdateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LocaleUpdateRequest): LocaleUpdateRequest.AsObject;
  static serializeBinaryToWriter(message: LocaleUpdateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LocaleUpdateRequest;
  static deserializeBinaryFromReader(message: LocaleUpdateRequest, reader: jspb.BinaryReader): LocaleUpdateRequest;
}

export namespace LocaleUpdateRequest {
  export type AsObject = {
    id: number,
    message: string,
  }
}

export class LocaleCreateRequest extends jspb.Message {
  getLang(): string;
  setLang(value: string): LocaleCreateRequest;

  getCode(): string;
  setCode(value: string): LocaleCreateRequest;

  getMessage(): string;
  setMessage(value: string): LocaleCreateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LocaleCreateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LocaleCreateRequest): LocaleCreateRequest.AsObject;
  static serializeBinaryToWriter(message: LocaleCreateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LocaleCreateRequest;
  static deserializeBinaryFromReader(message: LocaleCreateRequest, reader: jspb.BinaryReader): LocaleCreateRequest;
}

export namespace LocaleCreateRequest {
  export type AsObject = {
    lang: string,
    code: string,
    message: string,
  }
}

export class LocaleIndexResponse extends jspb.Message {
  getItemsList(): Array<LocaleIndexResponse.Item>;
  setItemsList(value: Array<LocaleIndexResponse.Item>): LocaleIndexResponse;
  clearItemsList(): LocaleIndexResponse;
  addItems(value?: LocaleIndexResponse.Item, index?: number): LocaleIndexResponse.Item;

  getPagination(): Pagination | undefined;
  setPagination(value?: Pagination): LocaleIndexResponse;
  hasPagination(): boolean;
  clearPagination(): LocaleIndexResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LocaleIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LocaleIndexResponse): LocaleIndexResponse.AsObject;
  static serializeBinaryToWriter(message: LocaleIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LocaleIndexResponse;
  static deserializeBinaryFromReader(message: LocaleIndexResponse, reader: jspb.BinaryReader): LocaleIndexResponse;
}

export namespace LocaleIndexResponse {
  export type AsObject = {
    itemsList: Array<LocaleIndexResponse.Item.AsObject>,
    pagination?: Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getLang(): string;
    setLang(value: string): Item;

    getCode(): string;
    setCode(value: string): Item;

    getMessage(): string;
    setMessage(value: string): Item;

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
      id: number,
      lang: string,
      code: string,
      message: string,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
  }

}

export class Resource extends jspb.Message {
  getType(): string;
  setType(value: string): Resource;

  getId(): number;
  setId(value: number): Resource;
  hasId(): boolean;
  clearId(): Resource;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Resource.AsObject;
  static toObject(includeInstance: boolean, msg: Resource): Resource.AsObject;
  static serializeBinaryToWriter(message: Resource, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Resource;
  static deserializeBinaryFromReader(message: Resource, reader: jspb.BinaryReader): Resource;
}

export namespace Resource {
  export type AsObject = {
    type: string,
    id?: number,
  }

  export enum IdCase { 
    _ID_NOT_SET = 0,
    ID = 2,
  }
}

export class EmailUserIndexResponse extends jspb.Message {
  getItemsList(): Array<EmailUserIndexResponse.Item>;
  setItemsList(value: Array<EmailUserIndexResponse.Item>): EmailUserIndexResponse;
  clearItemsList(): EmailUserIndexResponse;
  addItems(value?: EmailUserIndexResponse.Item, index?: number): EmailUserIndexResponse.Item;

  getPagination(): Pagination | undefined;
  setPagination(value?: Pagination): EmailUserIndexResponse;
  hasPagination(): boolean;
  clearPagination(): EmailUserIndexResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserIndexResponse): EmailUserIndexResponse.AsObject;
  static serializeBinaryToWriter(message: EmailUserIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserIndexResponse;
  static deserializeBinaryFromReader(message: EmailUserIndexResponse, reader: jspb.BinaryReader): EmailUserIndexResponse;
}

export namespace EmailUserIndexResponse {
  export type AsObject = {
    itemsList: Array<EmailUserIndexResponse.Item.AsObject>,
    pagination?: Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getRealName(): string;
    setRealName(value: string): Item;

    getAvatar(): string;
    setAvatar(value: string): Item;

    getConfirmedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setConfirmedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasConfirmedAt(): boolean;
    clearConfirmedAt(): Item;

    getDeletedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setDeletedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasDeletedAt(): boolean;
    clearDeletedAt(): Item;

    getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasUpdatedAt(): boolean;
    clearUpdatedAt(): Item;

    getUser(): UserIndexResponse.Item | undefined;
    setUser(value?: UserIndexResponse.Item): Item;
    hasUser(): boolean;
    clearUser(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      id: number,
      realName: string,
      avatar: string,
      confirmedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      user?: UserIndexResponse.Item.AsObject,
    }

    export enum ConfirmedAtCase { 
      _CONFIRMED_AT_NOT_SET = 0,
      CONFIRMED_AT = 4,
    }

    export enum DeletedAtCase { 
      _DELETED_AT_NOT_SET = 0,
      DELETED_AT = 5,
    }
  }

}

export class EmailUserDeleteByEmailRequest extends jspb.Message {
  getReason(): string;
  setReason(value: string): EmailUserDeleteByEmailRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserDeleteByEmailRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserDeleteByEmailRequest): EmailUserDeleteByEmailRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserDeleteByEmailRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserDeleteByEmailRequest;
  static deserializeBinaryFromReader(message: EmailUserDeleteByEmailRequest, reader: jspb.BinaryReader): EmailUserDeleteByEmailRequest;
}

export namespace EmailUserDeleteByEmailRequest {
  export type AsObject = {
    reason: string,
  }
}

export class EmailUserUploadAvatarResponse extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): EmailUserUploadAvatarResponse;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): EmailUserUploadAvatarResponse;
  hasTtl(): boolean;
  clearTtl(): EmailUserUploadAvatarResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserUploadAvatarResponse.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserUploadAvatarResponse): EmailUserUploadAvatarResponse.AsObject;
  static serializeBinaryToWriter(message: EmailUserUploadAvatarResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserUploadAvatarResponse;
  static deserializeBinaryFromReader(message: EmailUserUploadAvatarResponse, reader: jspb.BinaryReader): EmailUserUploadAvatarResponse;
}

export namespace EmailUserUploadAvatarResponse {
  export type AsObject = {
    url: string,
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class EmailUserSetAvatarRequest extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): EmailUserSetAvatarRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserSetAvatarRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserSetAvatarRequest): EmailUserSetAvatarRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserSetAvatarRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserSetAvatarRequest;
  static deserializeBinaryFromReader(message: EmailUserSetAvatarRequest, reader: jspb.BinaryReader): EmailUserSetAvatarRequest;
}

export namespace EmailUserSetAvatarRequest {
  export type AsObject = {
    url: string,
  }
}

export class EmailUserSetRealNameRequest extends jspb.Message {
  getName(): string;
  setName(value: string): EmailUserSetRealNameRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserSetRealNameRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserSetRealNameRequest): EmailUserSetRealNameRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserSetRealNameRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserSetRealNameRequest;
  static deserializeBinaryFromReader(message: EmailUserSetRealNameRequest, reader: jspb.BinaryReader): EmailUserSetRealNameRequest;
}

export namespace EmailUserSetRealNameRequest {
  export type AsObject = {
    name: string,
  }
}

export class EmailUserSetPasswordRequest extends jspb.Message {
  getId(): number;
  setId(value: number): EmailUserSetPasswordRequest;

  getPassword(): string;
  setPassword(value: string): EmailUserSetPasswordRequest;

  getSalt(): string;
  setSalt(value: string): EmailUserSetPasswordRequest;

  getReason(): string;
  setReason(value: string): EmailUserSetPasswordRequest;
  hasReason(): boolean;
  clearReason(): EmailUserSetPasswordRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserSetPasswordRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserSetPasswordRequest): EmailUserSetPasswordRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserSetPasswordRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserSetPasswordRequest;
  static deserializeBinaryFromReader(message: EmailUserSetPasswordRequest, reader: jspb.BinaryReader): EmailUserSetPasswordRequest;
}

export namespace EmailUserSetPasswordRequest {
  export type AsObject = {
    id: number,
    password: string,
    salt: string,
    reason?: string,
  }

  export enum ReasonCase { 
    _REASON_NOT_SET = 0,
    REASON = 4,
  }
}

export class EmailUserChangePasswordRequest extends jspb.Message {
  getCurrentPassword(): string;
  setCurrentPassword(value: string): EmailUserChangePasswordRequest;

  getNewPassword(): string;
  setNewPassword(value: string): EmailUserChangePasswordRequest;

  getSalt(): string;
  setSalt(value: string): EmailUserChangePasswordRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserChangePasswordRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserChangePasswordRequest): EmailUserChangePasswordRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserChangePasswordRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserChangePasswordRequest;
  static deserializeBinaryFromReader(message: EmailUserChangePasswordRequest, reader: jspb.BinaryReader): EmailUserChangePasswordRequest;
}

export namespace EmailUserChangePasswordRequest {
  export type AsObject = {
    currentPassword: string,
    newPassword: string,
    salt: string,
  }
}

export class EmailUserResetPasswordRequest extends jspb.Message {
  getToken(): string;
  setToken(value: string): EmailUserResetPasswordRequest;

  getPassword(): string;
  setPassword(value: string): EmailUserResetPasswordRequest;

  getSalt(): string;
  setSalt(value: string): EmailUserResetPasswordRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserResetPasswordRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserResetPasswordRequest): EmailUserResetPasswordRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserResetPasswordRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserResetPasswordRequest;
  static deserializeBinaryFromReader(message: EmailUserResetPasswordRequest, reader: jspb.BinaryReader): EmailUserResetPasswordRequest;
}

export namespace EmailUserResetPasswordRequest {
  export type AsObject = {
    token: string,
    password: string,
    salt: string,
  }
}

export class EmailUserByTokenRequest extends jspb.Message {
  getToken(): string;
  setToken(value: string): EmailUserByTokenRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserByTokenRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserByTokenRequest): EmailUserByTokenRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserByTokenRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserByTokenRequest;
  static deserializeBinaryFromReader(message: EmailUserByTokenRequest, reader: jspb.BinaryReader): EmailUserByTokenRequest;
}

export namespace EmailUserByTokenRequest {
  export type AsObject = {
    token: string,
  }
}

export class EmailUserRequest extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): EmailUserRequest;

  getHome(): string;
  setHome(value: string): EmailUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserRequest): EmailUserRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserRequest;
  static deserializeBinaryFromReader(message: EmailUserRequest, reader: jspb.BinaryReader): EmailUserRequest;
}

export namespace EmailUserRequest {
  export type AsObject = {
    email: string,
    home: string,
  }
}

export class EmailUserSignUpRequest extends jspb.Message {
  getRealName(): string;
  setRealName(value: string): EmailUserSignUpRequest;

  getEmail(): string;
  setEmail(value: string): EmailUserSignUpRequest;

  getPassword(): string;
  setPassword(value: string): EmailUserSignUpRequest;

  getSalt(): string;
  setSalt(value: string): EmailUserSignUpRequest;

  getLang(): string;
  setLang(value: string): EmailUserSignUpRequest;

  getTimezone(): string;
  setTimezone(value: string): EmailUserSignUpRequest;

  getHome(): string;
  setHome(value: string): EmailUserSignUpRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserSignUpRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserSignUpRequest): EmailUserSignUpRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserSignUpRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserSignUpRequest;
  static deserializeBinaryFromReader(message: EmailUserSignUpRequest, reader: jspb.BinaryReader): EmailUserSignUpRequest;
}

export namespace EmailUserSignUpRequest {
  export type AsObject = {
    realName: string,
    email: string,
    password: string,
    salt: string,
    lang: string,
    timezone: string,
    home: string,
  }
}

export class EmailUserSignInRequest extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): EmailUserSignInRequest;

  getPassword(): string;
  setPassword(value: string): EmailUserSignInRequest;

  getSalt(): string;
  setSalt(value: string): EmailUserSignInRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): EmailUserSignInRequest;
  hasTtl(): boolean;
  clearTtl(): EmailUserSignInRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserSignInRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserSignInRequest): EmailUserSignInRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserSignInRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserSignInRequest;
  static deserializeBinaryFromReader(message: EmailUserSignInRequest, reader: jspb.BinaryReader): EmailUserSignInRequest;
}

export namespace EmailUserSignInRequest {
  export type AsObject = {
    email: string,
    password: string,
    salt: string,
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }

  export enum TtlCase { 
    _TTL_NOT_SET = 0,
    TTL = 9,
  }
}

export class UserUploadRequest extends jspb.Message {
  getPublic(): boolean;
  setPublic(value: boolean): UserUploadRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): UserUploadRequest;
  hasTtl(): boolean;
  clearTtl(): UserUploadRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserUploadRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserUploadRequest): UserUploadRequest.AsObject;
  static serializeBinaryToWriter(message: UserUploadRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserUploadRequest;
  static deserializeBinaryFromReader(message: UserUploadRequest, reader: jspb.BinaryReader): UserUploadRequest;
}

export namespace UserUploadRequest {
  export type AsObject = {
    pb_public: boolean,
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class UserUploadResponse extends jspb.Message {
  getBucket(): string;
  setBucket(value: string): UserUploadResponse;

  getObject(): string;
  setObject(value: string): UserUploadResponse;

  getUrl(): string;
  setUrl(value: string): UserUploadResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserUploadResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserUploadResponse): UserUploadResponse.AsObject;
  static serializeBinaryToWriter(message: UserUploadResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserUploadResponse;
  static deserializeBinaryFromReader(message: UserUploadResponse, reader: jspb.BinaryReader): UserUploadResponse;
}

export namespace UserUploadResponse {
  export type AsObject = {
    bucket: string,
    object: string,
    url: string,
  }
}

export class UserSetVRequest extends jspb.Message {
  getKey(): string;
  setKey(value: string): UserSetVRequest;

  getValue(): Uint8Array | string;
  getValue_asU8(): Uint8Array;
  getValue_asB64(): string;
  setValue(value: Uint8Array | string): UserSetVRequest;

  getEncrypt(): boolean;
  setEncrypt(value: boolean): UserSetVRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSetVRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserSetVRequest): UserSetVRequest.AsObject;
  static serializeBinaryToWriter(message: UserSetVRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSetVRequest;
  static deserializeBinaryFromReader(message: UserSetVRequest, reader: jspb.BinaryReader): UserSetVRequest;
}

export namespace UserSetVRequest {
  export type AsObject = {
    key: string,
    value: Uint8Array | string,
    encrypt: boolean,
  }
}

export class UserGetVRequest extends jspb.Message {
  getKey(): string;
  setKey(value: string): UserGetVRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserGetVRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserGetVRequest): UserGetVRequest.AsObject;
  static serializeBinaryToWriter(message: UserGetVRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserGetVRequest;
  static deserializeBinaryFromReader(message: UserGetVRequest, reader: jspb.BinaryReader): UserGetVRequest;
}

export namespace UserGetVRequest {
  export type AsObject = {
    key: string,
  }
}

export class UserGetVResponse extends jspb.Message {
  getValue(): Uint8Array | string;
  getValue_asU8(): Uint8Array;
  getValue_asB64(): string;
  setValue(value: Uint8Array | string): UserGetVResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserGetVResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserGetVResponse): UserGetVResponse.AsObject;
  static serializeBinaryToWriter(message: UserGetVResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserGetVResponse;
  static deserializeBinaryFromReader(message: UserGetVResponse, reader: jspb.BinaryReader): UserGetVResponse;
}

export namespace UserGetVResponse {
  export type AsObject = {
    value: Uint8Array | string,
  }
}

export class UserSetLocationRequest extends jspb.Message {
  getTimezone(): string;
  setTimezone(value: string): UserSetLocationRequest;

  getLang(): string;
  setLang(value: string): UserSetLocationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSetLocationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserSetLocationRequest): UserSetLocationRequest.AsObject;
  static serializeBinaryToWriter(message: UserSetLocationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSetLocationRequest;
  static deserializeBinaryFromReader(message: UserSetLocationRequest, reader: jspb.BinaryReader): UserSetLocationRequest;
}

export namespace UserSetLocationRequest {
  export type AsObject = {
    timezone: string,
    lang: string,
  }
}

export class SetupUserRequest extends jspb.Message {
  getId(): number;
  setId(value: number): SetupUserRequest;

  getReason(): string;
  setReason(value: string): SetupUserRequest;
  hasReason(): boolean;
  clearReason(): SetupUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetupUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SetupUserRequest): SetupUserRequest.AsObject;
  static serializeBinaryToWriter(message: SetupUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetupUserRequest;
  static deserializeBinaryFromReader(message: SetupUserRequest, reader: jspb.BinaryReader): SetupUserRequest;
}

export namespace SetupUserRequest {
  export type AsObject = {
    id: number,
    reason?: string,
  }

  export enum ReasonCase { 
    _REASON_NOT_SET = 0,
    REASON = 2,
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
    itemsList: Array<UserIndexResponse.Item.AsObject>,
    pagination?: Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getUid(): string;
    setUid(value: string): Item;

    getName(): string;
    setName(value: string): Item;

    getLang(): string;
    setLang(value: string): Item;

    getTimezone(): string;
    setTimezone(value: string): Item;

    getSignInCount(): number;
    setSignInCount(value: number): Item;

    getCurrentSignInIp(): string;
    setCurrentSignInIp(value: string): Item;
    hasCurrentSignInIp(): boolean;
    clearCurrentSignInIp(): Item;

    getCurrentSignInAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setCurrentSignInAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasCurrentSignInAt(): boolean;
    clearCurrentSignInAt(): Item;

    getLastSignInIp(): string;
    setLastSignInIp(value: string): Item;
    hasLastSignInIp(): boolean;
    clearLastSignInIp(): Item;

    getLastSignInAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setLastSignInAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasLastSignInAt(): boolean;
    clearLastSignInAt(): Item;

    getLockedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setLockedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasLockedAt(): boolean;
    clearLockedAt(): Item;

    getDeletedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setDeletedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasDeletedAt(): boolean;
    clearDeletedAt(): Item;

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
      id: number,
      uid: string,
      name: string,
      lang: string,
      timezone: string,
      signInCount: number,
      currentSignInIp?: string,
      currentSignInAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      lastSignInIp?: string,
      lastSignInAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      lockedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum Type { 
      EMAIL = 0,
      WECHATMINIPROGRAM = 1,
      WECHATOAUTH2 = 2,
      GOOGLEOAUTH2 = 3,
      FACEBOOKOAUTH2 = 4,
    }

    export enum CurrentSignInIpCase { 
      _CURRENT_SIGN_IN_IP_NOT_SET = 0,
      CURRENT_SIGN_IN_IP = 7,
    }

    export enum CurrentSignInAtCase { 
      _CURRENT_SIGN_IN_AT_NOT_SET = 0,
      CURRENT_SIGN_IN_AT = 8,
    }

    export enum LastSignInIpCase { 
      _LAST_SIGN_IN_IP_NOT_SET = 0,
      LAST_SIGN_IN_IP = 9,
    }

    export enum LastSignInAtCase { 
      _LAST_SIGN_IN_AT_NOT_SET = 0,
      LAST_SIGN_IN_AT = 10,
    }

    export enum LockedAtCase { 
      _LOCKED_AT_NOT_SET = 0,
      LOCKED_AT = 11,
    }

    export enum DeletedAtCase { 
      _DELETED_AT_NOT_SET = 0,
      DELETED_AT = 12,
    }
  }

}

export class UserLogsResponse extends jspb.Message {
  getItemsList(): Array<UserLogsResponse.Item>;
  setItemsList(value: Array<UserLogsResponse.Item>): UserLogsResponse;
  clearItemsList(): UserLogsResponse;
  addItems(value?: UserLogsResponse.Item, index?: number): UserLogsResponse.Item;

  getPagination(): Pagination | undefined;
  setPagination(value?: Pagination): UserLogsResponse;
  hasPagination(): boolean;
  clearPagination(): UserLogsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserLogsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserLogsResponse): UserLogsResponse.AsObject;
  static serializeBinaryToWriter(message: UserLogsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserLogsResponse;
  static deserializeBinaryFromReader(message: UserLogsResponse, reader: jspb.BinaryReader): UserLogsResponse;
}

export namespace UserLogsResponse {
  export type AsObject = {
    itemsList: Array<UserLogsResponse.Item.AsObject>,
    pagination?: Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getPlugin(): string;
    setPlugin(value: string): Item;

    getIp(): string;
    setIp(value: string): Item;

    getLevel(): UserLogsResponse.Item.Level;
    setLevel(value: UserLogsResponse.Item.Level): Item;

    getResource(): Resource | undefined;
    setResource(value?: Resource): Item;
    hasResource(): boolean;
    clearResource(): Item;

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
      id: number,
      plugin: string,
      ip: string,
      level: UserLogsResponse.Item.Level,
      resource?: Resource.AsObject,
      message: string,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum Level { 
      DEBUG = 0,
      INFO = 1,
      WARNING = 2,
      ERROR = 3,
    }
  }

}

export class UserSignInResponse extends jspb.Message {
  getToken(): string;
  setToken(value: string): UserSignInResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSignInResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserSignInResponse): UserSignInResponse.AsObject;
  static serializeBinaryToWriter(message: UserSignInResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSignInResponse;
  static deserializeBinaryFromReader(message: UserSignInResponse, reader: jspb.BinaryReader): UserSignInResponse;
}

export namespace UserSignInResponse {
  export type AsObject = {
    token: string,
  }
}

export class SiteSetMaintenanceModeRequest extends jspb.Message {
  getOn(): boolean;
  setOn(value: boolean): SiteSetMaintenanceModeRequest;

  getReason(): string;
  setReason(value: string): SiteSetMaintenanceModeRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteSetMaintenanceModeRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SiteSetMaintenanceModeRequest): SiteSetMaintenanceModeRequest.AsObject;
  static serializeBinaryToWriter(message: SiteSetMaintenanceModeRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteSetMaintenanceModeRequest;
  static deserializeBinaryFromReader(message: SiteSetMaintenanceModeRequest, reader: jspb.BinaryReader): SiteSetMaintenanceModeRequest;
}

export namespace SiteSetMaintenanceModeRequest {
  export type AsObject = {
    on: boolean,
    reason: string,
  }
}

export class SiteUploadFaviconResponse extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): SiteUploadFaviconResponse;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): SiteUploadFaviconResponse;
  hasTtl(): boolean;
  clearTtl(): SiteUploadFaviconResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteUploadFaviconResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SiteUploadFaviconResponse): SiteUploadFaviconResponse.AsObject;
  static serializeBinaryToWriter(message: SiteUploadFaviconResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteUploadFaviconResponse;
  static deserializeBinaryFromReader(message: SiteUploadFaviconResponse, reader: jspb.BinaryReader): SiteUploadFaviconResponse;
}

export namespace SiteUploadFaviconResponse {
  export type AsObject = {
    url: string,
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class SiteFaviconProfile extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): SiteFaviconProfile;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteFaviconProfile.AsObject;
  static toObject(includeInstance: boolean, msg: SiteFaviconProfile): SiteFaviconProfile.AsObject;
  static serializeBinaryToWriter(message: SiteFaviconProfile, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteFaviconProfile;
  static deserializeBinaryFromReader(message: SiteFaviconProfile, reader: jspb.BinaryReader): SiteFaviconProfile;
}

export namespace SiteFaviconProfile {
  export type AsObject = {
    url: string,
  }
}

export class SiteAuthorProfile extends jspb.Message {
  getName(): string;
  setName(value: string): SiteAuthorProfile;

  getEmail(): string;
  setEmail(value: string): SiteAuthorProfile;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteAuthorProfile.AsObject;
  static toObject(includeInstance: boolean, msg: SiteAuthorProfile): SiteAuthorProfile.AsObject;
  static serializeBinaryToWriter(message: SiteAuthorProfile, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteAuthorProfile;
  static deserializeBinaryFromReader(message: SiteAuthorProfile, reader: jspb.BinaryReader): SiteAuthorProfile;
}

export namespace SiteAuthorProfile {
  export type AsObject = {
    name: string,
    email: string,
  }
}

export class GetSiteInfoByLangRequest extends jspb.Message {
  getLang(): string;
  setLang(value: string): GetSiteInfoByLangRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSiteInfoByLangRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetSiteInfoByLangRequest): GetSiteInfoByLangRequest.AsObject;
  static serializeBinaryToWriter(message: GetSiteInfoByLangRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSiteInfoByLangRequest;
  static deserializeBinaryFromReader(message: GetSiteInfoByLangRequest, reader: jspb.BinaryReader): GetSiteInfoByLangRequest;
}

export namespace GetSiteInfoByLangRequest {
  export type AsObject = {
    lang: string,
  }
}

export class SetSiteInfoByLangRequest extends jspb.Message {
  getLang(): string;
  setLang(value: string): SetSiteInfoByLangRequest;

  getItem(): GetSiteInfoByLangResponse | undefined;
  setItem(value?: GetSiteInfoByLangResponse): SetSiteInfoByLangRequest;
  hasItem(): boolean;
  clearItem(): SetSiteInfoByLangRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetSiteInfoByLangRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SetSiteInfoByLangRequest): SetSiteInfoByLangRequest.AsObject;
  static serializeBinaryToWriter(message: SetSiteInfoByLangRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetSiteInfoByLangRequest;
  static deserializeBinaryFromReader(message: SetSiteInfoByLangRequest, reader: jspb.BinaryReader): SetSiteInfoByLangRequest;
}

export namespace SetSiteInfoByLangRequest {
  export type AsObject = {
    lang: string,
    item?: GetSiteInfoByLangResponse.AsObject,
  }
}

export class GetSiteInfoByLangResponse extends jspb.Message {
  getTitle(): string;
  setTitle(value: string): GetSiteInfoByLangResponse;

  getSubhead(): string;
  setSubhead(value: string): GetSiteInfoByLangResponse;

  getDescription(): string;
  setDescription(value: string): GetSiteInfoByLangResponse;

  getCopyright(): string;
  setCopyright(value: string): GetSiteInfoByLangResponse;

  getKeywordsList(): Array<string>;
  setKeywordsList(value: Array<string>): GetSiteInfoByLangResponse;
  clearKeywordsList(): GetSiteInfoByLangResponse;
  addKeywords(value: string, index?: number): GetSiteInfoByLangResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSiteInfoByLangResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetSiteInfoByLangResponse): GetSiteInfoByLangResponse.AsObject;
  static serializeBinaryToWriter(message: GetSiteInfoByLangResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSiteInfoByLangResponse;
  static deserializeBinaryFromReader(message: GetSiteInfoByLangResponse, reader: jspb.BinaryReader): GetSiteInfoByLangResponse;
}

export namespace GetSiteInfoByLangResponse {
  export type AsObject = {
    title: string,
    subhead: string,
    description: string,
    copyright: string,
    keywordsList: Array<string>,
  }
}

export class BaiduSiteOwnershipVerification extends jspb.Message {
  getCode(): string;
  setCode(value: string): BaiduSiteOwnershipVerification;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BaiduSiteOwnershipVerification.AsObject;
  static toObject(includeInstance: boolean, msg: BaiduSiteOwnershipVerification): BaiduSiteOwnershipVerification.AsObject;
  static serializeBinaryToWriter(message: BaiduSiteOwnershipVerification, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BaiduSiteOwnershipVerification;
  static deserializeBinaryFromReader(message: BaiduSiteOwnershipVerification, reader: jspb.BinaryReader): BaiduSiteOwnershipVerification;
}

export namespace BaiduSiteOwnershipVerification {
  export type AsObject = {
    code: string,
  }
}

export class ReCaptchaProfile extends jspb.Message {
  getKey(): string;
  setKey(value: string): ReCaptchaProfile;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ReCaptchaProfile.AsObject;
  static toObject(includeInstance: boolean, msg: ReCaptchaProfile): ReCaptchaProfile.AsObject;
  static serializeBinaryToWriter(message: ReCaptchaProfile, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ReCaptchaProfile;
  static deserializeBinaryFromReader(message: ReCaptchaProfile, reader: jspb.BinaryReader): ReCaptchaProfile;
}

export namespace ReCaptchaProfile {
  export type AsObject = {
    key: string,
  }
}

export class GoogleSiteOwnershipVerification extends jspb.Message {
  getCode(): string;
  setCode(value: string): GoogleSiteOwnershipVerification;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GoogleSiteOwnershipVerification.AsObject;
  static toObject(includeInstance: boolean, msg: GoogleSiteOwnershipVerification): GoogleSiteOwnershipVerification.AsObject;
  static serializeBinaryToWriter(message: GoogleSiteOwnershipVerification, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GoogleSiteOwnershipVerification;
  static deserializeBinaryFromReader(message: GoogleSiteOwnershipVerification, reader: jspb.BinaryReader): GoogleSiteOwnershipVerification;
}

export namespace GoogleSiteOwnershipVerification {
  export type AsObject = {
    code: string,
  }
}

export class IndexNowProfile extends jspb.Message {
  getKey(): string;
  setKey(value: string): IndexNowProfile;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IndexNowProfile.AsObject;
  static toObject(includeInstance: boolean, msg: IndexNowProfile): IndexNowProfile.AsObject;
  static serializeBinaryToWriter(message: IndexNowProfile, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IndexNowProfile;
  static deserializeBinaryFromReader(message: IndexNowProfile, reader: jspb.BinaryReader): IndexNowProfile;
}

export namespace IndexNowProfile {
  export type AsObject = {
    key: string,
  }
}

export class SiteTimezonesResponse extends jspb.Message {
  getItemsList(): Array<string>;
  setItemsList(value: Array<string>): SiteTimezonesResponse;
  clearItemsList(): SiteTimezonesResponse;
  addItems(value: string, index?: number): SiteTimezonesResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteTimezonesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SiteTimezonesResponse): SiteTimezonesResponse.AsObject;
  static serializeBinaryToWriter(message: SiteTimezonesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteTimezonesResponse;
  static deserializeBinaryFromReader(message: SiteTimezonesResponse, reader: jspb.BinaryReader): SiteTimezonesResponse;
}

export namespace SiteTimezonesResponse {
  export type AsObject = {
    itemsList: Array<string>,
  }
}

export class SiteLanguagesResponse extends jspb.Message {
  getItemsList(): Array<string>;
  setItemsList(value: Array<string>): SiteLanguagesResponse;
  clearItemsList(): SiteLanguagesResponse;
  addItems(value: string, index?: number): SiteLanguagesResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteLanguagesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SiteLanguagesResponse): SiteLanguagesResponse.AsObject;
  static serializeBinaryToWriter(message: SiteLanguagesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteLanguagesResponse;
  static deserializeBinaryFromReader(message: SiteLanguagesResponse, reader: jspb.BinaryReader): SiteLanguagesResponse;
}

export namespace SiteLanguagesResponse {
  export type AsObject = {
    itemsList: Array<string>,
  }
}

export class SiteCurrenciesResponse extends jspb.Message {
  getItemsList(): Array<SiteCurrenciesResponse.Item>;
  setItemsList(value: Array<SiteCurrenciesResponse.Item>): SiteCurrenciesResponse;
  clearItemsList(): SiteCurrenciesResponse;
  addItems(value?: SiteCurrenciesResponse.Item, index?: number): SiteCurrenciesResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteCurrenciesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SiteCurrenciesResponse): SiteCurrenciesResponse.AsObject;
  static serializeBinaryToWriter(message: SiteCurrenciesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteCurrenciesResponse;
  static deserializeBinaryFromReader(message: SiteCurrenciesResponse, reader: jspb.BinaryReader): SiteCurrenciesResponse;
}

export namespace SiteCurrenciesResponse {
  export type AsObject = {
    itemsList: Array<SiteCurrenciesResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getName(): string;
    setName(value: string): Item;

    getCode(): string;
    setCode(value: string): Item;

    getCountry(): string;
    setCountry(value: string): Item;

    getUnits(): number;
    setUnits(value: number): Item;
    hasUnits(): boolean;
    clearUnits(): Item;

    getFund(): boolean;
    setFund(value: boolean): Item;
    hasFund(): boolean;
    clearFund(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      id: number,
      name: string,
      code: string,
      country: string,
      units?: number,
      fund?: boolean,
    }

    export enum UnitsCase { 
      _UNITS_NOT_SET = 0,
      UNITS = 5,
    }

    export enum FundCase { 
      _FUND_NOT_SET = 0,
      FUND = 6,
    }
  }

}

export class Rss extends jspb.Message {
  getChannel(): Rss.Channel | undefined;
  setChannel(value?: Rss.Channel): Rss;
  hasChannel(): boolean;
  clearChannel(): Rss;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Rss.AsObject;
  static toObject(includeInstance: boolean, msg: Rss): Rss.AsObject;
  static serializeBinaryToWriter(message: Rss, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Rss;
  static deserializeBinaryFromReader(message: Rss, reader: jspb.BinaryReader): Rss;
}

export namespace Rss {
  export type AsObject = {
    channel?: Rss.Channel.AsObject,
  }

  export class Channel extends jspb.Message {
    getTitle(): string;
    setTitle(value: string): Channel;

    getDescription(): string;
    setDescription(value: string): Channel;

    getLink(): string;
    setLink(value: string): Channel;

    getCopyright(): string;
    setCopyright(value: string): Channel;

    getLastBuildDate(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setLastBuildDate(value?: google_protobuf_timestamp_pb.Timestamp): Channel;
    hasLastBuildDate(): boolean;
    clearLastBuildDate(): Channel;

    getPubDate(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setPubDate(value?: google_protobuf_timestamp_pb.Timestamp): Channel;
    hasPubDate(): boolean;
    clearPubDate(): Channel;

    getTtl(): google_protobuf_duration_pb.Duration | undefined;
    setTtl(value?: google_protobuf_duration_pb.Duration): Channel;
    hasTtl(): boolean;
    clearTtl(): Channel;

    getItemsList(): Array<Rss.Channel.Item>;
    setItemsList(value: Array<Rss.Channel.Item>): Channel;
    clearItemsList(): Channel;
    addItems(value?: Rss.Channel.Item, index?: number): Rss.Channel.Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Channel.AsObject;
    static toObject(includeInstance: boolean, msg: Channel): Channel.AsObject;
    static serializeBinaryToWriter(message: Channel, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Channel;
    static deserializeBinaryFromReader(message: Channel, reader: jspb.BinaryReader): Channel;
  }

  export namespace Channel {
    export type AsObject = {
      title: string,
      description: string,
      link: string,
      copyright: string,
      lastBuildDate?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      pubDate?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      ttl?: google_protobuf_duration_pb.Duration.AsObject,
      itemsList: Array<Rss.Channel.Item.AsObject>,
    }

    export class Item extends jspb.Message {
      getTitle(): string;
      setTitle(value: string): Item;

      getDescription(): string;
      setDescription(value: string): Item;

      getLink(): string;
      setLink(value: string): Item;

      getPubDate(): google_protobuf_timestamp_pb.Timestamp | undefined;
      setPubDate(value?: google_protobuf_timestamp_pb.Timestamp): Item;
      hasPubDate(): boolean;
      clearPubDate(): Item;

      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): Item.AsObject;
      static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
      static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): Item;
      static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
    }

    export namespace Item {
      export type AsObject = {
        title: string,
        description: string,
        link: string,
        pubDate?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      }

      export class Guid extends jspb.Message {
        getPermanentLink(): boolean;
        setPermanentLink(value: boolean): Guid;

        getId(): string;
        setId(value: string): Guid;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): Guid.AsObject;
        static toObject(includeInstance: boolean, msg: Guid): Guid.AsObject;
        static serializeBinaryToWriter(message: Guid, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): Guid;
        static deserializeBinaryFromReader(message: Guid, reader: jspb.BinaryReader): Guid;
      }

      export namespace Guid {
        export type AsObject = {
          permanentLink: boolean,
          id: string,
        }
      }

    }

  }

}

export class Sitemap extends jspb.Message {
  getItemsMap(): jspb.Map<string, Sitemap.UrlSet>;
  clearItemsMap(): Sitemap;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Sitemap.AsObject;
  static toObject(includeInstance: boolean, msg: Sitemap): Sitemap.AsObject;
  static serializeBinaryToWriter(message: Sitemap, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Sitemap;
  static deserializeBinaryFromReader(message: Sitemap, reader: jspb.BinaryReader): Sitemap;
}

export namespace Sitemap {
  export type AsObject = {
    itemsMap: Array<[string, Sitemap.UrlSet.AsObject]>,
  }

  export class Url extends jspb.Message {
    getLoc(): string;
    setLoc(value: string): Url;

    getChangeFreq(): string;
    setChangeFreq(value: string): Url;
    hasChangeFreq(): boolean;
    clearChangeFreq(): Url;

    getMobile(): boolean;
    setMobile(value: boolean): Url;
    hasMobile(): boolean;
    clearMobile(): Url;

    getPriority(): number;
    setPriority(value: number): Url;
    hasPriority(): boolean;
    clearPriority(): Url;

    getLastMod(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setLastMod(value?: google_protobuf_timestamp_pb.Timestamp): Url;
    hasLastMod(): boolean;
    clearLastMod(): Url;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Url.AsObject;
    static toObject(includeInstance: boolean, msg: Url): Url.AsObject;
    static serializeBinaryToWriter(message: Url, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Url;
    static deserializeBinaryFromReader(message: Url, reader: jspb.BinaryReader): Url;
  }

  export namespace Url {
    export type AsObject = {
      loc: string,
      changeFreq?: string,
      mobile?: boolean,
      priority?: number,
      lastMod?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum ChangeFreq { 
      ALWAYS = 0,
      HOURLY = 1,
      DAILY = 2,
      WEEKLY = 3,
      MONTHLY = 4,
      YEARLY = 5,
      NEVER = 6,
    }

    export enum ChangeFreqCase { 
      _CHANGE_FREQ_NOT_SET = 0,
      CHANGE_FREQ = 2,
    }

    export enum MobileCase { 
      _MOBILE_NOT_SET = 0,
      MOBILE = 3,
    }

    export enum PriorityCase { 
      _PRIORITY_NOT_SET = 0,
      PRIORITY = 4,
    }

    export enum LastModCase { 
      _LAST_MOD_NOT_SET = 0,
      LAST_MOD = 5,
    }
  }


  export class UrlSet extends jspb.Message {
    getItemsList(): Array<Sitemap.Url>;
    setItemsList(value: Array<Sitemap.Url>): UrlSet;
    clearItemsList(): UrlSet;
    addItems(value?: Sitemap.Url, index?: number): Sitemap.Url;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): UrlSet.AsObject;
    static toObject(includeInstance: boolean, msg: UrlSet): UrlSet.AsObject;
    static serializeBinaryToWriter(message: UrlSet, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): UrlSet;
    static deserializeBinaryFromReader(message: UrlSet, reader: jspb.BinaryReader): UrlSet;
  }

  export namespace UrlSet {
    export type AsObject = {
      itemsList: Array<Sitemap.Url.AsObject>,
    }
  }

}

export class HtmlPage extends jspb.Message {
  getLang(): string;
  setLang(value: string): HtmlPage;

  getTemplate(): string;
  setTemplate(value: string): HtmlPage;

  getData(): Uint8Array | string;
  getData_asU8(): Uint8Array;
  getData_asB64(): string;
  setData(value: Uint8Array | string): HtmlPage;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HtmlPage.AsObject;
  static toObject(includeInstance: boolean, msg: HtmlPage): HtmlPage.AsObject;
  static serializeBinaryToWriter(message: HtmlPage, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HtmlPage;
  static deserializeBinaryFromReader(message: HtmlPage, reader: jspb.BinaryReader): HtmlPage;
}

export namespace HtmlPage {
  export type AsObject = {
    lang: string,
    template: string,
    data: Uint8Array | string,
  }
}

export class Theme extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Theme.AsObject;
  static toObject(includeInstance: boolean, msg: Theme): Theme.AsObject;
  static serializeBinaryToWriter(message: Theme, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Theme;
  static deserializeBinaryFromReader(message: Theme, reader: jspb.BinaryReader): Theme;
}

export namespace Theme {
  export type AsObject = {
  }

  export class Bootstrap extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Bootstrap.AsObject;
    static toObject(includeInstance: boolean, msg: Bootstrap): Bootstrap.AsObject;
    static serializeBinaryToWriter(message: Bootstrap, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Bootstrap;
    static deserializeBinaryFromReader(message: Bootstrap, reader: jspb.BinaryReader): Bootstrap;
  }

  export namespace Bootstrap {
    export type AsObject = {
    }

    export class Home extends jspb.Message {
      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): Home.AsObject;
      static toObject(includeInstance: boolean, msg: Home): Home.AsObject;
      static serializeBinaryToWriter(message: Home, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): Home;
      static deserializeBinaryFromReader(message: Home, reader: jspb.BinaryReader): Home;
    }

    export namespace Home {
      export type AsObject = {
      }
    }


    export class Sample extends jspb.Message {
      getTemplatesMap(): jspb.Map<string, string>;
      clearTemplatesMap(): Sample;

      getData(): string;
      setData(value: string): Sample;

      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): Sample.AsObject;
      static toObject(includeInstance: boolean, msg: Sample): Sample.AsObject;
      static serializeBinaryToWriter(message: Sample, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): Sample;
      static deserializeBinaryFromReader(message: Sample, reader: jspb.BinaryReader): Sample;
    }

    export namespace Sample {
      export type AsObject = {
        templatesMap: Array<[string, string]>,
        data: string,
      }

      export class Data extends jspb.Message {
        getHeader(): Theme.Bootstrap.Sample.Data.Header | undefined;
        setHeader(value?: Theme.Bootstrap.Sample.Data.Header): Data;
        hasHeader(): boolean;
        clearHeader(): Data;

        getFooter(): Theme.Bootstrap.Sample.Data.Footer | undefined;
        setFooter(value?: Theme.Bootstrap.Sample.Data.Footer): Data;
        hasFooter(): boolean;
        clearFooter(): Data;

        getBody(): Theme.Bootstrap.Sample.Data.Body | undefined;
        setBody(value?: Theme.Bootstrap.Sample.Data.Body): Data;
        hasBody(): boolean;
        clearBody(): Data;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): Data.AsObject;
        static toObject(includeInstance: boolean, msg: Data): Data.AsObject;
        static serializeBinaryToWriter(message: Data, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): Data;
        static deserializeBinaryFromReader(message: Data, reader: jspb.BinaryReader): Data;
      }

      export namespace Data {
        export type AsObject = {
          header?: Theme.Bootstrap.Sample.Data.Header.AsObject,
          footer?: Theme.Bootstrap.Sample.Data.Footer.AsObject,
          body?: Theme.Bootstrap.Sample.Data.Body.AsObject,
        }

        export class Header extends jspb.Message {
          getTitle(): string;
          setTitle(value: string): Header;

          serializeBinary(): Uint8Array;
          toObject(includeInstance?: boolean): Header.AsObject;
          static toObject(includeInstance: boolean, msg: Header): Header.AsObject;
          static serializeBinaryToWriter(message: Header, writer: jspb.BinaryWriter): void;
          static deserializeBinary(bytes: Uint8Array): Header;
          static deserializeBinaryFromReader(message: Header, reader: jspb.BinaryReader): Header;
        }

        export namespace Header {
          export type AsObject = {
            title: string,
          }
        }


        export class Footer extends jspb.Message {
          getCopyright(): string;
          setCopyright(value: string): Footer;

          serializeBinary(): Uint8Array;
          toObject(includeInstance?: boolean): Footer.AsObject;
          static toObject(includeInstance: boolean, msg: Footer): Footer.AsObject;
          static serializeBinaryToWriter(message: Footer, writer: jspb.BinaryWriter): void;
          static deserializeBinary(bytes: Uint8Array): Footer;
          static deserializeBinaryFromReader(message: Footer, reader: jspb.BinaryReader): Footer;
        }

        export namespace Footer {
          export type AsObject = {
            copyright: string,
          }
        }


        export class Body extends jspb.Message {
          getTitle(): string;
          setTitle(value: string): Body;

          getItemsList(): Array<Theme.Bootstrap.Sample.Data.Body.Item>;
          setItemsList(value: Array<Theme.Bootstrap.Sample.Data.Body.Item>): Body;
          clearItemsList(): Body;
          addItems(value?: Theme.Bootstrap.Sample.Data.Body.Item, index?: number): Theme.Bootstrap.Sample.Data.Body.Item;

          getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
          setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Body;
          hasCreatedAt(): boolean;
          clearCreatedAt(): Body;

          serializeBinary(): Uint8Array;
          toObject(includeInstance?: boolean): Body.AsObject;
          static toObject(includeInstance: boolean, msg: Body): Body.AsObject;
          static serializeBinaryToWriter(message: Body, writer: jspb.BinaryWriter): void;
          static deserializeBinary(bytes: Uint8Array): Body;
          static deserializeBinaryFromReader(message: Body, reader: jspb.BinaryReader): Body;
        }

        export namespace Body {
          export type AsObject = {
            title: string,
            itemsList: Array<Theme.Bootstrap.Sample.Data.Body.Item.AsObject>,
            createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
          }

          export class Link extends jspb.Message {
            getLabel(): string;
            setLabel(value: string): Link;

            getHref(): string;
            setHref(value: string): Link;

            serializeBinary(): Uint8Array;
            toObject(includeInstance?: boolean): Link.AsObject;
            static toObject(includeInstance: boolean, msg: Link): Link.AsObject;
            static serializeBinaryToWriter(message: Link, writer: jspb.BinaryWriter): void;
            static deserializeBinary(bytes: Uint8Array): Link;
            static deserializeBinaryFromReader(message: Link, reader: jspb.BinaryReader): Link;
          }

          export namespace Link {
            export type AsObject = {
              label: string,
              href: string,
            }
          }


          export class Panel extends jspb.Message {
            getTitle(): string;
            setTitle(value: string): Panel;

            getDescription(): string;
            setDescription(value: string): Panel;

            getLinksList(): Array<Theme.Bootstrap.Sample.Data.Body.Link>;
            setLinksList(value: Array<Theme.Bootstrap.Sample.Data.Body.Link>): Panel;
            clearLinksList(): Panel;
            addLinks(value?: Theme.Bootstrap.Sample.Data.Body.Link, index?: number): Theme.Bootstrap.Sample.Data.Body.Link;

            serializeBinary(): Uint8Array;
            toObject(includeInstance?: boolean): Panel.AsObject;
            static toObject(includeInstance: boolean, msg: Panel): Panel.AsObject;
            static serializeBinaryToWriter(message: Panel, writer: jspb.BinaryWriter): void;
            static deserializeBinary(bytes: Uint8Array): Panel;
            static deserializeBinaryFromReader(message: Panel, reader: jspb.BinaryReader): Panel;
          }

          export namespace Panel {
            export type AsObject = {
              title: string,
              description: string,
              linksList: Array<Theme.Bootstrap.Sample.Data.Body.Link.AsObject>,
            }
          }


          export class Item extends jspb.Message {
            getTitle(): string;
            setTitle(value: string): Item;

            getDescription(): string;
            setDescription(value: string): Item;

            getPanelsMap(): jspb.Map<string, Theme.Bootstrap.Sample.Data.Body.Panel>;
            clearPanelsMap(): Item;

            serializeBinary(): Uint8Array;
            toObject(includeInstance?: boolean): Item.AsObject;
            static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
            static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
            static deserializeBinary(bytes: Uint8Array): Item;
            static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
          }

          export namespace Item {
            export type AsObject = {
              title: string,
              description: string,
              panelsMap: Array<[string, Theme.Bootstrap.Sample.Data.Body.Panel.AsObject]>,
            }
          }

        }

      }

    }

  }


  export class Bulma extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Bulma.AsObject;
    static toObject(includeInstance: boolean, msg: Bulma): Bulma.AsObject;
    static serializeBinaryToWriter(message: Bulma, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Bulma;
    static deserializeBinaryFromReader(message: Bulma, reader: jspb.BinaryReader): Bulma;
  }

  export namespace Bulma {
    export type AsObject = {
    }

    export class Home extends jspb.Message {
      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): Home.AsObject;
      static toObject(includeInstance: boolean, msg: Home): Home.AsObject;
      static serializeBinaryToWriter(message: Home, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): Home;
      static deserializeBinaryFromReader(message: Home, reader: jspb.BinaryReader): Home;
    }

    export namespace Home {
      export type AsObject = {
      }
    }

  }

}

