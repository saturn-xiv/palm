import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"
import * as google_protobuf_duration_pb from 'google-protobuf/google/protobuf/duration_pb'; // proto import: "google/protobuf/duration.proto"


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
    lang: string;
    code: string;
    message: string;
  };
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
    lang: string;
  };
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
    itemsList: Array<LocaleIndexResponse.Item.AsObject>;
  };
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
    itemsList: Array<LocaleIndexResponse.Item.AsObject>;
    pagination?: Pagination.AsObject;
  };

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
      id: number;
      lang: string;
      code: string;
      message: string;
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
    };
  }

}

export class CurrenciesResponse extends jspb.Message {
  getItemsList(): Array<CurrenciesResponse.Item>;
  setItemsList(value: Array<CurrenciesResponse.Item>): CurrenciesResponse;
  clearItemsList(): CurrenciesResponse;
  addItems(value?: CurrenciesResponse.Item, index?: number): CurrenciesResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CurrenciesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CurrenciesResponse): CurrenciesResponse.AsObject;
  static serializeBinaryToWriter(message: CurrenciesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CurrenciesResponse;
  static deserializeBinaryFromReader(message: CurrenciesResponse, reader: jspb.BinaryReader): CurrenciesResponse;
}

export namespace CurrenciesResponse {
  export type AsObject = {
    itemsList: Array<CurrenciesResponse.Item.AsObject>;
  };

  export class Item extends jspb.Message {
    getCode(): string;
    setCode(value: string): Item;

    getName(): string;
    setName(value: string): Item;

    getCountry(): string;
    setCountry(value: string): Item;

    getNumber(): number;
    setNumber(value: number): Item;

    getUnits(): number;
    setUnits(value: number): Item;
    hasUnits(): boolean;
    clearUnits(): Item;

    getIsFund(): boolean;
    setIsFund(value: boolean): Item;
    hasIsFund(): boolean;
    clearIsFund(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      code: string;
      name: string;
      country: string;
      number: number;
      units?: number;
      isFund?: boolean;
    };

    export enum UnitsCase {
      _UNITS_NOT_SET = 0,
      UNITS = 5,
    }

    export enum IsFundCase {
      _IS_FUND_NOT_SET = 0,
      IS_FUND = 6,
    }
  }

}

export class EmailUserSignInRequest extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): EmailUserSignInRequest;

  getPassword(): string;
  setPassword(value: string): EmailUserSignInRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserSignInRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserSignInRequest): EmailUserSignInRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserSignInRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserSignInRequest;
  static deserializeBinaryFromReader(message: EmailUserSignInRequest, reader: jspb.BinaryReader): EmailUserSignInRequest;
}

export namespace EmailUserSignInRequest {
  export type AsObject = {
    email: string;
    password: string;
  };
}

export class EmailUserSignUpRequest extends jspb.Message {
  getName(): string;
  setName(value: string): EmailUserSignUpRequest;

  getEmail(): string;
  setEmail(value: string): EmailUserSignUpRequest;

  getPassword(): string;
  setPassword(value: string): EmailUserSignUpRequest;

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
    name: string;
    email: string;
    password: string;
    lang: string;
    timezone: string;
    home: string;
  };
}

export class EmailUserUnlockByEmailRequest extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): EmailUserUnlockByEmailRequest;

  getHome(): string;
  setHome(value: string): EmailUserUnlockByEmailRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserUnlockByEmailRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserUnlockByEmailRequest): EmailUserUnlockByEmailRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserUnlockByEmailRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserUnlockByEmailRequest;
  static deserializeBinaryFromReader(message: EmailUserUnlockByEmailRequest, reader: jspb.BinaryReader): EmailUserUnlockByEmailRequest;
}

export namespace EmailUserUnlockByEmailRequest {
  export type AsObject = {
    email: string;
    home: string;
  };
}

export class EmailUserUnlockByTokenRequest extends jspb.Message {
  getToken(): string;
  setToken(value: string): EmailUserUnlockByTokenRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserUnlockByTokenRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserUnlockByTokenRequest): EmailUserUnlockByTokenRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserUnlockByTokenRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserUnlockByTokenRequest;
  static deserializeBinaryFromReader(message: EmailUserUnlockByTokenRequest, reader: jspb.BinaryReader): EmailUserUnlockByTokenRequest;
}

