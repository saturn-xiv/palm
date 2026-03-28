import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"
import * as portal_pb from './portal_pb'; // proto import: "portal.proto"


export class IndexPageResponse extends jspb.Message {
  getItemsList(): Array<IndexPageResponse.Item>;
  setItemsList(value: Array<IndexPageResponse.Item>): IndexPageResponse;
  clearItemsList(): IndexPageResponse;
  addItems(value?: IndexPageResponse.Item, index?: number): IndexPageResponse.Item;

  getPagination(): portal_pb.Pagination | undefined;
  setPagination(value?: portal_pb.Pagination): IndexPageResponse;
  hasPagination(): boolean;
  clearPagination(): IndexPageResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IndexPageResponse.AsObject;
  static toObject(includeInstance: boolean, msg: IndexPageResponse): IndexPageResponse.AsObject;
  static serializeBinaryToWriter(message: IndexPageResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IndexPageResponse;
  static deserializeBinaryFromReader(message: IndexPageResponse, reader: jspb.BinaryReader): IndexPageResponse;
}

export namespace IndexPageResponse {
  export type AsObject = {
    itemsList: Array<IndexPageResponse.Item.AsObject>;
    pagination?: portal_pb.Pagination.AsObject;
  };

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getTitle(): string;
    setTitle(value: string): Item;

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
      title: string;
    };
  }

}

export class ShowPageHtml extends jspb.Message {
  getItem(): IndexPageResponse.Item | undefined;
  setItem(value?: IndexPageResponse.Item): ShowPageHtml;
  hasItem(): boolean;
  clearItem(): ShowPageHtml;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ShowPageHtml.AsObject;
  static toObject(includeInstance: boolean, msg: ShowPageHtml): ShowPageHtml.AsObject;
  static serializeBinaryToWriter(message: ShowPageHtml, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ShowPageHtml;
  static deserializeBinaryFromReader(message: ShowPageHtml, reader: jspb.BinaryReader): ShowPageHtml;
}

export namespace ShowPageHtml {
  export type AsObject = {
    item?: IndexPageResponse.Item.AsObject;
  };
}

export class IndexPageHtml extends jspb.Message {
  getItemsList(): Array<IndexPageResponse.Item>;
  setItemsList(value: Array<IndexPageResponse.Item>): IndexPageHtml;
  clearItemsList(): IndexPageHtml;
  addItems(value?: IndexPageResponse.Item, index?: number): IndexPageResponse.Item;

  getPagination(): portal_pb.Pagination | undefined;
  setPagination(value?: portal_pb.Pagination): IndexPageHtml;
  hasPagination(): boolean;
  clearPagination(): IndexPageHtml;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IndexPageHtml.AsObject;
  static toObject(includeInstance: boolean, msg: IndexPageHtml): IndexPageHtml.AsObject;
  static serializeBinaryToWriter(message: IndexPageHtml, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IndexPageHtml;
  static deserializeBinaryFromReader(message: IndexPageHtml, reader: jspb.BinaryReader): IndexPageHtml;
}

export namespace IndexPageHtml {
  export type AsObject = {
    itemsList: Array<IndexPageResponse.Item.AsObject>;
    pagination?: portal_pb.Pagination.AsObject;
  };
}

