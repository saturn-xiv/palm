import * as jspb from 'google-protobuf'



export class Task extends jspb.Message {
  constructor();
  constructor(opt_data?: Task.AsObject);
  getSn(): string;
  setSn(value: string): Task;

  getName(): string;
  setName(value: string): Task;

  getSummary(): string;
  setSummary(value: string): Task;

  getVersion(): string;
  setVersion(value: string): Task;

  getScript(): string;
  setScript(value: string): Task;

  getArgsList(): Array<string>;
  setArgsList(value: Array<string>): Task;
  clearArgsList(): Task;
  addArgs(value: string, index?: number): Task;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Task.AsObject;
  static toObject(includeInstance: boolean, msg: Task): Task.AsObject;
  static serializeBinaryToWriter(message: Task, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Task;
  static deserializeBinaryFromReader(message: Task, reader: jspb.BinaryReader): Task;
}

export namespace Task {
  export type AsObject = {
    sn: string;
    name: string;
    summary: string;
    version: string;
    script: string;
    argsList: Array<string>;
  };
}