export namespace EmailUserUnlockByTokenRequest {
  export type AsObject = {
    token: string;
  };
}

export class EmailUserConfirmByEmailRequest extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): EmailUserConfirmByEmailRequest;

  getHome(): string;
  setHome(value: string): EmailUserConfirmByEmailRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserConfirmByEmailRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserConfirmByEmailRequest): EmailUserConfirmByEmailRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserConfirmByEmailRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserConfirmByEmailRequest;
  static deserializeBinaryFromReader(message: EmailUserConfirmByEmailRequest, reader: jspb.BinaryReader): EmailUserConfirmByEmailRequest;
}

export namespace EmailUserConfirmByEmailRequest {
  export type AsObject = {
    email: string;
    home: string;
  };
}

export class EmailUserConfirmByTokenRequest extends jspb.Message {
  getToken(): string;
  setToken(value: string): EmailUserConfirmByTokenRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserConfirmByTokenRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserConfirmByTokenRequest): EmailUserConfirmByTokenRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserConfirmByTokenRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserConfirmByTokenRequest;
  static deserializeBinaryFromReader(message: EmailUserConfirmByTokenRequest, reader: jspb.BinaryReader): EmailUserConfirmByTokenRequest;
}

export namespace EmailUserConfirmByTokenRequest {
  export type AsObject = {
    token: string;
  };
}

export class EmailUserForgotPasswordRequest extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): EmailUserForgotPasswordRequest;

  getHome(): string;
  setHome(value: string): EmailUserForgotPasswordRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserForgotPasswordRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserForgotPasswordRequest): EmailUserForgotPasswordRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserForgotPasswordRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserForgotPasswordRequest;
  static deserializeBinaryFromReader(message: EmailUserForgotPasswordRequest, reader: jspb.BinaryReader): EmailUserForgotPasswordRequest;
}

export namespace EmailUserForgotPasswordRequest {
  export type AsObject = {
    email: string;
    home: string;
  };
}

export class EmailUserResetPasswordRequest extends jspb.Message {
  getToken(): string;
  setToken(value: string): EmailUserResetPasswordRequest;

  getPassword(): string;
  setPassword(value: string): EmailUserResetPasswordRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserResetPasswordRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserResetPasswordRequest): EmailUserResetPasswordRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserResetPasswordRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserResetPasswordRequest;
  static deserializeBinaryFromReader(message: EmailUserResetPasswordRequest, reader: jspb.BinaryReader): EmailUserResetPasswordRequest;
}

export namespace EmailUserResetPasswordRequest {
  export type AsObject = {
    token: string;
    password: string;
  };
}

export class EmailUserSetNameRequest extends jspb.Message {
  getName(): string;
  setName(value: string): EmailUserSetNameRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserSetNameRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserSetNameRequest): EmailUserSetNameRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserSetNameRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserSetNameRequest;
  static deserializeBinaryFromReader(message: EmailUserSetNameRequest, reader: jspb.BinaryReader): EmailUserSetNameRequest;
}

export namespace EmailUserSetNameRequest {
  export type AsObject = {
    name: string;
  };
}

export class EmailUserSetAvatarRequest extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): EmailUserSetAvatarRequest;
  hasUrl(): boolean;
  clearUrl(): EmailUserSetAvatarRequest;

  getFile(): File | undefined;
  setFile(value?: File): EmailUserSetAvatarRequest;
  hasFile(): boolean;
  clearFile(): EmailUserSetAvatarRequest;

  getByCase(): EmailUserSetAvatarRequest.ByCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserSetAvatarRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserSetAvatarRequest): EmailUserSetAvatarRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserSetAvatarRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserSetAvatarRequest;
  static deserializeBinaryFromReader(message: EmailUserSetAvatarRequest, reader: jspb.BinaryReader): EmailUserSetAvatarRequest;
}

export namespace EmailUserSetAvatarRequest {
  export type AsObject = {
    url?: string;
    file?: File.AsObject;
  };

