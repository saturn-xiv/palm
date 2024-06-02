import * as jspb from 'google-protobuf'

import * as google_protobuf_duration_pb from 'google-protobuf/google/protobuf/duration_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as azalea_pb from './azalea_pb';


export class UserInfo extends jspb.Message {
  getUid(): string;
  setUid(value: string): UserInfo;

  getName(): string;
  setName(value: string): UserInfo;

  getAvatar(): string;
  setAvatar(value: string): UserInfo;
  hasAvatar(): boolean;
  clearAvatar(): UserInfo;

  getDeletedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setDeletedAt(value?: google_protobuf_timestamp_pb.Timestamp): UserInfo;
  hasDeletedAt(): boolean;
  clearDeletedAt(): UserInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserInfo.AsObject;
  static toObject(includeInstance: boolean, msg: UserInfo): UserInfo.AsObject;
  static serializeBinaryToWriter(message: UserInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserInfo;
  static deserializeBinaryFromReader(message: UserInfo, reader: jspb.BinaryReader): UserInfo;
}

export namespace UserInfo {
  export type AsObject = {
    uid: string,
    name: string,
    avatar?: string,
    deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }

  export enum AvatarCase { 
    _AVATAR_NOT_SET = 0,
    AVATAR = 3,
  }

  export enum DeletedAtCase { 
    _DELETED_AT_NOT_SET = 0,
    DELETED_AT = 99,
  }
}

export class UserOauth2State extends jspb.Message {
  getUser(): number;
  setUser(value: number): UserOauth2State;
  hasUser(): boolean;
  clearUser(): UserOauth2State;

  getCode(): string;
  setCode(value: string): UserOauth2State;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserOauth2State.AsObject;
  static toObject(includeInstance: boolean, msg: UserOauth2State): UserOauth2State.AsObject;
  static serializeBinaryToWriter(message: UserOauth2State, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserOauth2State;
  static deserializeBinaryFromReader(message: UserOauth2State, reader: jspb.BinaryReader): UserOauth2State;
}

export namespace UserOauth2State {
  export type AsObject = {
    user?: number,
    code: string,
  }

  export enum UserCase { 
    _USER_NOT_SET = 0,
    USER = 1,
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

export class Permission extends jspb.Message {
  getOperation(): string;
  setOperation(value: string): Permission;

  getResource(): Resource | undefined;
  setResource(value?: Resource): Permission;
  hasResource(): boolean;
  clearResource(): Permission;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Permission.AsObject;
  static toObject(includeInstance: boolean, msg: Permission): Permission.AsObject;
  static serializeBinaryToWriter(message: Permission, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Permission;
  static deserializeBinaryFromReader(message: Permission, reader: jspb.BinaryReader): Permission;
}

export namespace Permission {
  export type AsObject = {
    operation: string,
    resource?: Resource.AsObject,
  }
}

export class Menu extends jspb.Message {
  getKey(): string;
  setKey(value: string): Menu;

  getLabel(): string;
  setLabel(value: string): Menu;

  getPath(): string;
  setPath(value: string): Menu;

  getChildrenList(): Array<Menu>;
  setChildrenList(value: Array<Menu>): Menu;
  clearChildrenList(): Menu;
  addChildren(value?: Menu, index?: number): Menu;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Menu.AsObject;
  static toObject(includeInstance: boolean, msg: Menu): Menu.AsObject;
  static serializeBinaryToWriter(message: Menu, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Menu;
  static deserializeBinaryFromReader(message: Menu, reader: jspb.BinaryReader): Menu;
}

export namespace Menu {
  export type AsObject = {
    key: string,
    label: string,
    path: string,
    childrenList: Array<Menu.AsObject>,
  }
}

export class UserIndexResponse extends jspb.Message {
  getItemsList(): Array<UserIndexResponse.Item>;
  setItemsList(value: Array<UserIndexResponse.Item>): UserIndexResponse;
  clearItemsList(): UserIndexResponse;
  addItems(value?: UserIndexResponse.Item, index?: number): UserIndexResponse.Item;

  getPagination(): azalea_pb.Pagination | undefined;
  setPagination(value?: azalea_pb.Pagination): UserIndexResponse;
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
    pagination?: azalea_pb.Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getType(): UserIndexResponse.Item.Type;
    setType(value: UserIndexResponse.Item.Type): Item;

    getId(): string;
    setId(value: string): Item;

    getName(): string;
    setName(value: string): Item;
    hasName(): boolean;
    clearName(): Item;

    getLang(): string;
    setLang(value: string): Item;

    getTimezone(): string;
    setTimezone(value: string): Item;

    getAvatar(): string;
    setAvatar(value: string): Item;
    hasAvatar(): boolean;
    clearAvatar(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      type: UserIndexResponse.Item.Type,
      id: string,
      name?: string,
      lang: string,
      timezone: string,
      avatar?: string,
    }

