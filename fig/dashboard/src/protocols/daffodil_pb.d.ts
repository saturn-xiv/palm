import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as azalea_pb from './azalea_pb';
import * as camelia_pb from './camelia_pb';


export class BookDetail extends jspb.Message {
  getId(): number;
  setId(value: number): BookDetail;

  getName(): string;
  setName(value: string): BookDetail;

  getCover(): string;
  setCover(value: string): BookDetail;
  hasCover(): boolean;
  clearCover(): BookDetail;

  getDeletedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setDeletedAt(value?: google_protobuf_timestamp_pb.Timestamp): BookDetail;
  hasDeletedAt(): boolean;
  clearDeletedAt(): BookDetail;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BookDetail.AsObject;
  static toObject(includeInstance: boolean, msg: BookDetail): BookDetail.AsObject;
  static serializeBinaryToWriter(message: BookDetail, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BookDetail;
  static deserializeBinaryFromReader(message: BookDetail, reader: jspb.BinaryReader): BookDetail;
}

export namespace BookDetail {
  export type AsObject = {
    id: number,
    name: string,
    cover?: string,
    deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }

  export enum CoverCase { 
    _COVER_NOT_SET = 0,
    COVER = 3,
  }

  export enum DeletedAtCase { 
    _DELETED_AT_NOT_SET = 0,
    DELETED_AT = 99,
  }
}

export class AccountDetail extends jspb.Message {
  getId(): number;
  setId(value: number): AccountDetail;

  getName(): string;
  setName(value: string): AccountDetail;

  getCover(): string;
  setCover(value: string): AccountDetail;
  hasCover(): boolean;
  clearCover(): AccountDetail;

  getType(): AccountIndexResponse.Item.Type;
  setType(value: AccountIndexResponse.Item.Type): AccountDetail;

  getDeletedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setDeletedAt(value?: google_protobuf_timestamp_pb.Timestamp): AccountDetail;
  hasDeletedAt(): boolean;
  clearDeletedAt(): AccountDetail;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AccountDetail.AsObject;
  static toObject(includeInstance: boolean, msg: AccountDetail): AccountDetail.AsObject;
  static serializeBinaryToWriter(message: AccountDetail, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AccountDetail;
  static deserializeBinaryFromReader(message: AccountDetail, reader: jspb.BinaryReader): AccountDetail;
}

export namespace AccountDetail {
  export type AsObject = {
    id: number,
    name: string,
    cover?: string,
    type: AccountIndexResponse.Item.Type,
    deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }

  export enum CoverCase { 
    _COVER_NOT_SET = 0,
    COVER = 3,
  }

  export enum DeletedAtCase { 
    _DELETED_AT_NOT_SET = 0,
    DELETED_AT = 99,
  }
}

export class MerchantDetail extends jspb.Message {
  getId(): number;
  setId(value: number): MerchantDetail;

  getName(): string;
  setName(value: string): MerchantDetail;

  getCover(): string;
  setCover(value: string): MerchantDetail;
  hasCover(): boolean;
  clearCover(): MerchantDetail;

  getDeletedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setDeletedAt(value?: google_protobuf_timestamp_pb.Timestamp): MerchantDetail;
  hasDeletedAt(): boolean;
  clearDeletedAt(): MerchantDetail;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MerchantDetail.AsObject;
  static toObject(includeInstance: boolean, msg: MerchantDetail): MerchantDetail.AsObject;
  static serializeBinaryToWriter(message: MerchantDetail, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MerchantDetail;
  static deserializeBinaryFromReader(message: MerchantDetail, reader: jspb.BinaryReader): MerchantDetail;
}

export namespace MerchantDetail {
  export type AsObject = {
    id: number,
    name: string,
    cover?: string,
    deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }

  export enum CoverCase { 
    _COVER_NOT_SET = 0,
    COVER = 3,
  }

  export enum DeletedAtCase { 
    _DELETED_AT_NOT_SET = 0,
    DELETED_AT = 99,
  }
}

export class Amount extends jspb.Message {
  getValue(): number;
  setValue(value: number): Amount;

  getCurrency(): string;
  setCurrency(value: string): Amount;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Amount.AsObject;
  static toObject(includeInstance: boolean, msg: Amount): Amount.AsObject;
  static serializeBinaryToWriter(message: Amount, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Amount;
  static deserializeBinaryFromReader(message: Amount, reader: jspb.BinaryReader): Amount;
}

export namespace Amount {
  export type AsObject = {
    value: number,
    currency: string,
  }
}

export class BookCreateRequest extends jspb.Message {
  getName(): string;
  setName(value: string): BookCreateRequest;