  export enum ByCase {
    BY_NOT_SET = 0,
    URL = 1,
    FILE = 2,
  }
}

export class EmailUserChangePasswordRequest extends jspb.Message {
  getCurrentPassword(): string;
  setCurrentPassword(value: string): EmailUserChangePasswordRequest;

  getNewPassword(): string;
  setNewPassword(value: string): EmailUserChangePasswordRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserChangePasswordRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserChangePasswordRequest): EmailUserChangePasswordRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserChangePasswordRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserChangePasswordRequest;
  static deserializeBinaryFromReader(message: EmailUserChangePasswordRequest, reader: jspb.BinaryReader): EmailUserChangePasswordRequest;
}

export namespace EmailUserChangePasswordRequest {
  export type AsObject = {
    currentPassword: string;
    newPassword: string;
  };
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
    itemsList: Array<EmailUserIndexResponse.Item.AsObject>;
    pagination?: Pagination.AsObject;
  };

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getEmail(): string;
    setEmail(value: string): Item;

    getName(): string;
    setName(value: string): Item;

    getAvatar(): string;
    setAvatar(value: string): Item;
    hasAvatar(): boolean;
    clearAvatar(): Item;

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
      id: number;
      email: string;
      name: string;
      avatar?: string;
      confirmedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
      deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
      user?: UserIndexResponse.Item.AsObject;
    };

    export enum AvatarCase {
      _AVATAR_NOT_SET = 0,
      AVATAR = 16,
    }

    export enum ConfirmedAtCase {
      _CONFIRMED_AT_NOT_SET = 0,
      CONFIRMED_AT = 17,
    }

    export enum DeletedAtCase {
      _DELETED_AT_NOT_SET = 0,
      DELETED_AT = 18,
    }
  }

}

export class EmailUserSetPasswordRequest extends jspb.Message {
  getId(): number;
  setId(value: number): EmailUserSetPasswordRequest;

  getPassword(): string;
  setPassword(value: string): EmailUserSetPasswordRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailUserSetPasswordRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailUserSetPasswordRequest): EmailUserSetPasswordRequest.AsObject;
  static serializeBinaryToWriter(message: EmailUserSetPasswordRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailUserSetPasswordRequest;
  static deserializeBinaryFromReader(message: EmailUserSetPasswordRequest, reader: jspb.BinaryReader): EmailUserSetPasswordRequest;
}

export namespace EmailUserSetPasswordRequest {
  export type AsObject = {
    id: number;
    password: string;
  };
}

export class UserSetLangRequest extends jspb.Message {
  getLang(): string;
  setLang(value: string): UserSetLangRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSetLangRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserSetLangRequest): UserSetLangRequest.AsObject;
  static serializeBinaryToWriter(message: UserSetLangRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSetLangRequest;
  static deserializeBinaryFromReader(message: UserSetLangRequest, reader: jspb.BinaryReader): UserSetLangRequest;
}

export namespace UserSetLangRequest {
  export type AsObject = {
    lang: string;
  };
}

export class UserSetTimezoneRequest extends jspb.Message {
  getTimezone(): string;
  setTimezone(value: string): UserSetTimezoneRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSetTimezoneRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserSetTimezoneRequest): UserSetTimezoneRequest.AsObject;
  static serializeBinaryToWriter(message: UserSetTimezoneRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSetTimezoneRequest;
  static deserializeBinaryFromReader(message: UserSetTimezoneRequest, reader: jspb.BinaryReader): UserSetTimezoneRequest;
}

export namespace UserSetTimezoneRequest {
  export type AsObject = {
    timezone: string;
  };
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

    getSn(): string;
    setSn(value: string): Item;

    getLang(): string;
    setLang(value: string): Item;

    getTimezone(): string;
    setTimezone(value: string): Item;

    getSignedInTotal(): number;
    setSignedInTotal(value: number): Item;

    getCurrentSignedInAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setCurrentSignedInAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasCurrentSignedInAt(): boolean;
    clearCurrentSignedInAt(): Item;

    getCurrentSignedInIp(): string;
    setCurrentSignedInIp(value: string): Item;
    hasCurrentSignedInIp(): boolean;
    clearCurrentSignedInIp(): Item;

    getLastSignedInAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setLastSignedInAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasLastSignedInAt(): boolean;
    clearLastSignedInAt(): Item;

    getLastSignedInIp(): string;
    setLastSignedInIp(value: string): Item;
    hasLastSignedInIp(): boolean;
    clearLastSignedInIp(): Item;

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
      id: number;
      sn: string;
      lang: string;
      timezone: string;
      signedInTotal: number;
      currentSignedInAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
      currentSignedInIp?: string;
      lastSignedInAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
      lastSignedInIp?: string;
      lockedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
      deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
    };

    export enum CurrentSignedInAtCase {
      _CURRENT_SIGNED_IN_AT_NOT_SET = 0,
      CURRENT_SIGNED_IN_AT = 12,
    }

    export enum CurrentSignedInIpCase {
      _CURRENT_SIGNED_IN_IP_NOT_SET = 0,
      CURRENT_SIGNED_IN_IP = 13,
    }

    export enum LastSignedInAtCase {
      _LAST_SIGNED_IN_AT_NOT_SET = 0,
      LAST_SIGNED_IN_AT = 14,
    }

    export enum LastSignedInIpCase {
      _LAST_SIGNED_IN_IP_NOT_SET = 0,
      LAST_SIGNED_IN_IP = 15,
    }

    export enum LockedAtCase {
      _LOCKED_AT_NOT_SET = 0,
      LOCKED_AT = 17,
    }

    export enum DeletedAtCase {
      _DELETED_AT_NOT_SET = 0,
      DELETED_AT = 18,
    }
  }

}

export class UserSetAttachmentTitleRequest extends jspb.Message {
  getId(): number;
  setId(value: number): UserSetAttachmentTitleRequest;

  getTitle(): string;
  setTitle(value: string): UserSetAttachmentTitleRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSetAttachmentTitleRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserSetAttachmentTitleRequest): UserSetAttachmentTitleRequest.AsObject;
  static serializeBinaryToWriter(message: UserSetAttachmentTitleRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSetAttachmentTitleRequest;
  static deserializeBinaryFromReader(message: UserSetAttachmentTitleRequest, reader: jspb.BinaryReader): UserSetAttachmentTitleRequest;
}

export namespace UserSetAttachmentTitleRequest {
  export type AsObject = {
    id: number;
    title: string;
  };
}

export class UserShowAttachmentRequest extends jspb.Message {
  getId(): number;
  setId(value: number): UserShowAttachmentRequest;

  getDownload(): boolean;
  setDownload(value: boolean): UserShowAttachmentRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): UserShowAttachmentRequest;
  hasTtl(): boolean;
  clearTtl(): UserShowAttachmentRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserShowAttachmentRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserShowAttachmentRequest): UserShowAttachmentRequest.AsObject;
  static serializeBinaryToWriter(message: UserShowAttachmentRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserShowAttachmentRequest;
  static deserializeBinaryFromReader(message: UserShowAttachmentRequest, reader: jspb.BinaryReader): UserShowAttachmentRequest;
}

export namespace UserShowAttachmentRequest {
  export type AsObject = {
    id: number;
    download: boolean;
    ttl?: google_protobuf_duration_pb.Duration.AsObject;
  };

  export enum TtlCase {
    _TTL_NOT_SET = 0,
    TTL = 9,
  }
}

export class UserShowAttachmentResponse extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): UserShowAttachmentResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserShowAttachmentResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserShowAttachmentResponse): UserShowAttachmentResponse.AsObject;
  static serializeBinaryToWriter(message: UserShowAttachmentResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserShowAttachmentResponse;
  static deserializeBinaryFromReader(message: UserShowAttachmentResponse, reader: jspb.BinaryReader): UserShowAttachmentResponse;
}

export namespace UserShowAttachmentResponse {
  export type AsObject = {
    url: string;
  };
}

export class UserCreateAttachmentRequest extends jspb.Message {
  getTitle(): string;
  setTitle(value: string): UserCreateAttachmentRequest;

  getContentType(): string;
  setContentType(value: string): UserCreateAttachmentRequest;