    export enum Type { 
      EMAIL = 0,
      PHONE = 1,
      GOOGLE = 2,
      WECHATMINIPROGRAM = 11,
      WECHATOAUTH2 = 12,
    }

    export enum NameCase { 
      _NAME_NOT_SET = 0,
      NAME = 11,
    }

    export enum AvatarCase { 
      _AVATAR_NOT_SET = 0,
      AVATAR = 19,
    }
  }

}

export class UserCancelRequest extends jspb.Message {
  getHome(): string;
  setHome(value: string): UserCancelRequest;

  getReason(): string;
  setReason(value: string): UserCancelRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserCancelRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserCancelRequest): UserCancelRequest.AsObject;
  static serializeBinaryToWriter(message: UserCancelRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserCancelRequest;
  static deserializeBinaryFromReader(message: UserCancelRequest, reader: jspb.BinaryReader): UserCancelRequest;
}

export namespace UserCancelRequest {
  export type AsObject = {
    home: string,
    reason: string,
  }
}

export class UserCancelResponse extends jspb.Message {
  getReminder(): string;
  setReminder(value: string): UserCancelResponse;

  getRedirectTo(): string;
  setRedirectTo(value: string): UserCancelResponse;

  getPayloadCase(): UserCancelResponse.PayloadCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserCancelResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserCancelResponse): UserCancelResponse.AsObject;
  static serializeBinaryToWriter(message: UserCancelResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserCancelResponse;
  static deserializeBinaryFromReader(message: UserCancelResponse, reader: jspb.BinaryReader): UserCancelResponse;
}

export namespace UserCancelResponse {
  export type AsObject = {
    reminder: string,
    redirectTo: string,
  }

  export enum PayloadCase { 
    PAYLOAD_NOT_SET = 0,
    REMINDER = 1,
    REDIRECT_TO = 2,
  }
}

export class UserSetProfileRequest extends jspb.Message {
  getRealName(): string;
  setRealName(value: string): UserSetProfileRequest;

  getAvatar(): string;
  setAvatar(value: string): UserSetProfileRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSetProfileRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserSetProfileRequest): UserSetProfileRequest.AsObject;
  static serializeBinaryToWriter(message: UserSetProfileRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSetProfileRequest;
  static deserializeBinaryFromReader(message: UserSetProfileRequest, reader: jspb.BinaryReader): UserSetProfileRequest;
}

export namespace UserSetProfileRequest {
  export type AsObject = {
    realName: string,
    avatar: string,
  }
}

export class UserSetLocationRequest extends jspb.Message {
  getLang(): string;
  setLang(value: string): UserSetLocationRequest;

  getTimezone(): string;
  setTimezone(value: string): UserSetLocationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSetLocationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserSetLocationRequest): UserSetLocationRequest.AsObject;
  static serializeBinaryToWriter(message: UserSetLocationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSetLocationRequest;
  static deserializeBinaryFromReader(message: UserSetLocationRequest, reader: jspb.BinaryReader): UserSetLocationRequest;
}

export namespace UserSetLocationRequest {
  export type AsObject = {
    lang: string,
    timezone: string,
  }
}

export class UserChangePasswordRequest extends jspb.Message {
  getCurrentPassword(): string;
  setCurrentPassword(value: string): UserChangePasswordRequest;

  getNewPassword(): string;
  setNewPassword(value: string): UserChangePasswordRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserChangePasswordRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserChangePasswordRequest): UserChangePasswordRequest.AsObject;
  static serializeBinaryToWriter(message: UserChangePasswordRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserChangePasswordRequest;
  static deserializeBinaryFromReader(message: UserChangePasswordRequest, reader: jspb.BinaryReader): UserChangePasswordRequest;
}

export namespace UserChangePasswordRequest {
  export type AsObject = {
    currentPassword: string,
    newPassword: string,
  }
}

export class UserSetPasswordRequest extends jspb.Message {
  getProviderType(): UserIndexResponse.Item.Type;
  setProviderType(value: UserIndexResponse.Item.Type): UserSetPasswordRequest;

  getProviderId(): number;
  setProviderId(value: number): UserSetPasswordRequest;

  getPassword(): string;
  setPassword(value: string): UserSetPasswordRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSetPasswordRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserSetPasswordRequest): UserSetPasswordRequest.AsObject;
  static serializeBinaryToWriter(message: UserSetPasswordRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSetPasswordRequest;
  static deserializeBinaryFromReader(message: UserSetPasswordRequest, reader: jspb.BinaryReader): UserSetPasswordRequest;
}

export namespace UserSetPasswordRequest {
  export type AsObject = {
    providerType: UserIndexResponse.Item.Type,
    providerId: number,
    password: string,
  }
}

export class UserResetPasswordRequest extends jspb.Message {
  getToken(): string;
  setToken(value: string): UserResetPasswordRequest;

