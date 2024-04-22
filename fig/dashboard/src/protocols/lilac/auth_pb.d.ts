import * as jspb from 'google-protobuf'

import * as google_protobuf_duration_pb from 'google-protobuf/google/protobuf/duration_pb'; // proto import: "google/protobuf/duration.proto"
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"
import * as rbac_pb from './rbac_pb'; // proto import: "rbac.proto"


export class Pager extends jspb.Message {
  getPage(): number;
  setPage(value: number): Pager;

  getSize(): number;
  setSize(value: number): Pager;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Pager.AsObject;
  static toObject(includeInstance: boolean, msg: Pager): Pager.AsObject;
  static serializeBinaryToWriter(message: Pager, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Pager;
  static deserializeBinaryFromReader(message: Pager, reader: jspb.BinaryReader): Pager;
}

export namespace Pager {
  export type AsObject = {
    page: number,
    size: number,
  }
}

export class Pagination extends jspb.Message {
  getPage(): number;
  setPage(value: number): Pagination;

  getSize(): number;
  setSize(value: number): Pagination;

  getTotal(): number;
  setTotal(value: number): Pagination;

  getHasNext(): boolean;
  setHasNext(value: boolean): Pagination;

  getHasPrevious(): boolean;
  setHasPrevious(value: boolean): Pagination;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Pagination.AsObject;
  static toObject(includeInstance: boolean, msg: Pagination): Pagination.AsObject;
  static serializeBinaryToWriter(message: Pagination, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Pagination;
  static deserializeBinaryFromReader(message: Pagination, reader: jspb.BinaryReader): Pagination;
}

export namespace Pagination {
  export type AsObject = {
    page: number,
    size: number,
    total: number,
    hasNext: boolean,
    hasPrevious: boolean,
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

export class UserByEmailRequest extends jspb.Message {
  getUser(): EmailUserQuery | undefined;
  setUser(value?: EmailUserQuery): UserByEmailRequest;
  hasUser(): boolean;
  clearUser(): UserByEmailRequest;

  getHome(): string;
  setHome(value: string): UserByEmailRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserByEmailRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserByEmailRequest): UserByEmailRequest.AsObject;
  static serializeBinaryToWriter(message: UserByEmailRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserByEmailRequest;
  static deserializeBinaryFromReader(message: UserByEmailRequest, reader: jspb.BinaryReader): UserByEmailRequest;
}

export namespace UserByEmailRequest {
  export type AsObject = {
    user?: EmailUserQuery.AsObject,
    home: string,
  }
}

export class EmailUserQuery extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): EmailUserQuery;

  getNickname(): string;
  setNickname(value: string): EmailUserQuery;

  getByCase(): EmailUserQuery.ByCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserQuery.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserQuery): EmailUserQuery.AsObject;
  static serializeBinaryToWriter(message: EmailUserQuery, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserQuery;
  static deserializeBinaryFromReader(message: EmailUserQuery, reader: jspb.BinaryReader): EmailUserQuery;
}

export namespace EmailUserQuery {
  export type AsObject = {
    email: string,
    nickname: string,
  }

  export enum ByCase { 
    BY_NOT_SET = 0,
    EMAIL = 1,
    NICKNAME = 2,
  }
}

export class UserSignInByEmailRequest extends jspb.Message {
  getUser(): EmailUserQuery | undefined;
  setUser(value?: EmailUserQuery): UserSignInByEmailRequest;
  hasUser(): boolean;
  clearUser(): UserSignInByEmailRequest;

