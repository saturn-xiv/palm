import * as jspb from 'google-protobuf'



export class Task extends jspb.Message {
  constructor();
  constructor(opt_data?: Task.AsObject);
  getFrom(): string;
  setFrom(value: string): Task;

  getToList(): Array<string>;
  setToList(value: Array<string>): Task;
  clearToList(): Task;
  addTo(value: string, index?: number): Task;

  getBody(): string;
  setBody(value: string): Task;

  getStatusCallback(): string;
  setStatusCallback(value: string): Task;
  hasStatusCallback(): boolean;
  clearStatusCallback(): Task;

  getBy(): Task.By;
  setBy(value: Task.By): Task;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Task.AsObject;
  static toObject(includeInstance: boolean, msg: Task): Task.AsObject;
  static serializeBinaryToWriter(message: Task, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Task;
  static deserializeBinaryFromReader(message: Task, reader: jspb.BinaryReader): Task;
}

export namespace Task {
  export type AsObject = {
    from: string;
    toList: Array<string>;
    body: string;
    statusCallback?: string;
    by: Task.By;
  };

  export enum By {
    TWILIO = 0,
  }

  export enum StatusCallbackCase {
    _STATUS_CALLBACK_NOT_SET = 0,
    STATUS_CALLBACK = 8,
  }
}