  getPassword(): string;
  setPassword(value: string): UserResetPasswordRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserResetPasswordRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserResetPasswordRequest): UserResetPasswordRequest.AsObject;
  static serializeBinaryToWriter(message: UserResetPasswordRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserResetPasswordRequest;
  static deserializeBinaryFromReader(message: UserResetPasswordRequest, reader: jspb.BinaryReader): UserResetPasswordRequest;
}

export namespace UserResetPasswordRequest {
  export type AsObject = {
    token: string,
    password: string,
  }
}

export class UserEmailRequest extends jspb.Message {
  getNickname(): string;
  setNickname(value: string): UserEmailRequest;

  getEmail(): string;
  setEmail(value: string): UserEmailRequest;

  getHome(): string;
  setHome(value: string): UserEmailRequest;

  getUserCase(): UserEmailRequest.UserCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserEmailRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserEmailRequest): UserEmailRequest.AsObject;
  static serializeBinaryToWriter(message: UserEmailRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserEmailRequest;
  static deserializeBinaryFromReader(message: UserEmailRequest, reader: jspb.BinaryReader): UserEmailRequest;
}

export namespace UserEmailRequest {
  export type AsObject = {
    nickname: string,
    email: string,
    home: string,
  }

  export enum UserCase { 
    USER_NOT_SET = 0,
    NICKNAME = 1,
    EMAIL = 2,
  }
}

export class UserTokenRequest extends jspb.Message {
  getToken(): string;
  setToken(value: string): UserTokenRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserTokenRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserTokenRequest): UserTokenRequest.AsObject;
  static serializeBinaryToWriter(message: UserTokenRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserTokenRequest;
  static deserializeBinaryFromReader(message: UserTokenRequest, reader: jspb.BinaryReader): UserTokenRequest;
}

export namespace UserTokenRequest {
  export type AsObject = {
    token: string,
  }
}

export class UserSignInResponse extends jspb.Message {
  getToken(): string;
  setToken(value: string): UserSignInResponse;

  getUser(): UserIndexResponse.Item | undefined;
  setUser(value?: UserIndexResponse.Item): UserSignInResponse;
  hasUser(): boolean;
  clearUser(): UserSignInResponse;

  getRolesList(): Array<string>;
  setRolesList(value: Array<string>): UserSignInResponse;
  clearRolesList(): UserSignInResponse;
  addRoles(value: string, index?: number): UserSignInResponse;

  getPermissionsList(): Array<Permission>;
  setPermissionsList(value: Array<Permission>): UserSignInResponse;
  clearPermissionsList(): UserSignInResponse;
  addPermissions(value?: Permission, index?: number): Permission;

  getMenusList(): Array<Menu>;
  setMenusList(value: Array<Menu>): UserSignInResponse;
  clearMenusList(): UserSignInResponse;
  addMenus(value?: Menu, index?: number): Menu;

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
    user?: UserIndexResponse.Item.AsObject,
    rolesList: Array<string>,
    permissionsList: Array<Permission.AsObject>,
    menusList: Array<Menu.AsObject>,
  }
}

export class UserSignInByPasswordRequest extends jspb.Message {
  getNickname(): string;
  setNickname(value: string): UserSignInByPasswordRequest;

  getEmail(): string;
  setEmail(value: string): UserSignInByPasswordRequest;

  getPhone(): string;
  setPhone(value: string): UserSignInByPasswordRequest;

  getPassword(): string;
  setPassword(value: string): UserSignInByPasswordRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): UserSignInByPasswordRequest;
  hasTtl(): boolean;
  clearTtl(): UserSignInByPasswordRequest;

  getWhoCase(): UserSignInByPasswordRequest.WhoCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSignInByPasswordRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserSignInByPasswordRequest): UserSignInByPasswordRequest.AsObject;
  static serializeBinaryToWriter(message: UserSignInByPasswordRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSignInByPasswordRequest;
  static deserializeBinaryFromReader(message: UserSignInByPasswordRequest, reader: jspb.BinaryReader): UserSignInByPasswordRequest;
}

export namespace UserSignInByPasswordRequest {
  export type AsObject = {
    nickname: string,
    email: string,
    phone: string,
    password: string,
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }

  export enum WhoCase { 
    WHO_NOT_SET = 0,
    NICKNAME = 1,
    EMAIL = 2,
    PHONE = 3,
  }
}

export class UserSignUpByEmailRequest extends jspb.Message {
  getRealName(): string;
  setRealName(value: string): UserSignUpByEmailRequest;

  getEmail(): string;
  setEmail(value: string): UserSignUpByEmailRequest;

  getNickname(): string;
  setNickname(value: string): UserSignUpByEmailRequest;

  getPassword(): string;
  setPassword(value: string): UserSignUpByEmailRequest;