  getSize(): number;
  setSize(value: number): UserCreateAttachmentRequest;

  getExpireAfterDays(): number;
  setExpireAfterDays(value: number): UserCreateAttachmentRequest;
  hasExpireAfterDays(): boolean;
  clearExpireAfterDays(): UserCreateAttachmentRequest;

  getPublic(): boolean;
  setPublic(value: boolean): UserCreateAttachmentRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): UserCreateAttachmentRequest;
  hasTtl(): boolean;
  clearTtl(): UserCreateAttachmentRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserCreateAttachmentRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserCreateAttachmentRequest): UserCreateAttachmentRequest.AsObject;
  static serializeBinaryToWriter(message: UserCreateAttachmentRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserCreateAttachmentRequest;
  static deserializeBinaryFromReader(message: UserCreateAttachmentRequest, reader: jspb.BinaryReader): UserCreateAttachmentRequest;
}

export namespace UserCreateAttachmentRequest {
  export type AsObject = {
    title: string;
    contentType: string;
    size: number;
    expireAfterDays?: number;
    pb_public: boolean;
    ttl?: google_protobuf_duration_pb.Duration.AsObject;
  };

  export enum ExpireAfterDaysCase {
    _EXPIRE_AFTER_DAYS_NOT_SET = 0,
    EXPIRE_AFTER_DAYS = 4,
  }
}

export class UserCreateAttachmentUploadResponse extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): UserCreateAttachmentUploadResponse;

  getItem(): UserIndexAttachmentResponse.Item | undefined;
  setItem(value?: UserIndexAttachmentResponse.Item): UserCreateAttachmentUploadResponse;
  hasItem(): boolean;
  clearItem(): UserCreateAttachmentUploadResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserCreateAttachmentUploadResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserCreateAttachmentUploadResponse): UserCreateAttachmentUploadResponse.AsObject;
  static serializeBinaryToWriter(message: UserCreateAttachmentUploadResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserCreateAttachmentUploadResponse;
  static deserializeBinaryFromReader(message: UserCreateAttachmentUploadResponse, reader: jspb.BinaryReader): UserCreateAttachmentUploadResponse;
}

export namespace UserCreateAttachmentUploadResponse {
  export type AsObject = {
    url: string;
    item?: UserIndexAttachmentResponse.Item.AsObject;
  };
}

export class UserIndexAttachmentResponse extends jspb.Message {
  getItemsList(): Array<UserIndexAttachmentResponse.Item>;
  setItemsList(value: Array<UserIndexAttachmentResponse.Item>): UserIndexAttachmentResponse;
  clearItemsList(): UserIndexAttachmentResponse;
  addItems(value?: UserIndexAttachmentResponse.Item, index?: number): UserIndexAttachmentResponse.Item;

  getPagination(): Pagination | undefined;
  setPagination(value?: Pagination): UserIndexAttachmentResponse;
  hasPagination(): boolean;
  clearPagination(): UserIndexAttachmentResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserIndexAttachmentResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserIndexAttachmentResponse): UserIndexAttachmentResponse.AsObject;
  static serializeBinaryToWriter(message: UserIndexAttachmentResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserIndexAttachmentResponse;
  static deserializeBinaryFromReader(message: UserIndexAttachmentResponse, reader: jspb.BinaryReader): UserIndexAttachmentResponse;
}

export namespace UserIndexAttachmentResponse {
  export type AsObject = {
    itemsList: Array<UserIndexAttachmentResponse.Item.AsObject>;
    pagination?: Pagination.AsObject;
  };

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getBucket(): string;
    setBucket(value: string): Item;

    getObject(): string;
    setObject(value: string): Item;

    getTitle(): string;
    setTitle(value: string): Item;

    getContentType(): string;
    setContentType(value: string): Item;

    getSize(): number;
    setSize(value: number): Item;

    getUploadedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setUploadedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasUploadedAt(): boolean;
    clearUploadedAt(): Item;

    getExpireAfterDays(): number;
    setExpireAfterDays(value: number): Item;
    hasExpireAfterDays(): boolean;
    clearExpireAfterDays(): Item;