  getPassword(): string;
  setPassword(value: string): UserSignInByEmailRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): UserSignInByEmailRequest;
  hasTtl(): boolean;
  clearTtl(): UserSignInByEmailRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSignInByEmailRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserSignInByEmailRequest): UserSignInByEmailRequest.AsObject;
  static serializeBinaryToWriter(message: UserSignInByEmailRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSignInByEmailRequest;
  static deserializeBinaryFromReader(message: UserSignInByEmailRequest, reader: jspb.BinaryReader): UserSignInByEmailRequest;
}

export namespace UserSignInByEmailRequest {
  export type AsObject = {
    user?: EmailUserQuery.AsObject,
    password: string,
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class UserSignInResponse extends jspb.Message {
  getToken(): string;
  setToken(value: string): UserSignInResponse;

  getNickname(): string;
  setNickname(value: string): UserSignInResponse;

  getEmail(): string;
  setEmail(value: string): UserSignInResponse;

  getRealName(): string;
  setRealName(value: string): UserSignInResponse;

  getAvatar(): string;
  setAvatar(value: string): UserSignInResponse;

  getLang(): string;
  setLang(value: string): UserSignInResponse;

  getTimezone(): string;
  setTimezone(value: string): UserSignInResponse;

  getRolesList(): Array<rbac_pb.Role>;
  setRolesList(value: Array<rbac_pb.Role>): UserSignInResponse;
  clearRolesList(): UserSignInResponse;
  addRoles(value?: rbac_pb.Role, index?: number): rbac_pb.Role;

  getPermissionsList(): Array<rbac_pb.Permission>;
  setPermissionsList(value: Array<rbac_pb.Permission>): UserSignInResponse;
  clearPermissionsList(): UserSignInResponse;
  addPermissions(value?: rbac_pb.Permission, index?: number): rbac_pb.Permission;

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
    nickname: string,
    email: string,
    realName: string,
    avatar: string,
    lang: string,
    timezone: string,
    rolesList: Array<rbac_pb.Role.AsObject>,
    permissionsList: Array<rbac_pb.Permission.AsObject>,
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

  getLang(): string;
  setLang(value: string): UserSignUpByEmailRequest;

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
    lang: string,
    timezone: string,
    home: string,
  }
}

export class UserConfirmByTokenRequest extends jspb.Message {
  getToken(): string;
  setToken(value: string): UserConfirmByTokenRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserConfirmByTokenRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserConfirmByTokenRequest): UserConfirmByTokenRequest.AsObject;
  static serializeBinaryToWriter(message: UserConfirmByTokenRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserConfirmByTokenRequest;
  static deserializeBinaryFromReader(message: UserConfirmByTokenRequest, reader: jspb.BinaryReader): UserConfirmByTokenRequest;
}

export namespace UserConfirmByTokenRequest {
  export type AsObject = {
    token: string,
  }
}

export class UserUnlockByTokenRequest extends jspb.Message {
  getToken(): string;
  setToken(value: string): UserUnlockByTokenRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserUnlockByTokenRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserUnlockByTokenRequest): UserUnlockByTokenRequest.AsObject;
  static serializeBinaryToWriter(message: UserUnlockByTokenRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserUnlockByTokenRequest;
  static deserializeBinaryFromReader(message: UserUnlockByTokenRequest, reader: jspb.BinaryReader): UserUnlockByTokenRequest;
}

export namespace UserUnlockByTokenRequest {
  export type AsObject = {
    token: string,
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

    getResource(): rbac_pb.Resource | undefined;
    setResource(value?: rbac_pb.Resource): Item;
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
      resource?: rbac_pb.Resource.AsObject,
      message: string,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum Level { 
      INFO = 0,
      WARN = 1,
      ERROR = 2,
    }
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

export class UserUpdateProfileRequest extends jspb.Message {
  getRealName(): string;
  setRealName(value: string): UserUpdateProfileRequest;

  getAvatar(): string;
  setAvatar(value: string): UserUpdateProfileRequest;

  getLang(): string;
  setLang(value: string): UserUpdateProfileRequest;

  getTimezone(): string;
  setTimezone(value: string): UserUpdateProfileRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserUpdateProfileRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserUpdateProfileRequest): UserUpdateProfileRequest.AsObject;
  static serializeBinaryToWriter(message: UserUpdateProfileRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserUpdateProfileRequest;
  static deserializeBinaryFromReader(message: UserUpdateProfileRequest, reader: jspb.BinaryReader): UserUpdateProfileRequest;
}

export namespace UserUpdateProfileRequest {
  export type AsObject = {
    realName: string,
    avatar: string,
    lang: string,
    timezone: string,
  }
}

export class UserIndexRequest extends jspb.Message {
  getProviderType(): rbac_pb.UserDetail.Provider.Type;
  setProviderType(value: rbac_pb.UserDetail.Provider.Type): UserIndexRequest;

  getPager(): Pager | undefined;
  setPager(value?: Pager): UserIndexRequest;
  hasPager(): boolean;
  clearPager(): UserIndexRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserIndexRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserIndexRequest): UserIndexRequest.AsObject;
  static serializeBinaryToWriter(message: UserIndexRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserIndexRequest;
  static deserializeBinaryFromReader(message: UserIndexRequest, reader: jspb.BinaryReader): UserIndexRequest;
}

export namespace UserIndexRequest {
  export type AsObject = {
    providerType: rbac_pb.UserDetail.Provider.Type,
    pager?: Pager.AsObject,
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
    getDetail(): UserIndexResponse.Item.Detail | undefined;
    setDetail(value?: UserIndexResponse.Item.Detail): Item;
    hasDetail(): boolean;
    clearDetail(): Item;

    getLang(): string;
    setLang(value: string): Item;

    getTimezone(): string;
    setTimezone(value: string): Item;

    getSignInCount(): number;
    setSignInCount(value: number): Item;

    getCurrentSignedInIp(): string;
    setCurrentSignedInIp(value: string): Item;

    getCurrentSignedInAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setCurrentSignedInAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasCurrentSignedInAt(): boolean;
    clearCurrentSignedInAt(): Item;

    getLastSignedInIp(): string;
    setLastSignedInIp(value: string): Item;

    getLastSignedInAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setLastSignedInAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasLastSignedInAt(): boolean;
    clearLastSignedInAt(): Item;

    getLockedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setLockedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasLockedAt(): boolean;
    clearLockedAt(): Item;

    getDeletedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setDeletedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasDeletedAt(): boolean;
    clearDeletedAt(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      detail?: UserIndexResponse.Item.Detail.AsObject,
      lang: string,
      timezone: string,
      signInCount: number,
      currentSignedInIp: string,
      currentSignedInAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      lastSignedInIp: string,
      lastSignedInAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      lockedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export class Detail extends jspb.Message {
      getProviderType(): rbac_pb.UserDetail.Provider.Type;
      setProviderType(value: rbac_pb.UserDetail.Provider.Type): Detail;

      getProviderId(): string;
      setProviderId(value: string): Detail;

      getName(): string;
      setName(value: string): Detail;

      getAvatar(): string;
      setAvatar(value: string): Detail;

      getConfirmedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
      setConfirmedAt(value?: google_protobuf_timestamp_pb.Timestamp): Detail;
      hasConfirmedAt(): boolean;
      clearConfirmedAt(): Detail;

      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): Detail.AsObject;
      static toObject(includeInstance: boolean, msg: Detail): Detail.AsObject;
      static serializeBinaryToWriter(message: Detail, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): Detail;
      static deserializeBinaryFromReader(message: Detail, reader: jspb.BinaryReader): Detail;
    }

    export namespace Detail {
      export type AsObject = {
        providerType: rbac_pb.UserDetail.Provider.Type,
        providerId: string,
        name: string,
        avatar: string,
        confirmedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      }

      export enum ConfirmedAtCase { 
        _CONFIRMED_AT_NOT_SET = 0,
        CONFIRMED_AT = 9,
      }
    }


    export enum LockedAtCase { 
      _LOCKED_AT_NOT_SET = 0,
      LOCKED_AT = 31,
    }

    export enum DeletedAtCase { 
      _DELETED_AT_NOT_SET = 0,
      DELETED_AT = 32,
    }
  }

}

export class UserSessionsResponse extends jspb.Message {
  getItemsList(): Array<UserSessionsResponse.Item>;
  setItemsList(value: Array<UserSessionsResponse.Item>): UserSessionsResponse;
  clearItemsList(): UserSessionsResponse;
  addItems(value?: UserSessionsResponse.Item, index?: number): UserSessionsResponse.Item;

  getPagination(): Pagination | undefined;
  setPagination(value?: Pagination): UserSessionsResponse;
  hasPagination(): boolean;
  clearPagination(): UserSessionsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSessionsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserSessionsResponse): UserSessionsResponse.AsObject;
  static serializeBinaryToWriter(message: UserSessionsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSessionsResponse;
  static deserializeBinaryFromReader(message: UserSessionsResponse, reader: jspb.BinaryReader): UserSessionsResponse;
}

export namespace UserSessionsResponse {
  export type AsObject = {
    itemsList: Array<UserSessionsResponse.Item.AsObject>,
    pagination?: Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getUser(): UserIndexResponse.Item | undefined;
    setUser(value?: UserIndexResponse.Item): Item;
    hasUser(): boolean;
    clearUser(): Item;

    getUid(): string;
    setUid(value: string): Item;

    getIp(): string;
    setIp(value: string): Item;

    getExpiredAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setExpiredAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasExpiredAt(): boolean;
    clearExpiredAt(): Item;

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
      user?: UserIndexResponse.Item.AsObject,
      uid: string,
      ip: string,
      expiredAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
  }

}

export class UserSetPasswordRequest extends jspb.Message {
  getUser(): number;
  setUser(value: number): UserSetPasswordRequest;

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
    user: number,
    password: string,
  }
}

export class UserAttachmentsResponse extends jspb.Message {
  getItemsList(): Array<UserAttachmentsResponse.Item>;
  setItemsList(value: Array<UserAttachmentsResponse.Item>): UserAttachmentsResponse;
  clearItemsList(): UserAttachmentsResponse;
  addItems(value?: UserAttachmentsResponse.Item, index?: number): UserAttachmentsResponse.Item;