  getTimezone(): string;
  setTimezone(value: string): UserSignUpByEmailRequest;

  getHome(): string;
  setHome(value: string): UserSignUpByEmailRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSignUpByEmailRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserSignUpByEmailRequest): UserSignUpByEmailRequest.AsObject;
  static serializeBinaryToWriter(message: UserSignUpByEmailRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSignUpByEmailRequest;
  static deserializeBinaryFromReader(message: UserSignUpByEmailRequest, reader: jspb.BinaryReader): UserSignUpByEmailRequest;
}

export namespace UserSignUpByEmailRequest {
  export type AsObject = {
    realName: string,
    email: string,
    nickname: string,
    password: string,
    timezone: string,
    home: string,
  }
}

export class UserLogsResponse extends jspb.Message {
  getItemsList(): Array<UserLogsResponse.Item>;
  setItemsList(value: Array<UserLogsResponse.Item>): UserLogsResponse;
  clearItemsList(): UserLogsResponse;
  addItems(value?: UserLogsResponse.Item, index?: number): UserLogsResponse.Item;

  getPagination(): azalea_pb.Pagination | undefined;
  setPagination(value?: azalea_pb.Pagination): UserLogsResponse;
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
    pagination?: azalea_pb.Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getLevel(): UserLogsResponse.Item.Level;
    setLevel(value: UserLogsResponse.Item.Level): Item;

    getIp(): string;
    setIp(value: string): Item;

    getMessage(): string;
    setMessage(value: string): Item;

    getResource(): Resource | undefined;
    setResource(value?: Resource): Item;
    hasResource(): boolean;
    clearResource(): Item;

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
      level: UserLogsResponse.Item.Level,
      ip: string,
      message: string,
      resource?: Resource.AsObject,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum Level { 
      DEBUG = 0,
      INFO = 1,
      WARN = 2,
      ERROR = 3,
    }
  }

}

export class AttachmentShowRequest extends jspb.Message {
  getId(): number;
  setId(value: number): AttachmentShowRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): AttachmentShowRequest;
  hasTtl(): boolean;
  clearTtl(): AttachmentShowRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AttachmentShowRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AttachmentShowRequest): AttachmentShowRequest.AsObject;
  static serializeBinaryToWriter(message: AttachmentShowRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AttachmentShowRequest;
  static deserializeBinaryFromReader(message: AttachmentShowRequest, reader: jspb.BinaryReader): AttachmentShowRequest;
}

export namespace AttachmentShowRequest {
  export type AsObject = {
    id: number,
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }

  export enum TtlCase { 
    _TTL_NOT_SET = 0,
    TTL = 9,
  }
}

export class AttachmentShowResponse extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): AttachmentShowResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AttachmentShowResponse.AsObject;
  static toObject(includeInstance: boolean, msg: AttachmentShowResponse): AttachmentShowResponse.AsObject;
  static serializeBinaryToWriter(message: AttachmentShowResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AttachmentShowResponse;
  static deserializeBinaryFromReader(message: AttachmentShowResponse, reader: jspb.BinaryReader): AttachmentShowResponse;
}

export namespace AttachmentShowResponse {
  export type AsObject = {
    url: string,
  }
}

export class AttachmentResourceRequest extends jspb.Message {
  getId(): number;
  setId(value: number): AttachmentResourceRequest;

  getResourceType(): string;
  setResourceType(value: string): AttachmentResourceRequest;

  getResourceId(): number;
  setResourceId(value: number): AttachmentResourceRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AttachmentResourceRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AttachmentResourceRequest): AttachmentResourceRequest.AsObject;
  static serializeBinaryToWriter(message: AttachmentResourceRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AttachmentResourceRequest;
  static deserializeBinaryFromReader(message: AttachmentResourceRequest, reader: jspb.BinaryReader): AttachmentResourceRequest;
}

export namespace AttachmentResourceRequest {
  export type AsObject = {
    id: number,
    resourceType: string,
    resourceId: number,
  }
}

export class AttachmentUpdateRequest extends jspb.Message {
  getId(): number;
  setId(value: number): AttachmentUpdateRequest;

  getTitle(): string;
  setTitle(value: string): AttachmentUpdateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AttachmentUpdateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AttachmentUpdateRequest): AttachmentUpdateRequest.AsObject;
  static serializeBinaryToWriter(message: AttachmentUpdateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AttachmentUpdateRequest;
  static deserializeBinaryFromReader(message: AttachmentUpdateRequest, reader: jspb.BinaryReader): AttachmentUpdateRequest;
}

export namespace AttachmentUpdateRequest {
  export type AsObject = {
    id: number,
    title: string,
  }
}

export class AttachmentUploadUrlRequest extends jspb.Message {
  getTitle(): string;
  setTitle(value: string): AttachmentUploadUrlRequest;

