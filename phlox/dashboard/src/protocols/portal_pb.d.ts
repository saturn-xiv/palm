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
    itemsList: Array<UserIndexLogResponse.Item.AsObject>,
    pagination?: Pagination.AsObject,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getPlugin(): string;
    setPlugin(value: string): Item;

    getIp(): string;
    setIp(value: string): Item;

    getLevel(): UserIndexLogResponse.Item.Level;
    setLevel(value: UserIndexLogResponse.Item.Level): Item;

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
      level: UserIndexLogResponse.Item.Level,
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

export class UserSignInByEmailRequest extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): UserSignInByEmailRequest;

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
    email: string,
    password: string,
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }

  export enum TtlCase { 
    _TTL_NOT_SET = 0,
    TTL = 9,
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

