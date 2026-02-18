import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"


export class Task extends jspb.Message {
  getPrinter(): string;
  setPrinter(value: string): Task;

  getPaper(): Task.Paper;
  setPaper(value: Task.Paper): Task;

  getDocument(): Uint8Array | string;
  getDocument_asU8(): Uint8Array;
  getDocument_asB64(): string;
  setDocument(value: Uint8Array | string): Task;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Task.AsObject;
  static toObject(includeInstance: boolean, msg: Task): Task.AsObject;
  static serializeBinaryToWriter(message: Task, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Task;
  static deserializeBinaryFromReader(message: Task, reader: jspb.BinaryReader): Task;
}

export namespace Task {
  export type AsObject = {
    printer: string;
    paper: Task.Paper;
    document: Uint8Array | string;
  };

  export enum Paper {
    A4 = 0,
    LETTER = 1,
  }
}

export class CupsPrintersResponse extends jspb.Message {
  getItemsList(): Array<CupsPrintersResponse.Item>;
  setItemsList(value: Array<CupsPrintersResponse.Item>): CupsPrintersResponse;
  clearItemsList(): CupsPrintersResponse;
  addItems(value?: CupsPrintersResponse.Item, index?: number): CupsPrintersResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CupsPrintersResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CupsPrintersResponse): CupsPrintersResponse.AsObject;
  static serializeBinaryToWriter(message: CupsPrintersResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CupsPrintersResponse;
  static deserializeBinaryFromReader(message: CupsPrintersResponse, reader: jspb.BinaryReader): CupsPrintersResponse;
}

export namespace CupsPrintersResponse {
  export type AsObject = {
    itemsList: Array<CupsPrintersResponse.Item.AsObject>;
  };

  export class Item extends jspb.Message {
    getName(): string;
    setName(value: string): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      name: string;
    };
  }

}