  getContentType(): string;
  setContentType(value: string): AttachmentUploadUrlRequest;

  getSize(): number;
  setSize(value: number): AttachmentUploadUrlRequest;

  getPublic(): boolean;
  setPublic(value: boolean): AttachmentUploadUrlRequest;

  getExpirationDays(): number;
  setExpirationDays(value: number): AttachmentUploadUrlRequest;
  hasExpirationDays(): boolean;
  clearExpirationDays(): AttachmentUploadUrlRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AttachmentUploadUrlRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AttachmentUploadUrlRequest): AttachmentUploadUrlRequest.AsObject;
  static serializeBinaryToWriter(message: AttachmentUploadUrlRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AttachmentUploadUrlRequest;
  static deserializeBinaryFromReader(message: AttachmentUploadUrlRequest, reader: jspb.BinaryReader): AttachmentUploadUrlRequest;
}

export namespace AttachmentUploadUrlRequest {
  export type AsObject = {
    title: string,
    contentType: string,
    size: number,
    pb_public: boolean,
    expirationDays?: number,
  }

  export enum ExpirationDaysCase { 
    _EXPIRATION_DAYS_NOT_SET = 0,
    EXPIRATION_DAYS = 12,
  }
}

export class AttachmentUploadUrlResponse extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): AttachmentUploadUrlResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AttachmentUploadUrlResponse.AsObject;
  static toObject(includeInstance: boolean, msg: AttachmentUploadUrlResponse): AttachmentUploadUrlResponse.AsObject;
  static serializeBinaryToWriter(message: AttachmentUploadUrlResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AttachmentUploadUrlResponse;
  static deserializeBinaryFromReader(message: AttachmentUploadUrlResponse, reader: jspb.BinaryReader): AttachmentUploadUrlResponse;
}

export namespace AttachmentUploadUrlResponse {
  export type AsObject = {
    url: string,
  }
}

export class AttachmentIndexResponse extends jspb.Message {
  getItemsList(): Array<AttachmentIndexResponse.Item>;
  setItemsList(value: Array<AttachmentIndexResponse.Item>): AttachmentIndexResponse;
  clearItemsList(): AttachmentIndexResponse;
  addItems(value?: AttachmentIndexResponse.Item, index?: number): AttachmentIndexResponse.Item;

  getPagination(): azalea_pb.Pagination | undefined;
  setPagination(value?: azalea_pb.Pagination): AttachmentIndexResponse;
  hasPagination(): boolean;
  clearPagination(): AttachmentIndexResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AttachmentIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: AttachmentIndexResponse): AttachmentIndexResponse.AsObject;
  static serializeBinaryToWriter(message: AttachmentIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AttachmentIndexResponse;
  static deserializeBinaryFromReader(message: AttachmentIndexResponse, reader: jspb.BinaryReader): AttachmentIndexResponse;
}

export namespace AttachmentIndexResponse {
  export type AsObject = {
    itemsList: Array<AttachmentIndexResponse.Item.AsObject>,
    pagination?: azalea_pb.Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getBucket(): string;
    setBucket(value: string): Item;

    getName(): string;
    setName(value: string): Item;

    getTitle(): string;
    setTitle(value: string): Item;

    getSize(): number;
    setSize(value: number): Item;

    getContentType(): string;
    setContentType(value: string): Item;

    getPublishedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setPublishedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasPublishedAt(): boolean;
    clearPublishedAt(): Item;

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
      bucket: string,
      name: string,
      title: string,
      size: number,
      contentType: string,
      publishedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum PublishedAtCase { 
      _PUBLISHED_AT_NOT_SET = 0,
      PUBLISHED_AT = 7,
    }

    export enum DeletedAtCase { 
      _DELETED_AT_NOT_SET = 0,
      DELETED_AT = 8,
    }
  }

}

export class LeaveWordCreateRequest extends jspb.Message {
  getBody(): string;
  setBody(value: string): LeaveWordCreateRequest;

  getEditor(): azalea_pb.TextEditor;
  setEditor(value: azalea_pb.TextEditor): LeaveWordCreateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LeaveWordCreateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LeaveWordCreateRequest): LeaveWordCreateRequest.AsObject;
  static serializeBinaryToWriter(message: LeaveWordCreateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LeaveWordCreateRequest;
  static deserializeBinaryFromReader(message: LeaveWordCreateRequest, reader: jspb.BinaryReader): LeaveWordCreateRequest;
}

export namespace LeaveWordCreateRequest {
  export type AsObject = {
    body: string,
    editor: azalea_pb.TextEditor,
  }
}

export class LeaveWordIndexResponse extends jspb.Message {
  getItemsList(): Array<LeaveWordIndexResponse.Item>;
  setItemsList(value: Array<LeaveWordIndexResponse.Item>): LeaveWordIndexResponse;
  clearItemsList(): LeaveWordIndexResponse;
  addItems(value?: LeaveWordIndexResponse.Item, index?: number): LeaveWordIndexResponse.Item;