  getPagination(): Pagination | undefined;
  setPagination(value?: Pagination): UserAttachmentsResponse;
  hasPagination(): boolean;
  clearPagination(): UserAttachmentsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserAttachmentsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserAttachmentsResponse): UserAttachmentsResponse.AsObject;
  static serializeBinaryToWriter(message: UserAttachmentsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserAttachmentsResponse;
  static deserializeBinaryFromReader(message: UserAttachmentsResponse, reader: jspb.BinaryReader): UserAttachmentsResponse;
}

export namespace UserAttachmentsResponse {
  export type AsObject = {
    itemsList: Array<UserAttachmentsResponse.Item.AsObject>,
    pagination?: Pagination.AsObject,
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
      deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum DeletedAtCase { 
      _DELETED_AT_NOT_SET = 0,
      DELETED_AT = 8,
    }
  }

}

export class UserRefreshRequest extends jspb.Message {
  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): UserRefreshRequest;
  hasTtl(): boolean;
  clearTtl(): UserRefreshRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserRefreshRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserRefreshRequest): UserRefreshRequest.AsObject;
  static serializeBinaryToWriter(message: UserRefreshRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserRefreshRequest;
  static deserializeBinaryFromReader(message: UserRefreshRequest, reader: jspb.BinaryReader): UserRefreshRequest;
}

export namespace UserRefreshRequest {
  export type AsObject = {
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class UserRefreshResponse extends jspb.Message {
  getToken(): string;
  setToken(value: string): UserRefreshResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserRefreshResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserRefreshResponse): UserRefreshResponse.AsObject;
  static serializeBinaryToWriter(message: UserRefreshResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserRefreshResponse;
  static deserializeBinaryFromReader(message: UserRefreshResponse, reader: jspb.BinaryReader): UserRefreshResponse;
}

export namespace UserRefreshResponse {
  export type AsObject = {
    token: string,
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
  getItemsList(): Array<LocaleByLangResponse.Item>;
  setItemsList(value: Array<LocaleByLangResponse.Item>): LocaleByLangResponse;
  clearItemsList(): LocaleByLangResponse;
  addItems(value?: LocaleByLangResponse.Item, index?: number): LocaleByLangResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LocaleByLangResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LocaleByLangResponse): LocaleByLangResponse.AsObject;
  static serializeBinaryToWriter(message: LocaleByLangResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LocaleByLangResponse;
  static deserializeBinaryFromReader(message: LocaleByLangResponse, reader: jspb.BinaryReader): LocaleByLangResponse;
}

export namespace LocaleByLangResponse {
  export type AsObject = {
    itemsList: Array<LocaleByLangResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getCode(): string;
    setCode(value: string): Item;

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
      code: string,
      message: string,
    }
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

export class KvSetRequest extends jspb.Message {
  getKey(): string;
  setKey(value: string): KvSetRequest;

  getValue(): Uint8Array | string;
  getValue_asU8(): Uint8Array;
  getValue_asB64(): string;
  setValue(value: Uint8Array | string): KvSetRequest;

  getEncrypt(): boolean;
  setEncrypt(value: boolean): KvSetRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): KvSetRequest.AsObject;
  static toObject(includeInstance: boolean, msg: KvSetRequest): KvSetRequest.AsObject;
  static serializeBinaryToWriter(message: KvSetRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): KvSetRequest;
  static deserializeBinaryFromReader(message: KvSetRequest, reader: jspb.BinaryReader): KvSetRequest;
}

export namespace KvSetRequest {
  export type AsObject = {
    key: string,
    value: Uint8Array | string,
    encrypt: boolean,
  }
}

export class KvGetRequest extends jspb.Message {
  getKey(): string;
  setKey(value: string): KvGetRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): KvGetRequest.AsObject;
  static toObject(includeInstance: boolean, msg: KvGetRequest): KvGetRequest.AsObject;
  static serializeBinaryToWriter(message: KvGetRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): KvGetRequest;
  static deserializeBinaryFromReader(message: KvGetRequest, reader: jspb.BinaryReader): KvGetRequest;
}

export namespace KvGetRequest {
  export type AsObject = {
    key: string,
  }
}

export class KvGetResponse extends jspb.Message {
  getValue(): Uint8Array | string;
  getValue_asU8(): Uint8Array;
  getValue_asB64(): string;
  setValue(value: Uint8Array | string): KvGetResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): KvGetResponse.AsObject;
  static toObject(includeInstance: boolean, msg: KvGetResponse): KvGetResponse.AsObject;
  static serializeBinaryToWriter(message: KvGetResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): KvGetResponse;
  static deserializeBinaryFromReader(message: KvGetResponse, reader: jspb.BinaryReader): KvGetResponse;
}

export namespace KvGetResponse {
  export type AsObject = {
    value: Uint8Array | string,
  }
}

export class SiteIcpCode extends jspb.Message {
  getCode(): string;
  setCode(value: string): SiteIcpCode;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteIcpCode.AsObject;
  static toObject(includeInstance: boolean, msg: SiteIcpCode): SiteIcpCode.AsObject;
  static serializeBinaryToWriter(message: SiteIcpCode, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteIcpCode;
  static deserializeBinaryFromReader(message: SiteIcpCode, reader: jspb.BinaryReader): SiteIcpCode;
}

export namespace SiteIcpCode {
  export type AsObject = {
    code: string,
  }
}

export class SiteGabCode extends jspb.Message {
  getCode(): string;
  setCode(value: string): SiteGabCode;