  getDescription(): string;
  setDescription(value: string): BookCreateRequest;

  getCover(): number;
  setCover(value: number): BookCreateRequest;
  hasCover(): boolean;
  clearCover(): BookCreateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BookCreateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: BookCreateRequest): BookCreateRequest.AsObject;
  static serializeBinaryToWriter(message: BookCreateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BookCreateRequest;
  static deserializeBinaryFromReader(message: BookCreateRequest, reader: jspb.BinaryReader): BookCreateRequest;
}

export namespace BookCreateRequest {
  export type AsObject = {
    name: string,
    description: string,
    cover?: number,
  }

  export enum CoverCase { 
    _COVER_NOT_SET = 0,
    COVER = 3,
  }
}

export class BookUpdateRequest extends jspb.Message {
  getId(): number;
  setId(value: number): BookUpdateRequest;

  getName(): string;
  setName(value: string): BookUpdateRequest;

  getDescription(): string;
  setDescription(value: string): BookUpdateRequest;

  getCover(): number;
  setCover(value: number): BookUpdateRequest;
  hasCover(): boolean;
  clearCover(): BookUpdateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BookUpdateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: BookUpdateRequest): BookUpdateRequest.AsObject;
  static serializeBinaryToWriter(message: BookUpdateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BookUpdateRequest;
  static deserializeBinaryFromReader(message: BookUpdateRequest, reader: jspb.BinaryReader): BookUpdateRequest;
}

export namespace BookUpdateRequest {
  export type AsObject = {
    id: number,
    name: string,
    description: string,
    cover?: number,
  }

  export enum CoverCase { 
    _COVER_NOT_SET = 0,
    COVER = 4,
  }
}

export class BookIndexResponse extends jspb.Message {
  getItemsList(): Array<BookIndexResponse.Item>;
  setItemsList(value: Array<BookIndexResponse.Item>): BookIndexResponse;
  clearItemsList(): BookIndexResponse;
  addItems(value?: BookIndexResponse.Item, index?: number): BookIndexResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BookIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: BookIndexResponse): BookIndexResponse.AsObject;
  static serializeBinaryToWriter(message: BookIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BookIndexResponse;
  static deserializeBinaryFromReader(message: BookIndexResponse, reader: jspb.BinaryReader): BookIndexResponse;
}

export namespace BookIndexResponse {
  export type AsObject = {
    itemsList: Array<BookIndexResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getName(): string;
    setName(value: string): Item;

    getDescription(): string;
    setDescription(value: string): Item;

    getCover(): string;
    setCover(value: string): Item;
    hasCover(): boolean;
    clearCover(): Item;

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
      name: string,
      description: string,
      cover?: string,
      deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum CoverCase { 
      _COVER_NOT_SET = 0,
      COVER = 4,
    }
  }

}

export class BookShowResponse extends jspb.Message {
  getItem(): BookIndexResponse.Item | undefined;
  setItem(value?: BookIndexResponse.Item): BookShowResponse;
  hasItem(): boolean;
  clearItem(): BookShowResponse;

  getMerchantsList(): Array<MerchantDetail>;
  setMerchantsList(value: Array<MerchantDetail>): BookShowResponse;
  clearMerchantsList(): BookShowResponse;
  addMerchants(value?: MerchantDetail, index?: number): MerchantDetail;

  getAccountsList(): Array<AccountDetail>;
  setAccountsList(value: Array<AccountDetail>): BookShowResponse;
  clearAccountsList(): BookShowResponse;
  addAccounts(value?: AccountDetail, index?: number): AccountDetail;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BookShowResponse.AsObject;
  static toObject(includeInstance: boolean, msg: BookShowResponse): BookShowResponse.AsObject;
  static serializeBinaryToWriter(message: BookShowResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BookShowResponse;
  static deserializeBinaryFromReader(message: BookShowResponse, reader: jspb.BinaryReader): BookShowResponse;
}

export namespace BookShowResponse {
  export type AsObject = {
    item?: BookIndexResponse.Item.AsObject,
    merchantsList: Array<MerchantDetail.AsObject>,
    accountsList: Array<AccountDetail.AsObject>,
  }
}

export class AccountCreateRequest extends jspb.Message {
  getBook(): number;
  setBook(value: number): AccountCreateRequest;

  getName(): string;
  setName(value: string): AccountCreateRequest;

  getCurrency(): string;
  setCurrency(value: string): AccountCreateRequest;