  getPagination(): azalea_pb.Pagination | undefined;
  setPagination(value?: azalea_pb.Pagination): LeaveWordIndexResponse;
  hasPagination(): boolean;
  clearPagination(): LeaveWordIndexResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LeaveWordIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LeaveWordIndexResponse): LeaveWordIndexResponse.AsObject;
  static serializeBinaryToWriter(message: LeaveWordIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LeaveWordIndexResponse;
  static deserializeBinaryFromReader(message: LeaveWordIndexResponse, reader: jspb.BinaryReader): LeaveWordIndexResponse;
}

export namespace LeaveWordIndexResponse {
  export type AsObject = {
    itemsList: Array<LeaveWordIndexResponse.Item.AsObject>,
    pagination?: azalea_pb.Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getLang(): string;
    setLang(value: string): Item;

    getIp(): string;
    setIp(value: string): Item;

    getBody(): string;
    setBody(value: string): Item;

    getEditor(): azalea_pb.TextEditor;
    setEditor(value: azalea_pb.TextEditor): Item;

    getStatus(): LeaveWordIndexResponse.Item.Status;
    setStatus(value: LeaveWordIndexResponse.Item.Status): Item;

    getPublishedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setPublishedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasPublishedAt(): boolean;
    clearPublishedAt(): Item;

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
      lang: string,
      ip: string,
      body: string,
      editor: azalea_pb.TextEditor,
      status: LeaveWordIndexResponse.Item.Status,
      publishedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum Status { 
      PENDING = 0,
      PROCESSING = 2,
      CLOSED = 3,
    }

    export enum PublishedAtCase { 
      _PUBLISHED_AT_NOT_SET = 0,
      PUBLISHED_AT = 7,
    }

    export enum DeletedAtCase { 
      _DELETED_AT_NOT_SET = 0,
      DELETED_AT = 8,
    }
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

export class LocaleSetRequest extends jspb.Message {
  getLang(): string;
  setLang(value: string): LocaleSetRequest;

  getCode(): string;
  setCode(value: string): LocaleSetRequest;

  getMessage(): string;
  setMessage(value: string): LocaleSetRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LocaleSetRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LocaleSetRequest): LocaleSetRequest.AsObject;
  static serializeBinaryToWriter(message: LocaleSetRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LocaleSetRequest;
  static deserializeBinaryFromReader(message: LocaleSetRequest, reader: jspb.BinaryReader): LocaleSetRequest;
}

export namespace LocaleSetRequest {
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

  getPagination(): azalea_pb.Pagination | undefined;
  setPagination(value?: azalea_pb.Pagination): LocaleIndexResponse;
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
    pagination?: azalea_pb.Pagination.AsObject,
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

export class BaiduProfile extends jspb.Message {
  getCode(): string;
  setCode(value: string): BaiduProfile;

  getContent(): string;
  setContent(value: string): BaiduProfile;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BaiduProfile.AsObject;
  static toObject(includeInstance: boolean, msg: BaiduProfile): BaiduProfile.AsObject;
  static serializeBinaryToWriter(message: BaiduProfile, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BaiduProfile;
  static deserializeBinaryFromReader(message: BaiduProfile, reader: jspb.BinaryReader): BaiduProfile;
}

export namespace BaiduProfile {
  export type AsObject = {
    code: string,
    content: string,
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

export class GoogleProfile extends jspb.Message {
  getCode(): string;
  setCode(value: string): GoogleProfile;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GoogleProfile.AsObject;
  static toObject(includeInstance: boolean, msg: GoogleProfile): GoogleProfile.AsObject;
  static serializeBinaryToWriter(message: GoogleProfile, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GoogleProfile;
  static deserializeBinaryFromReader(message: GoogleProfile, reader: jspb.BinaryReader): GoogleProfile;
}

export namespace GoogleProfile {
  export type AsObject = {
    code: string,
  }
}

export class ReCaptchaProfile extends jspb.Message {
  getSiteKey(): string;
  setSiteKey(value: string): ReCaptchaProfile;

  getSecret(): string;
  setSecret(value: string): ReCaptchaProfile;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ReCaptchaProfile.AsObject;
  static toObject(includeInstance: boolean, msg: ReCaptchaProfile): ReCaptchaProfile.AsObject;
  static serializeBinaryToWriter(message: ReCaptchaProfile, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ReCaptchaProfile;
  static deserializeBinaryFromReader(message: ReCaptchaProfile, reader: jspb.BinaryReader): ReCaptchaProfile;
}

export namespace ReCaptchaProfile {
  export type AsObject = {
    siteKey: string,
    secret: string,
  }
}

export class SiteSetFaviconRequest extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): SiteSetFaviconRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteSetFaviconRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SiteSetFaviconRequest): SiteSetFaviconRequest.AsObject;
  static serializeBinaryToWriter(message: SiteSetFaviconRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteSetFaviconRequest;
  static deserializeBinaryFromReader(message: SiteSetFaviconRequest, reader: jspb.BinaryReader): SiteSetFaviconRequest;
}

export namespace SiteSetFaviconRequest {
  export type AsObject = {
    url: string,
  }
}

export class SiteLayoutResponse extends jspb.Message {
  getLang(): string;
  setLang(value: string): SiteLayoutResponse;

  getTitle(): string;
  setTitle(value: string): SiteLayoutResponse;

  getSubhead(): string;
  setSubhead(value: string): SiteLayoutResponse;

  getAuthor(): SiteLayoutResponse.Author | undefined;
  setAuthor(value?: SiteLayoutResponse.Author): SiteLayoutResponse;
  hasAuthor(): boolean;
  clearAuthor(): SiteLayoutResponse;

  getDescription(): string;
  setDescription(value: string): SiteLayoutResponse;

  getKeywordsList(): Array<string>;
  setKeywordsList(value: Array<string>): SiteLayoutResponse;
  clearKeywordsList(): SiteLayoutResponse;
  addKeywords(value: string, index?: number): SiteLayoutResponse;

  getFavicon(): string;
  setFavicon(value: string): SiteLayoutResponse;

  getCopyright(): string;
  setCopyright(value: string): SiteLayoutResponse;

  getGab(): SiteLayoutResponse.GabCode | undefined;
  setGab(value?: SiteLayoutResponse.GabCode): SiteLayoutResponse;
  hasGab(): boolean;
  clearGab(): SiteLayoutResponse;

  getIcp(): SiteLayoutResponse.IcpCode | undefined;
  setIcp(value?: SiteLayoutResponse.IcpCode): SiteLayoutResponse;
  hasIcp(): boolean;
  clearIcp(): SiteLayoutResponse;

  getLanguagesList(): Array<string>;
  setLanguagesList(value: Array<string>): SiteLayoutResponse;
  clearLanguagesList(): SiteLayoutResponse;
  addLanguages(value: string, index?: number): SiteLayoutResponse;

  getVersion(): string;
  setVersion(value: string): SiteLayoutResponse;

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
    lang: string,
    title: string,
    subhead: string,
    author?: SiteLayoutResponse.Author.AsObject,
    description: string,
    keywordsList: Array<string>,
    favicon: string,
    copyright: string,
    gab?: SiteLayoutResponse.GabCode.AsObject,
    icp?: SiteLayoutResponse.IcpCode.AsObject,
    languagesList: Array<string>,
    version: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }

  export class Author extends jspb.Message {
    getName(): string;
    setName(value: string): Author;

    getEmail(): string;
    setEmail(value: string): Author;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Author.AsObject;
    static toObject(includeInstance: boolean, msg: Author): Author.AsObject;
    static serializeBinaryToWriter(message: Author, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Author;
    static deserializeBinaryFromReader(message: Author, reader: jspb.BinaryReader): Author;
  }

  export namespace Author {
    export type AsObject = {
      name: string,
      email: string,
    }
  }


  export class GabCode extends jspb.Message {
    getCode(): string;
    setCode(value: string): GabCode;

    getName(): string;
    setName(value: string): GabCode;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GabCode.AsObject;
    static toObject(includeInstance: boolean, msg: GabCode): GabCode.AsObject;
    static serializeBinaryToWriter(message: GabCode, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GabCode;
    static deserializeBinaryFromReader(message: GabCode, reader: jspb.BinaryReader): GabCode;
  }

  export namespace GabCode {
    export type AsObject = {
      code: string,
      name: string,
    }
  }


  export class IcpCode extends jspb.Message {
    getCode(): string;
    setCode(value: string): IcpCode;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): IcpCode.AsObject;
    static toObject(includeInstance: boolean, msg: IcpCode): IcpCode.AsObject;
    static serializeBinaryToWriter(message: IcpCode, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): IcpCode;
    static deserializeBinaryFromReader(message: IcpCode, reader: jspb.BinaryReader): IcpCode;
  }

  export namespace IcpCode {
    export type AsObject = {
      code: string,
    }
  }

}

export class SiteSetKeywordsRequest extends jspb.Message {
  getItemsList(): Array<string>;
  setItemsList(value: Array<string>): SiteSetKeywordsRequest;
  clearItemsList(): SiteSetKeywordsRequest;
  addItems(value: string, index?: number): SiteSetKeywordsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteSetKeywordsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SiteSetKeywordsRequest): SiteSetKeywordsRequest.AsObject;
  static serializeBinaryToWriter(message: SiteSetKeywordsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteSetKeywordsRequest;
  static deserializeBinaryFromReader(message: SiteSetKeywordsRequest, reader: jspb.BinaryReader): SiteSetKeywordsRequest;
}

export namespace SiteSetKeywordsRequest {
  export type AsObject = {
    itemsList: Array<string>,
  }
}

export class SiteSetInfoRequest extends jspb.Message {
  getLang(): string;
  setLang(value: string): SiteSetInfoRequest;

  getTitle(): string;
  setTitle(value: string): SiteSetInfoRequest;

  getSubhead(): string;
  setSubhead(value: string): SiteSetInfoRequest;

  getDescription(): string;
  setDescription(value: string): SiteSetInfoRequest;

  getCopyright(): string;
  setCopyright(value: string): SiteSetInfoRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteSetInfoRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SiteSetInfoRequest): SiteSetInfoRequest.AsObject;
  static serializeBinaryToWriter(message: SiteSetInfoRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteSetInfoRequest;
  static deserializeBinaryFromReader(message: SiteSetInfoRequest, reader: jspb.BinaryReader): SiteSetInfoRequest;
}

export namespace SiteSetInfoRequest {
  export type AsObject = {
    lang: string,
    title: string,
    subhead: string,
    description: string,
    copyright: string,
  }
}

export class SiteStatusResponse extends jspb.Message {
  getPostgresql(): SiteStatusResponse.PostgreSql | undefined;
  setPostgresql(value?: SiteStatusResponse.PostgreSql): SiteStatusResponse;
  hasPostgresql(): boolean;
  clearPostgresql(): SiteStatusResponse;

  getRedis(): SiteStatusResponse.Redis | undefined;
  setRedis(value?: SiteStatusResponse.Redis): SiteStatusResponse;
  hasRedis(): boolean;
  clearRedis(): SiteStatusResponse;

  getOpensearch(): SiteStatusResponse.OpenSearch | undefined;
  setOpensearch(value?: SiteStatusResponse.OpenSearch): SiteStatusResponse;
  hasOpensearch(): boolean;
  clearOpensearch(): SiteStatusResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteStatusResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SiteStatusResponse): SiteStatusResponse.AsObject;
  static serializeBinaryToWriter(message: SiteStatusResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteStatusResponse;
  static deserializeBinaryFromReader(message: SiteStatusResponse, reader: jspb.BinaryReader): SiteStatusResponse;
}

export namespace SiteStatusResponse {
  export type AsObject = {
    postgresql?: SiteStatusResponse.PostgreSql.AsObject,
    redis?: SiteStatusResponse.Redis.AsObject,
    opensearch?: SiteStatusResponse.OpenSearch.AsObject,
  }

  export class PostgreSql extends jspb.Message {
    getMigrationsList(): Array<string>;
    setMigrationsList(value: Array<string>): PostgreSql;
    clearMigrationsList(): PostgreSql;
    addMigrations(value: string, index?: number): PostgreSql;

    getVersion(): string;
    setVersion(value: string): PostgreSql;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): PostgreSql.AsObject;
    static toObject(includeInstance: boolean, msg: PostgreSql): PostgreSql.AsObject;
    static serializeBinaryToWriter(message: PostgreSql, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): PostgreSql;
    static deserializeBinaryFromReader(message: PostgreSql, reader: jspb.BinaryReader): PostgreSql;
  }

  export namespace PostgreSql {
    export type AsObject = {
      migrationsList: Array<string>,
      version: string,
    }
  }


  export class Redis extends jspb.Message {
    getNodesList(): Array<string>;
    setNodesList(value: Array<string>): Redis;
    clearNodesList(): Redis;
    addNodes(value: string, index?: number): Redis;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Redis.AsObject;
    static toObject(includeInstance: boolean, msg: Redis): Redis.AsObject;
    static serializeBinaryToWriter(message: Redis, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Redis;
    static deserializeBinaryFromReader(message: Redis, reader: jspb.BinaryReader): Redis;
  }

  export namespace Redis {
    export type AsObject = {
      nodesList: Array<string>,
    }
  }


  export class OpenSearch extends jspb.Message {
    getVersion(): string;
    setVersion(value: string): OpenSearch;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): OpenSearch.AsObject;
    static toObject(includeInstance: boolean, msg: OpenSearch): OpenSearch.AsObject;
    static serializeBinaryToWriter(message: OpenSearch, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): OpenSearch;
    static deserializeBinaryFromReader(message: OpenSearch, reader: jspb.BinaryReader): OpenSearch;
  }

  export namespace OpenSearch {
    export type AsObject = {
      version: string,
    }
  }

}

export enum UserTokenAction { 
  SIGNIN = 0,
  CONFIRM = 1,
  RESETPASSWORD = 2,
  UNLOCK = 3,
  CANCEL = 9,
}
export enum Operation { 
  VIEW = 0,
  READ = 1,
  WRITE = 2,
  MANAGE = 3,
}