  getName(): string;
  setName(value: string): SiteGabCode;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteGabCode.AsObject;
  static toObject(includeInstance: boolean, msg: SiteGabCode): SiteGabCode.AsObject;
  static serializeBinaryToWriter(message: SiteGabCode, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteGabCode;
  static deserializeBinaryFromReader(message: SiteGabCode, reader: jspb.BinaryReader): SiteGabCode;
}

export namespace SiteGabCode {
  export type AsObject = {
    code: string,
    name: string,
  }
}

export class SiteBaidu extends jspb.Message {
  getCode(): string;
  setCode(value: string): SiteBaidu;

  getContent(): string;
  setContent(value: string): SiteBaidu;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteBaidu.AsObject;
  static toObject(includeInstance: boolean, msg: SiteBaidu): SiteBaidu.AsObject;
  static serializeBinaryToWriter(message: SiteBaidu, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteBaidu;
  static deserializeBinaryFromReader(message: SiteBaidu, reader: jspb.BinaryReader): SiteBaidu;
}

export namespace SiteBaidu {
  export type AsObject = {
    code: string,
    content: string,
  }
}

export class SiteGoogle extends jspb.Message {
  getCode(): string;
  setCode(value: string): SiteGoogle;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteGoogle.AsObject;
  static toObject(includeInstance: boolean, msg: SiteGoogle): SiteGoogle.AsObject;
  static serializeBinaryToWriter(message: SiteGoogle, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteGoogle;
  static deserializeBinaryFromReader(message: SiteGoogle, reader: jspb.BinaryReader): SiteGoogle;
}

export namespace SiteGoogle {
  export type AsObject = {
    code: string,
  }
}

export class SiteIndexNow extends jspb.Message {
  getKey(): string;
  setKey(value: string): SiteIndexNow;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteIndexNow.AsObject;
  static toObject(includeInstance: boolean, msg: SiteIndexNow): SiteIndexNow.AsObject;
  static serializeBinaryToWriter(message: SiteIndexNow, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteIndexNow;
  static deserializeBinaryFromReader(message: SiteIndexNow, reader: jspb.BinaryReader): SiteIndexNow;
}

export namespace SiteIndexNow {
  export type AsObject = {
    key: string,
  }
}

export class SiteGoogleReCaptcha extends jspb.Message {
  getSiteKey(): string;
  setSiteKey(value: string): SiteGoogleReCaptcha;

