import * as jspb from 'google-protobuf'



export class Task extends jspb.Message {
  constructor();
  constructor(opt_data?: Task.AsObject);
  getName(): string;
  setName(value: string): Task;

  getDocument(): Uint8Array | string;
  getDocument_asU8(): Uint8Array;
  getDocument_asB64(): string;
  setDocument(value: Uint8Array | string): Task;

  getNumberUp(): number;
  setNumberUp(value: number): Task;

  getCopies(): number;
  setCopies(value: number): Task;

  getJobSheet(): Task.JobSheet;
  setJobSheet(value: Task.JobSheet): Task;

  getMedia(): Task.Media;
  setMedia(value: Task.Media): Task;

  getOrientation(): Task.Orientation;
  setOrientation(value: Task.Orientation): Task;

  getQuality(): Task.Quality;
  setQuality(value: Task.Quality): Task;

  getSides(): Task.Sides;
  setSides(value: Task.Sides): Task;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Task.AsObject;
  static toObject(includeInstance: boolean, msg: Task): Task.AsObject;
  static serializeBinaryToWriter(message: Task, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Task;
  static deserializeBinaryFromReader(message: Task, reader: jspb.BinaryReader): Task;
}

export namespace Task {
  export type AsObject = {
    name: string;
    document: Uint8Array | string;
    numberUp: number;
    copies: number;
    jobSheet: Task.JobSheet;
    media: Task.Media;
    orientation: Task.Orientation;
    quality: Task.Quality;
    sides: Task.Sides;
  };

  export enum JobSheet {
    STANDARD = 0,
    UNCLASSIFIED = 1,
    CLASSIFIED = 2,
    CONFIDENTIAL = 3,
    SECRET = 4,
    TOP_SECRET = 5,
  }

  export enum Media {
    A4 = 0,
    A3 = 2,
    LETTER = 3,
  }

  export enum Orientation {
    LABDSCAPE_COUNTER_CLOCKWISE90 = 0,
    LABDSCAPE_CLOCKWISE90 = 1,
    REVERSE_PORTRAIT = 2,
  }

  export enum Quality {
    DRAFT = 0,
    NORMAL = 1,
    BEST = 2,
  }

  export enum Sides {
    ONE = 0,
    TWO_LONG = 1,
    TWO_SHORT = 2,
  }
}

