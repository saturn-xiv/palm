import * as jspb from 'google-protobuf'



export class Task extends jspb.Message {
  getDocument(): Task.Document;
  setDocument(value: Task.Document): Task;

  getEntry(): string;
  setEntry(value: string): Task;

  getFilesMap(): jspb.Map<string, Uint8Array | string>;
  clearFilesMap(): Task;

  getTarget(): Task.Target | undefined;
  setTarget(value?: Task.Target): Task;
  hasTarget(): boolean;
  clearTarget(): Task;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Task.AsObject;
  static toObject(includeInstance: boolean, msg: Task): Task.AsObject;
  static serializeBinaryToWriter(message: Task, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Task;
  static deserializeBinaryFromReader(message: Task, reader: jspb.BinaryReader): Task;
}

export namespace Task {
  export type AsObject = {
    document: Task.Document;
    entry: string;
    filesMap: Array<[string, Uint8Array | string]>;
    target?: Task.Target.AsObject;
  };

  export class Target extends jspb.Message {
    getBucket(): string;
    setBucket(value: string): Target;

    getObject(): string;
    setObject(value: string): Target;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Target.AsObject;
    static toObject(includeInstance: boolean, msg: Target): Target.AsObject;
    static serializeBinaryToWriter(message: Target, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Target;
    static deserializeBinaryFromReader(message: Target, reader: jspb.BinaryReader): Target;
  }

  export namespace Target {
    export type AsObject = {
      bucket: string;
      object: string;
    };
  }


  export enum Document {
    BOOK = 0,
    ARTICLE = 1,
    REPORT = 2,
    LETTER = 3,
    BEAMER = 4,
  }
}