  getSecret(): string;
  setSecret(value: string): SiteGoogleReCaptcha;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteGoogleReCaptcha.AsObject;
  static toObject(includeInstance: boolean, msg: SiteGoogleReCaptcha): SiteGoogleReCaptcha.AsObject;
  static serializeBinaryToWriter(message: SiteGoogleReCaptcha, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteGoogleReCaptcha;
  static deserializeBinaryFromReader(message: SiteGoogleReCaptcha, reader: jspb.BinaryReader): SiteGoogleReCaptcha;
}

export namespace SiteGoogleReCaptcha {
  export type AsObject = {
    siteKey: string,
    secret: string,
  }
}

export class SiteAuthors extends jspb.Message {
  getItesList(): Array<SiteAuthors.Item>;
  setItesList(value: Array<SiteAuthors.Item>): SiteAuthors;
  clearItesList(): SiteAuthors;
  addItes(value?: SiteAuthors.Item, index?: number): SiteAuthors.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteAuthors.AsObject;
  static toObject(includeInstance: boolean, msg: SiteAuthors): SiteAuthors.AsObject;
  static serializeBinaryToWriter(message: SiteAuthors, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteAuthors;
  static deserializeBinaryFromReader(message: SiteAuthors, reader: jspb.BinaryReader): SiteAuthors;
}

export namespace SiteAuthors {
  export type AsObject = {
    itesList: Array<SiteAuthors.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getName(): string;
    setName(value: string): Item;

    getEmail(): string;
    setEmail(value: string): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      name: string,
      email: string,
    }
  }

}

export class SiteKeywords extends jspb.Message {
  getItemsList(): Array<string>;
  setItemsList(value: Array<string>): SiteKeywords;
  clearItemsList(): SiteKeywords;
  addItems(value: string, index?: number): SiteKeywords;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteKeywords.AsObject;
  static toObject(includeInstance: boolean, msg: SiteKeywords): SiteKeywords.AsObject;
  static serializeBinaryToWriter(message: SiteKeywords, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteKeywords;
  static deserializeBinaryFromReader(message: SiteKeywords, reader: jspb.BinaryReader): SiteKeywords;
}

export namespace SiteKeywords {
  export type AsObject = {
    itemsList: Array<string>,
  }
}

export class SiteInfo extends jspb.Message {
  getTitle(): string;
  setTitle(value: string): SiteInfo;

