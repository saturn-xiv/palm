import * as jspb from 'google-protobuf'



export class Task extends jspb.Message {
  constructor();
  constructor(opt_data?: Task.AsObject);
  getEntry(): string;
  setEntry(value: string): Task;

  getClass(): Task.Class;
  setClass(value: Task.Class): Task;

  getFilesMap(): jspb.Map<string, Uint8Array | string>;
  clearFilesMap(): Task;

  getOutput(): string;
  setOutput(value: string): Task;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Task.AsObject;
  static toObject(includeInstance: boolean, msg: Task): Task.AsObject;
  static serializeBinaryToWriter(message: Task, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Task;
  static deserializeBinaryFromReader(message: Task, reader: jspb.BinaryReader): Task;
}

export namespace Task {
  export type AsObject = {
    entry: string;
    pb_class: Task.Class;
    filesMap: Array<[string, Uint8Array | string]>;
    output: string;
  };

  export enum Class {
    ARTICLE = 0,
    REPORT = 1,
    BOOK = 2,
    LETTER = 3,
    BEAMER = 4,
  }
}

