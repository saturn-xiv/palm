import * as jspb from 'google-protobuf'



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

  getHasPrevious(): boolean;
  setHasPrevious(value: boolean): Pagination;

  getHasNext(): boolean;
  setHasNext(value: boolean): Pagination;

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
    hasPrevious: boolean,
    hasNext: boolean,
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

export enum TextEditor { 
  TEXTAREA = 0,
  MARKDOWN = 1,
  QUILL = 9,
}