  getSubhead(): string;
  setSubhead(value: string): SiteInfo;

  getDescription(): string;
  setDescription(value: string): SiteInfo;

  getCopyright(): string;
  setCopyright(value: string): SiteInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteInfo.AsObject;
  static toObject(includeInstance: boolean, msg: SiteInfo): SiteInfo.AsObject;
  static serializeBinaryToWriter(message: SiteInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteInfo;
  static deserializeBinaryFromReader(message: SiteInfo, reader: jspb.BinaryReader): SiteInfo;
}

export namespace SiteInfo {
  export type AsObject = {
    title: string,
    subhead: string,
    description: string,
    copyright: string,
  }
}

export class SiteFavicon extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): SiteFavicon;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SiteFavicon.AsObject;
  static toObject(includeInstance: boolean, msg: SiteFavicon): SiteFavicon.AsObject;
  static serializeBinaryToWriter(message: SiteFavicon, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SiteFavicon;
  static deserializeBinaryFromReader(message: SiteFavicon, reader: jspb.BinaryReader): SiteFavicon;
}

export namespace SiteFavicon {
  export type AsObject = {
    url: string,
  }
}

export class LeaveWordCreateRequest extends jspb.Message {
  getContent(): string;
  setContent(value: string): LeaveWordCreateRequest;

  getEditor(): Editor;
  setEditor(value: Editor): LeaveWordCreateRequest;