    getPublic(): boolean;
    setPublic(value: boolean): Item;

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
      id: number;
      bucket: string;
      object: string;
      title: string;
      contentType: string;
      size: number;
      uploadedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
      expireAfterDays?: number;
      pb_public: boolean;
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
    };

    export enum UploadedAtCase {
      _UPLOADED_AT_NOT_SET = 0,
      UPLOADED_AT = 7,
    }

    export enum ExpireAfterDaysCase {
      _EXPIRE_AFTER_DAYS_NOT_SET = 0,
      EXPIRE_AFTER_DAYS = 8,
    }
  }

}

export class UserIndexLogResponse extends jspb.Message {
  getItemsList(): Array<UserIndexLogResponse.Item>;
  setItemsList(value: Array<UserIndexLogResponse.Item>): UserIndexLogResponse;
  clearItemsList(): UserIndexLogResponse;
  addItems(value?: UserIndexLogResponse.Item, index?: number): UserIndexLogResponse.Item;

  getPagination(): Pagination | undefined;
  setPagination(value?: Pagination): UserIndexLogResponse;
  hasPagination(): boolean;
  clearPagination(): UserIndexLogResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserIndexLogResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserIndexLogResponse): UserIndexLogResponse.AsObject;
  static serializeBinaryToWriter(message: UserIndexLogResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserIndexLogResponse;
  static deserializeBinaryFromReader(message: UserIndexLogResponse, reader: jspb.BinaryReader): UserIndexLogResponse;
}

export namespace UserIndexLogResponse {
  export type AsObject = {
    itemsList: Array<UserIndexLogResponse.Item.AsObject>;
    pagination?: Pagination.AsObject;
  };

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getPlugin(): string;
    setPlugin(value: string): Item;

    getIp(): string;
    setIp(value: string): Item;

    getLevel(): UserIndexLogResponse.Item.Level;
    setLevel(value: UserIndexLogResponse.Item.Level): Item;

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
      id: number;
      plugin: string;
      ip: string;
      level: UserIndexLogResponse.Item.Level;
      message: string;
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
    };

    export enum Level {
      DEBUG = 0,
      INFO = 1,
      WARNING = 2,
      ERROR = 3,
    }
  }

}

export class Session extends jspb.Message {
  getSubject(): Session.Subject | undefined;
  setSubject(value?: Session.Subject): Session;
  hasSubject(): boolean;
  clearSubject(): Session;

  getName(): string;
  setName(value: string): Session;

  getAvatar(): string;
  setAvatar(value: string): Session;

  getUser(): UserIndexResponse.Item | undefined;
  setUser(value?: UserIndexResponse.Item): Session;
  hasUser(): boolean;
  clearUser(): Session;

  getClientIp(): string;
  setClientIp(value: string): Session;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Session.AsObject;
  static toObject(includeInstance: boolean, msg: Session): Session.AsObject;
  static serializeBinaryToWriter(message: Session, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Session;
  static deserializeBinaryFromReader(message: Session, reader: jspb.BinaryReader): Session;
}

export namespace Session {
  export type AsObject = {
    subject?: Session.Subject.AsObject;
    name: string;
    avatar: string;
    user?: UserIndexResponse.Item.AsObject;
    clientIp: string;
  };

  export class Subject extends jspb.Message {
    getType(): Session.ProviderType;
    setType(value: Session.ProviderType): Subject;

    getSn(): string;
    setSn(value: string): Subject;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Subject.AsObject;
    static toObject(includeInstance: boolean, msg: Subject): Subject.AsObject;
    static serializeBinaryToWriter(message: Subject, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Subject;
    static deserializeBinaryFromReader(message: Subject, reader: jspb.BinaryReader): Subject;
  }

  export namespace Subject {
    export type AsObject = {
      type: Session.ProviderType;
      sn: string;
    };
  }


  export enum ProviderType {
    EMAIL = 0,
    PHONE = 1,
    GOOGLE_OAUTH2 = 2,
    WECHAT_OAUTH2 = 3,
    WECHAT_MINI_PROGRAM = 4,
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
    token: string;
  };
}