  getType(): AccountIndexResponse.Item.Type;
  setType(value: AccountIndexResponse.Item.Type): AccountCreateRequest;

  getDescription(): string;
  setDescription(value: string): AccountCreateRequest;

  getCover(): number;
  setCover(value: number): AccountCreateRequest;
  hasCover(): boolean;
  clearCover(): AccountCreateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AccountCreateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AccountCreateRequest): AccountCreateRequest.AsObject;
  static serializeBinaryToWriter(message: AccountCreateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AccountCreateRequest;
  static deserializeBinaryFromReader(message: AccountCreateRequest, reader: jspb.BinaryReader): AccountCreateRequest;
}

export namespace AccountCreateRequest {
  export type AsObject = {
    book: number,
    name: string,
    currency: string,
    type: AccountIndexResponse.Item.Type,
    description: string,
    cover?: number,
  }

  export enum CoverCase { 
    _COVER_NOT_SET = 0,
    COVER = 6,
  }
}

export class AccountUpdateRequest extends jspb.Message {
  getId(): number;
  setId(value: number): AccountUpdateRequest;

  getName(): string;
  setName(value: string): AccountUpdateRequest;

  getCurrency(): string;
  setCurrency(value: string): AccountUpdateRequest;

  getType(): AccountIndexResponse.Item.Type;
  setType(value: AccountIndexResponse.Item.Type): AccountUpdateRequest;

  getDescription(): string;
  setDescription(value: string): AccountUpdateRequest;

  getCover(): number;
  setCover(value: number): AccountUpdateRequest;
  hasCover(): boolean;
  clearCover(): AccountUpdateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AccountUpdateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AccountUpdateRequest): AccountUpdateRequest.AsObject;
  static serializeBinaryToWriter(message: AccountUpdateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AccountUpdateRequest;
  static deserializeBinaryFromReader(message: AccountUpdateRequest, reader: jspb.BinaryReader): AccountUpdateRequest;
}

export namespace AccountUpdateRequest {
  export type AsObject = {
    id: number,
    name: string,
    currency: string,
    type: AccountIndexResponse.Item.Type,
    description: string,
    cover?: number,
  }

  export enum CoverCase { 
    _COVER_NOT_SET = 0,
    COVER = 6,
  }
}

export class AccountIndexResponse extends jspb.Message {
  getItemsList(): Array<AccountIndexResponse.Item>;
  setItemsList(value: Array<AccountIndexResponse.Item>): AccountIndexResponse;
  clearItemsList(): AccountIndexResponse;
  addItems(value?: AccountIndexResponse.Item, index?: number): AccountIndexResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AccountIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: AccountIndexResponse): AccountIndexResponse.AsObject;
  static serializeBinaryToWriter(message: AccountIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AccountIndexResponse;
  static deserializeBinaryFromReader(message: AccountIndexResponse, reader: jspb.BinaryReader): AccountIndexResponse;
}

export namespace AccountIndexResponse {
  export type AsObject = {
    itemsList: Array<AccountIndexResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getName(): string;
    setName(value: string): Item;

    getCurrency(): string;
    setCurrency(value: string): Item;

    getType(): AccountIndexResponse.Item.Type;
    setType(value: AccountIndexResponse.Item.Type): Item;

    getDescription(): string;
    setDescription(value: string): Item;

    getCover(): string;
    setCover(value: string): Item;
    hasCover(): boolean;
    clearCover(): Item;

    getUser(): camelia_pb.UserInfo | undefined;
    setUser(value?: camelia_pb.UserInfo): Item;
    hasUser(): boolean;
    clearUser(): Item;

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
      name: string,
      currency: string,
      type: AccountIndexResponse.Item.Type,
      description: string,
      cover?: string,
      user?: camelia_pb.UserInfo.AsObject,
      deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum Type { 
      CASH = 0,
      CHECKING = 1,
      SAVING = 2,
      CREDITCARD = 3,
      MORTGAGE = 4,
      LOAN = 5,
    }

    export enum CoverCase { 
      _COVER_NOT_SET = 0,
      COVER = 6,
    }
  }

}

export class AccountShowResponse extends jspb.Message {
  getItem(): AccountIndexResponse.Item | undefined;
  setItem(value?: AccountIndexResponse.Item): AccountShowResponse;
  hasItem(): boolean;
  clearItem(): AccountShowResponse;