  getIp(): string;
  setIp(value: string): LeaveWordCreateRequest;

  getLang(): string;
  setLang(value: string): LeaveWordCreateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LeaveWordCreateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LeaveWordCreateRequest): LeaveWordCreateRequest.AsObject;
  static serializeBinaryToWriter(message: LeaveWordCreateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LeaveWordCreateRequest;
  static deserializeBinaryFromReader(message: LeaveWordCreateRequest, reader: jspb.BinaryReader): LeaveWordCreateRequest;
}

export namespace LeaveWordCreateRequest {
  export type AsObject = {
    content: string,
    editor: Editor,
    ip: string,
    lang: string,
  }
}

export class LeaveWordIndexResponse extends jspb.Message {
  getItemsList(): Array<LeaveWordIndexResponse.Item>;
  setItemsList(value: Array<LeaveWordIndexResponse.Item>): LeaveWordIndexResponse;
  clearItemsList(): LeaveWordIndexResponse;
  addItems(value?: LeaveWordIndexResponse.Item, index?: number): LeaveWordIndexResponse.Item;

  getPagination(): Pagination | undefined;
  setPagination(value?: Pagination): LeaveWordIndexResponse;
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
    pagination?: Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getIp(): string;
    setIp(value: string): Item;

    getLang(): string;
    setLang(value: string): Item;

    getContent(): string;
    setContent(value: string): Item;

    getEditor(): Editor;
    setEditor(value: Editor): Item;

    getPublishedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setPublishedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasPublishedAt(): boolean;
    clearPublishedAt(): Item;

    getDeletedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setDeletedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasDeletedAt(): boolean;
    clearDeletedAt(): Item;

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
      ip: string,
      lang: string,
      content: string,
      editor: Editor,
      publishedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
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

export class TagCreateRequest extends jspb.Message {
  getName(): string;
  setName(value: string): TagCreateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TagCreateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: TagCreateRequest): TagCreateRequest.AsObject;
  static serializeBinaryToWriter(message: TagCreateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TagCreateRequest;
  static deserializeBinaryFromReader(message: TagCreateRequest, reader: jspb.BinaryReader): TagCreateRequest;
}

export namespace TagCreateRequest {
  export type AsObject = {
    name: string,
  }
}

export class TagEditRequest extends jspb.Message {
  getId(): number;
  setId(value: number): TagEditRequest;

  getName(): string;
  setName(value: string): TagEditRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TagEditRequest.AsObject;
  static toObject(includeInstance: boolean, msg: TagEditRequest): TagEditRequest.AsObject;
  static serializeBinaryToWriter(message: TagEditRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TagEditRequest;
  static deserializeBinaryFromReader(message: TagEditRequest, reader: jspb.BinaryReader): TagEditRequest;
}

export namespace TagEditRequest {
  export type AsObject = {
    id: number,
    name: string,
  }
}

export class TagIndexResponse extends jspb.Message {
  getItemsList(): Array<TagIndexResponse.Item>;
  setItemsList(value: Array<TagIndexResponse.Item>): TagIndexResponse;
  clearItemsList(): TagIndexResponse;
  addItems(value?: TagIndexResponse.Item, index?: number): TagIndexResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TagIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: TagIndexResponse): TagIndexResponse.AsObject;
  static serializeBinaryToWriter(message: TagIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TagIndexResponse;
  static deserializeBinaryFromReader(message: TagIndexResponse, reader: jspb.BinaryReader): TagIndexResponse;
}

export namespace TagIndexResponse {
  export type AsObject = {
    itemsList: Array<TagIndexResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getName(): string;
    setName(value: string): Item;

    getSortOrder(): number;
    setSortOrder(value: number): Item;

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
      name: string,
      sortOrder: number,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
  }

}

export class CategoryCreateRequest extends jspb.Message {
  getCode(): string;
  setCode(value: string): CategoryCreateRequest;

  getLeft(): number;
  setLeft(value: number): CategoryCreateRequest;

