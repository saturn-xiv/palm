import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"


export class ListBucketResponse extends jspb.Message {
  getItemsList(): Array<ListBucketResponse.Item>;
  setItemsList(value: Array<ListBucketResponse.Item>): ListBucketResponse;
  clearItemsList(): ListBucketResponse;
  addItems(value?: ListBucketResponse.Item, index?: number): ListBucketResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListBucketResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListBucketResponse): ListBucketResponse.AsObject;
  static serializeBinaryToWriter(message: ListBucketResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListBucketResponse;
  static deserializeBinaryFromReader(message: ListBucketResponse, reader: jspb.BinaryReader): ListBucketResponse;
}

export namespace ListBucketResponse {
  export type AsObject = {
    itemsList: Array<ListBucketResponse.Item.AsObject>;
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