  getBook(): BookDetail | undefined;
  setBook(value?: BookDetail): AccountShowResponse;
  hasBook(): boolean;
  clearBook(): AccountShowResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AccountShowResponse.AsObject;
  static toObject(includeInstance: boolean, msg: AccountShowResponse): AccountShowResponse.AsObject;
  static serializeBinaryToWriter(message: AccountShowResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AccountShowResponse;
  static deserializeBinaryFromReader(message: AccountShowResponse, reader: jspb.BinaryReader): AccountShowResponse;
}

export namespace AccountShowResponse {
  export type AsObject = {
    item?: AccountIndexResponse.Item.AsObject,
    book?: BookDetail.AsObject,
  }
}

export class MerchantCreateRequest extends jspb.Message {
  getBook(): number;
  setBook(value: number): MerchantCreateRequest;

  getName(): string;
  setName(value: string): MerchantCreateRequest;

  getAddress(): string;
  setAddress(value: string): MerchantCreateRequest;
  hasAddress(): boolean;
  clearAddress(): MerchantCreateRequest;

  getContact(): string;
  setContact(value: string): MerchantCreateRequest;
  hasContact(): boolean;
  clearContact(): MerchantCreateRequest;

  getDescription(): string;
  setDescription(value: string): MerchantCreateRequest;
  hasDescription(): boolean;
  clearDescription(): MerchantCreateRequest;

  getCover(): number;
  setCover(value: number): MerchantCreateRequest;
  hasCover(): boolean;
  clearCover(): MerchantCreateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MerchantCreateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: MerchantCreateRequest): MerchantCreateRequest.AsObject;
  static serializeBinaryToWriter(message: MerchantCreateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MerchantCreateRequest;
  static deserializeBinaryFromReader(message: MerchantCreateRequest, reader: jspb.BinaryReader): MerchantCreateRequest;
}

export namespace MerchantCreateRequest {
  export type AsObject = {
    book: number,
    name: string,
    address?: string,
    contact?: string,
    description?: string,
    cover?: number,
  }

  export enum AddressCase { 
    _ADDRESS_NOT_SET = 0,
    ADDRESS = 11,
  }

  export enum ContactCase { 
    _CONTACT_NOT_SET = 0,
    CONTACT = 12,
  }

  export enum DescriptionCase { 
    _DESCRIPTION_NOT_SET = 0,
    DESCRIPTION = 13,
  }

  export enum CoverCase { 
    _COVER_NOT_SET = 0,
    COVER = 14,
  }
}

export class MerchantUpdateRequest extends jspb.Message {
  getId(): number;
  setId(value: number): MerchantUpdateRequest;

  getName(): string;
  setName(value: string): MerchantUpdateRequest;

  getAddress(): string;
  setAddress(value: string): MerchantUpdateRequest;
  hasAddress(): boolean;
  clearAddress(): MerchantUpdateRequest;

  getContact(): string;
  setContact(value: string): MerchantUpdateRequest;
  hasContact(): boolean;
  clearContact(): MerchantUpdateRequest;

  getDescription(): string;
  setDescription(value: string): MerchantUpdateRequest;
  hasDescription(): boolean;
  clearDescription(): MerchantUpdateRequest;

  getCover(): number;
  setCover(value: number): MerchantUpdateRequest;
  hasCover(): boolean;
  clearCover(): MerchantUpdateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MerchantUpdateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: MerchantUpdateRequest): MerchantUpdateRequest.AsObject;
  static serializeBinaryToWriter(message: MerchantUpdateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MerchantUpdateRequest;
  static deserializeBinaryFromReader(message: MerchantUpdateRequest, reader: jspb.BinaryReader): MerchantUpdateRequest;
}

export namespace MerchantUpdateRequest {
  export type AsObject = {
    id: number,
    name: string,
    address?: string,
    contact?: string,
    description?: string,
    cover?: number,
  }

  export enum AddressCase { 
    _ADDRESS_NOT_SET = 0,
    ADDRESS = 11,
  }

  export enum ContactCase { 
    _CONTACT_NOT_SET = 0,
    CONTACT = 12,
  }

  export enum DescriptionCase { 
    _DESCRIPTION_NOT_SET = 0,
    DESCRIPTION = 13,
  }