  getRight(): number;
  setRight(value: number): CategoryCreateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CategoryCreateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CategoryCreateRequest): CategoryCreateRequest.AsObject;
  static serializeBinaryToWriter(message: CategoryCreateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CategoryCreateRequest;
  static deserializeBinaryFromReader(message: CategoryCreateRequest, reader: jspb.BinaryReader): CategoryCreateRequest;
}

export namespace CategoryCreateRequest {
  export type AsObject = {
    code: string,
    left: number,
    right: number,
  }
}

export class CategoryEditRequest extends jspb.Message {
  getId(): number;
  setId(value: number): CategoryEditRequest;

  getCode(): string;
  setCode(value: string): CategoryEditRequest;

  getLeft(): number;
  setLeft(value: number): CategoryEditRequest;

  getRight(): number;
  setRight(value: number): CategoryEditRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CategoryEditRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CategoryEditRequest): CategoryEditRequest.AsObject;
  static serializeBinaryToWriter(message: CategoryEditRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CategoryEditRequest;
  static deserializeBinaryFromReader(message: CategoryEditRequest, reader: jspb.BinaryReader): CategoryEditRequest;
}

export namespace CategoryEditRequest {
  export type AsObject = {
    id: number,
    code: string,
    left: number,
    right: number,
  }
}

export class CategoryIndexResponse extends jspb.Message {
  getItemsList(): Array<CategoryIndexResponse.Item>;
  setItemsList(value: Array<CategoryIndexResponse.Item>): CategoryIndexResponse;
  clearItemsList(): CategoryIndexResponse;
  addItems(value?: CategoryIndexResponse.Item, index?: number): CategoryIndexResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CategoryIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CategoryIndexResponse): CategoryIndexResponse.AsObject;
  static serializeBinaryToWriter(message: CategoryIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CategoryIndexResponse;
  static deserializeBinaryFromReader(message: CategoryIndexResponse, reader: jspb.BinaryReader): CategoryIndexResponse;
}

export namespace CategoryIndexResponse {
  export type AsObject = {
    itemsList: Array<CategoryIndexResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getCode(): string;
    setCode(value: string): Item;

    getLeft(): number;
    setLeft(value: number): Item;

    getRight(): number;
    setRight(value: number): Item;

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
      code: string,
      left: number,
      right: number,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
  }

}

export class NotificationCreateRequest extends jspb.Message {
  getTitle(): string;
  setTitle(value: string): NotificationCreateRequest;

  getBody(): string;
  setBody(value: string): NotificationCreateRequest;

  getSummary(): string;
  setSummary(value: string): NotificationCreateRequest;

  getUrl(): string;
  setUrl(value: string): NotificationCreateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): NotificationCreateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: NotificationCreateRequest): NotificationCreateRequest.AsObject;
  static serializeBinaryToWriter(message: NotificationCreateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): NotificationCreateRequest;
  static deserializeBinaryFromReader(message: NotificationCreateRequest, reader: jspb.BinaryReader): NotificationCreateRequest;
}

export namespace NotificationCreateRequest {
  export type AsObject = {
    title: string,
    body: string,
    summary: string,
    url: string,
  }
}

export class NotificationIndexResponse extends jspb.Message {
  getItemsList(): Array<NotificationIndexResponse.Item>;
  setItemsList(value: Array<NotificationIndexResponse.Item>): NotificationIndexResponse;
  clearItemsList(): NotificationIndexResponse;
  addItems(value?: NotificationIndexResponse.Item, index?: number): NotificationIndexResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): NotificationIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: NotificationIndexResponse): NotificationIndexResponse.AsObject;
  static serializeBinaryToWriter(message: NotificationIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): NotificationIndexResponse;
  static deserializeBinaryFromReader(message: NotificationIndexResponse, reader: jspb.BinaryReader): NotificationIndexResponse;
}

export namespace NotificationIndexResponse {
  export type AsObject = {
    itemsList: Array<NotificationIndexResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getTitle(): string;
    setTitle(value: string): Item;

    getBody(): string;
    setBody(value: string): Item;

    getSummary(): string;
    setSummary(value: string): Item;

    getUrl(): string;
    setUrl(value: string): Item;

    getReadAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setReadAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasReadAt(): boolean;
    clearReadAt(): Item;

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
      title: string,
      body: string,
      summary: string,
      url: string,
      readAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum ReadAtCase { 
      _READ_AT_NOT_SET = 0,
      READ_AT = 8,
    }
  }

}

export enum Editor { 
  TEXTAREA = 0,
  QUILL = 1,
}
