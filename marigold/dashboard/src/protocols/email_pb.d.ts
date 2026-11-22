import * as jspb from 'google-protobuf'



export class Task extends jspb.Message {
  constructor();
  constructor(opt_data?: Task.AsObject);
  getToList(): Array<Task.Address>;
  setToList(value: Array<Task.Address>): Task;
  clearToList(): Task;
  addTo(value?: Task.Address, index?: number): Task.Address;

  getReplyTo(): Task.Address | undefined;
  setReplyTo(value?: Task.Address): Task;
  hasReplyTo(): boolean;
  clearReplyTo(): Task;

  getCcList(): Array<Task.Address>;
  setCcList(value: Array<Task.Address>): Task;
  clearCcList(): Task;
  addCc(value?: Task.Address, index?: number): Task.Address;

  getBccList(): Array<Task.Address>;
  setBccList(value: Array<Task.Address>): Task;
  clearBccList(): Task;
  addBcc(value?: Task.Address, index?: number): Task.Address;

  getSubject(): string;
  setSubject(value: string): Task;

  getBody(): Task.Body | undefined;
  setBody(value?: Task.Body): Task;
  hasBody(): boolean;
  clearBody(): Task;

  getAttachmentsList(): Array<Task.Attachment>;
  setAttachmentsList(value: Array<Task.Attachment>): Task;
  clearAttachmentsList(): Task;
  addAttachments(value?: Task.Attachment, index?: number): Task.Attachment;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Task.AsObject;
  static toObject(includeInstance: boolean, msg: Task): Task.AsObject;
  static serializeBinaryToWriter(message: Task, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Task;
  static deserializeBinaryFromReader(message: Task, reader: jspb.BinaryReader): Task;
}

export namespace Task {
  export type AsObject = {
    toList: Array<Task.Address.AsObject>;
    replyTo?: Task.Address.AsObject;
    ccList: Array<Task.Address.AsObject>;
    bccList: Array<Task.Address.AsObject>;
    subject: string;
    body?: Task.Body.AsObject;
    attachmentsList: Array<Task.Attachment.AsObject>;
  };

  export class Address extends jspb.Message {
    constructor();
    constructor(opt_data?: Address.AsObject);
    getName(): string;
    setName(value: string): Address;
    hasName(): boolean;
    clearName(): Address;

    getEmail(): string;
    setEmail(value: string): Address;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Address.AsObject;
    static toObject(includeInstance: boolean, msg: Address): Address.AsObject;
    static serializeBinaryToWriter(message: Address, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Address;
    static deserializeBinaryFromReader(message: Address, reader: jspb.BinaryReader): Address;
  }

  export namespace Address {
    export type AsObject = {
      name?: string;
      email: string;
    };

    export enum NameCase {
      _NAME_NOT_SET = 0,
      NAME = 1,
    }
  }


  export class Body extends jspb.Message {
    constructor();
    constructor(opt_data?: Body.AsObject);
    getHtml(): Task.Html;
    setHtml(value: Task.Html): Body;
    hasHtml(): boolean;
    clearHtml(): Body;

    getContent(): string;
    setContent(value: string): Body;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Body.AsObject;
    static toObject(includeInstance: boolean, msg: Body): Body.AsObject;
    static serializeBinaryToWriter(message: Body, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Body;
    static deserializeBinaryFromReader(message: Body, reader: jspb.BinaryReader): Body;
  }

  export namespace Body {
    export type AsObject = {
      html?: Task.Html;
      content: string;
    };

    export enum HtmlCase {
      _HTML_NOT_SET = 0,
      HTML = 1,
    }
  }


  export class Attachment extends jspb.Message {
    constructor();
    constructor(opt_data?: Attachment.AsObject);
    getInlineId(): string;
    setInlineId(value: string): Attachment;
    hasInlineId(): boolean;
    clearInlineId(): Attachment;

    getName(): string;
    setName(value: string): Attachment;

    getContentType(): string;
    setContentType(value: string): Attachment;

    getContent(): Uint8Array | string;
    getContent_asU8(): Uint8Array;
    getContent_asB64(): string;
    setContent(value: Uint8Array | string): Attachment;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Attachment.AsObject;
    static toObject(includeInstance: boolean, msg: Attachment): Attachment.AsObject;
    static serializeBinaryToWriter(message: Attachment, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Attachment;
    static deserializeBinaryFromReader(message: Attachment, reader: jspb.BinaryReader): Attachment;
  }

  export namespace Attachment {
    export type AsObject = {
      inlineId?: string;
      name: string;
      contentType: string;
      content: Uint8Array | string;
    };

    export enum InlineIdCase {
      _INLINE_ID_NOT_SET = 0,
      INLINE_ID = 1,
    }
  }


  export enum Html {
    NONE = 0,
    FOUNDATION = 1,
  }

  export enum ReplyToCase {
    _REPLY_TO_NOT_SET = 0,
    REPLY_TO = 2,
  }
}

