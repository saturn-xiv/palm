import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"


export class SmsSendRequest extends jspb.Message {
  getToList(): Array<string>;
  setToList(value: Array<string>): SmsSendRequest;
  clearToList(): SmsSendRequest;
  addTo(value: string, index?: number): SmsSendRequest;

  getMessage(): string;
  setMessage(value: string): SmsSendRequest;

  getCallback(): string;
  setCallback(value: string): SmsSendRequest;
  hasCallback(): boolean;
  clearCallback(): SmsSendRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SmsSendRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SmsSendRequest): SmsSendRequest.AsObject;
  static serializeBinaryToWriter(message: SmsSendRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SmsSendRequest;
  static deserializeBinaryFromReader(message: SmsSendRequest, reader: jspb.BinaryReader): SmsSendRequest;
}

export namespace SmsSendRequest {
  export type AsObject = {
    toList: Array<string>,
    message: string,
    callback?: string,
  }

  export enum CallbackCase { 
    _CALLBACK_NOT_SET = 0,
    CALLBACK = 3,
  }
}