  export enum CoverCase { 
    _COVER_NOT_SET = 0,
    COVER = 14,
  }
}

export class MerchantIndexResponse extends jspb.Message {
  getItemsList(): Array<MerchantIndexResponse.Item>;
  setItemsList(value: Array<MerchantIndexResponse.Item>): MerchantIndexResponse;
  clearItemsList(): MerchantIndexResponse;
  addItems(value?: MerchantIndexResponse.Item, index?: number): MerchantIndexResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MerchantIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: MerchantIndexResponse): MerchantIndexResponse.AsObject;
  static serializeBinaryToWriter(message: MerchantIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MerchantIndexResponse;
  static deserializeBinaryFromReader(message: MerchantIndexResponse, reader: jspb.BinaryReader): MerchantIndexResponse;
}

export namespace MerchantIndexResponse {
  export type AsObject = {
    itemsList: Array<MerchantIndexResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getName(): string;
    setName(value: string): Item;

    getAddress(): string;
    setAddress(value: string): Item;
    hasAddress(): boolean;
    clearAddress(): Item;

    getContact(): string;
    setContact(value: string): Item;
    hasContact(): boolean;
    clearContact(): Item;

    getDescription(): string;
    setDescription(value: string): Item;
    hasDescription(): boolean;
    clearDescription(): Item;

    getCover(): string;
    setCover(value: string): Item;
    hasCover(): boolean;
    clearCover(): Item;

    getUser(): camelia_pb.UserInfo | undefined;
    setUser(value?: camelia_pb.UserInfo): Item;
    hasUser(): boolean;
    clearUser(): Item;

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
      name: string,
      address?: string,
      contact?: string,
      description?: string,
      cover?: string,
      user?: camelia_pb.UserInfo.AsObject,
      deletedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum AddressCase { 
      _ADDRESS_NOT_SET = 0,
      ADDRESS = 11,
    }

    export enum ContactCase { 
      _CONTACT_NOT_SET = 0,
      CONTACT = 12,
    }

    export enum DescriptionCase { 
      _DESCRIPTION_NOT_SET = 0,
      DESCRIPTION = 13,
    }

    export enum CoverCase { 
      _COVER_NOT_SET = 0,
      COVER = 14,
    }
  }

}

export class MerchantShowResponse extends jspb.Message {
  getItem(): MerchantIndexResponse.Item | undefined;
  setItem(value?: MerchantIndexResponse.Item): MerchantShowResponse;
  hasItem(): boolean;
  clearItem(): MerchantShowResponse;

  getBook(): BookDetail | undefined;
  setBook(value?: BookDetail): MerchantShowResponse;
  hasBook(): boolean;
  clearBook(): MerchantShowResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MerchantShowResponse.AsObject;
  static toObject(includeInstance: boolean, msg: MerchantShowResponse): MerchantShowResponse.AsObject;
  static serializeBinaryToWriter(message: MerchantShowResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MerchantShowResponse;
  static deserializeBinaryFromReader(message: MerchantShowResponse, reader: jspb.BinaryReader): MerchantShowResponse;
}

export namespace MerchantShowResponse {
  export type AsObject = {
    item?: MerchantIndexResponse.Item.AsObject,
    book?: BookDetail.AsObject,
  }
}

export class TransactionDeleteRequest extends jspb.Message {
  getId(): number;
  setId(value: number): TransactionDeleteRequest;

  getReason(): string;
  setReason(value: string): TransactionDeleteRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TransactionDeleteRequest.AsObject;
  static toObject(includeInstance: boolean, msg: TransactionDeleteRequest): TransactionDeleteRequest.AsObject;
  static serializeBinaryToWriter(message: TransactionDeleteRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TransactionDeleteRequest;
  static deserializeBinaryFromReader(message: TransactionDeleteRequest, reader: jspb.BinaryReader): TransactionDeleteRequest;
}

export namespace TransactionDeleteRequest {
  export type AsObject = {
    id: number,
    reason: string,
  }
}

export class TransactionCreateRequest extends jspb.Message {
  getBook(): number;
  setBook(value: number): TransactionCreateRequest;

  getSourceAccount(): number;
  setSourceAccount(value: number): TransactionCreateRequest;

  getDestinationAccount(): number;
  setDestinationAccount(value: number): TransactionCreateRequest;

  getMerchant(): number;
  setMerchant(value: number): TransactionCreateRequest;

  getAmount(): number;
  setAmount(value: number): TransactionCreateRequest;

  getSummary(): string;
  setSummary(value: string): TransactionCreateRequest;

  getPaidAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setPaidAt(value?: google_protobuf_timestamp_pb.Timestamp): TransactionCreateRequest;
  hasPaidAt(): boolean;
  clearPaidAt(): TransactionCreateRequest;

