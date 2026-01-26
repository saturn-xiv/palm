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

export class User extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): User.AsObject;
  static toObject(includeInstance: boolean, msg: User): User.AsObject;
  static serializeBinaryToWriter(message: User, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): User;
  static deserializeBinaryFromReader(message: User, reader: jspb.BinaryReader): User;
}

export namespace User {
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
  getType(): User.ProviderType;
  setType(value: User.ProviderType): Session;

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
    type: User.ProviderType;
    sn: string;
  };
}

