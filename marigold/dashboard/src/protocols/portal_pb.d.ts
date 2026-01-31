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

export class Attachment extends jspb.Message {
  getBucket(): string;
  setBucket(value: string): Attachment;

  getObject(): string;
  setObject(value: string): Attachment;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Attachment.AsObject;
  static toObject(includeInstance: boolean, msg: Attachment): Attachment.AsObject;
  static serializeBinaryToWriter(message: Attachment, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Attachment;
  static deserializeBinaryFromReader(message: Attachment, reader: jspb.BinaryReader): Attachment;
}

export namespace Attachment {
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

  getAttachmentsList(): Array<Attachment>;
  setAttachmentsList(value: Array<Attachment>): RichText;
  clearAttachmentsList(): RichText;
  addAttachments(value?: Attachment, index?: number): Attachment;

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
    attachmentsList: Array<Attachment.AsObject>;
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

export class Log extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Log.AsObject;
  static toObject(includeInstance: boolean, msg: Log): Log.AsObject;
  static serializeBinaryToWriter(message: Log, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Log;
  static deserializeBinaryFromReader(message: Log, reader: jspb.BinaryReader): Log;
}

export namespace Log {
  export type AsObject = {
  };

  export enum Level {
    DEBUG = 0,
    INFO = 1,
    WARNING = 2,
    ERROR = 3,
  }
}

export class UserDetail extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserDetail.AsObject;
  static toObject(includeInstance: boolean, msg: UserDetail): UserDetail.AsObject;
  static serializeBinaryToWriter(message: UserDetail, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserDetail;
  static deserializeBinaryFromReader(message: UserDetail, reader: jspb.BinaryReader): UserDetail;
}

export namespace UserDetail {
  export type AsObject = {
  };

  export enum ProviderType {
    EMAIL = 0,
    PHONE = 1,
    GOOGLE_OAUTH2 = 2,
    WECHAT_OAUTH2 = 3,
    WECHAT_MINI_PROGRAM = 4,
  }
}

export class Session extends jspb.Message {
  getType(): UserDetail.ProviderType;
  setType(value: UserDetail.ProviderType): Session;

  getSn(): string;
  setSn(value: string): Session;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Session.AsObject;
  static toObject(includeInstance: boolean, msg: Session): Session.AsObject;
  static serializeBinaryToWriter(message: Session, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Session;
  static deserializeBinaryFromReader(message: Session, reader: jspb.BinaryReader): Session;
}

export namespace Session {
  export type AsObject = {
    type: UserDetail.ProviderType;
    sn: string;
  };
}