  getType(): TransactionIndexResponse.Item.Type;
  setType(value: TransactionIndexResponse.Item.Type): TransactionCreateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TransactionCreateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: TransactionCreateRequest): TransactionCreateRequest.AsObject;
  static serializeBinaryToWriter(message: TransactionCreateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TransactionCreateRequest;
  static deserializeBinaryFromReader(message: TransactionCreateRequest, reader: jspb.BinaryReader): TransactionCreateRequest;
}

export namespace TransactionCreateRequest {
  export type AsObject = {
    book: number,
    sourceAccount: number,
    destinationAccount: number,
    merchant: number,
    amount: number,
    summary: string,
    paidAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    type: TransactionIndexResponse.Item.Type,
  }
}

export class TransactionIndexResponse extends jspb.Message {
  getItemsList(): Array<TransactionIndexResponse.Item>;
  setItemsList(value: Array<TransactionIndexResponse.Item>): TransactionIndexResponse;
  clearItemsList(): TransactionIndexResponse;
  addItems(value?: TransactionIndexResponse.Item, index?: number): TransactionIndexResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TransactionIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: TransactionIndexResponse): TransactionIndexResponse.AsObject;
  static serializeBinaryToWriter(message: TransactionIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TransactionIndexResponse;
  static deserializeBinaryFromReader(message: TransactionIndexResponse, reader: jspb.BinaryReader): TransactionIndexResponse;
}

export namespace TransactionIndexResponse {
  export type AsObject = {
    itemsList: Array<TransactionIndexResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getSummary(): string;
    setSummary(value: string): Item;

    getPaidAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setPaidAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasPaidAt(): boolean;
    clearPaidAt(): Item;

    getUser(): camelia_pb.UserInfo | undefined;
    setUser(value?: camelia_pb.UserInfo): Item;
    hasUser(): boolean;
    clearUser(): Item;

    getAmount(): Amount | undefined;
    setAmount(value?: Amount): Item;
    hasAmount(): boolean;
    clearAmount(): Item;

    getBook(): BookDetail | undefined;
    setBook(value?: BookDetail): Item;
    hasBook(): boolean;
    clearBook(): Item;

    getSource(): AccountDetail | undefined;
    setSource(value?: AccountDetail): Item;
    hasSource(): boolean;
    clearSource(): Item;

    getDestination(): AccountDetail | undefined;
    setDestination(value?: AccountDetail): Item;
    hasDestination(): boolean;
    clearDestination(): Item;

    getMerchant(): MerchantDetail | undefined;
    setMerchant(value?: MerchantDetail): Item;
    hasMerchant(): boolean;
    clearMerchant(): Item;

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
      summary: string,
      paidAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
      user?: camelia_pb.UserInfo.AsObject,
      amount?: Amount.AsObject,
      book?: BookDetail.AsObject,
      source?: AccountDetail.AsObject,
      destination?: AccountDetail.AsObject,
      merchant?: MerchantDetail.AsObject,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }

    export enum Type { 
      ASSETS = 0,
      LIABILITIES = 1,
      EQUITY = 2,
      INCOME = 3,
      EXPENSES = 4,
    }
  }

}

export class TransactionTrashResponse extends jspb.Message {
  getItemsList(): Array<TransactionTrashResponse.Item>;
  setItemsList(value: Array<TransactionTrashResponse.Item>): TransactionTrashResponse;
  clearItemsList(): TransactionTrashResponse;
  addItems(value?: TransactionTrashResponse.Item, index?: number): TransactionTrashResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TransactionTrashResponse.AsObject;
  static toObject(includeInstance: boolean, msg: TransactionTrashResponse): TransactionTrashResponse.AsObject;
  static serializeBinaryToWriter(message: TransactionTrashResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TransactionTrashResponse;
  static deserializeBinaryFromReader(message: TransactionTrashResponse, reader: jspb.BinaryReader): TransactionTrashResponse;
}

export namespace TransactionTrashResponse {
  export type AsObject = {
    itemsList: Array<TransactionTrashResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getOriginal(): TransactionIndexResponse.Item | undefined;
    setOriginal(value?: TransactionIndexResponse.Item): Item;
    hasOriginal(): boolean;
    clearOriginal(): Item;

    getReason(): string;
    setReason(value: string): Item;

    getOperator(): camelia_pb.UserInfo | undefined;
    setOperator(value?: camelia_pb.UserInfo): Item;
    hasOperator(): boolean;
    clearOperator(): Item;

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
      original?: TransactionIndexResponse.Item.AsObject,
      reason: string,
      operator?: camelia_pb.UserInfo.AsObject,
      createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
  }

}

