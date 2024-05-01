import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"


export class EmailSendRequest extends jspb.Message {
  getTo(): EmailSendRequest.Address | undefined;
  setTo(value?: EmailSendRequest.Address): EmailSendRequest;
  hasTo(): boolean;
  clearTo(): EmailSendRequest;

  getCcList(): Array<EmailSendRequest.Address>;
  setCcList(value: Array<EmailSendRequest.Address>): EmailSendRequest;
  clearCcList(): EmailSendRequest;
  addCc(value?: EmailSendRequest.Address, index?: number): EmailSendRequest.Address;

  getBccList(): Array<EmailSendRequest.Address>;
  setBccList(value: Array<EmailSendRequest.Address>): EmailSendRequest;
  clearBccList(): EmailSendRequest;
  addBcc(value?: EmailSendRequest.Address, index?: number): EmailSendRequest.Address;

  getSubject(): string;
  setSubject(value: string): EmailSendRequest;

  getBody(): EmailSendRequest.Body | undefined;
  setBody(value?: EmailSendRequest.Body): EmailSendRequest;
  hasBody(): boolean;
  clearBody(): EmailSendRequest;

  getAttachmentsList(): Array<EmailSendRequest.Attachment>;
  setAttachmentsList(value: Array<EmailSendRequest.Attachment>): EmailSendRequest;
  clearAttachmentsList(): EmailSendRequest;
  addAttachments(value?: EmailSendRequest.Attachment, index?: number): EmailSendRequest.Attachment;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EmailSendRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EmailSendRequest): EmailSendRequest.AsObject;
  static serializeBinaryToWriter(message: EmailSendRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EmailSendRequest;
  static deserializeBinaryFromReader(message: EmailSendRequest, reader: jspb.BinaryReader): EmailSendRequest;
}

export namespace EmailSendRequest {
  export type AsObject = {
    to?: EmailSendRequest.Address.AsObject,
    ccList: Array<EmailSendRequest.Address.AsObject>,
    bccList: Array<EmailSendRequest.Address.AsObject>,
    subject: string,
    body?: EmailSendRequest.Body.AsObject,
    attachmentsList: Array<EmailSendRequest.Attachment.AsObject>,
  }

  export class Address extends jspb.Message {
    getName(): string;
    setName(value: string): Address;

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
      name: string,
      email: string,
    }
  }


  export class Body extends jspb.Message {
    getPayload(): string;
    setPayload(value: string): Body;

    getHtml(): boolean;
    setHtml(value: boolean): Body;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Body.AsObject;
    static toObject(includeInstance: boolean, msg: Body): Body.AsObject;
    static serializeBinaryToWriter(message: Body, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Body;
    static deserializeBinaryFromReader(message: Body, reader: jspb.BinaryReader): Body;
  }

  export namespace Body {
    export type AsObject = {
      payload: string,
      html: boolean,
    }
  }


  export class Attachment extends jspb.Message {
    getTitle(): string;
    setTitle(value: string): Attachment;

    getPayload(): Uint8Array | string;
    getPayload_asU8(): Uint8Array;
    getPayload_asB64(): string;
    setPayload(value: Uint8Array | string): Attachment;

    getInline(): boolean;
    setInline(value: boolean): Attachment;

    getContentType(): string;
    setContentType(value: string): Attachment;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Attachment.AsObject;
    static toObject(includeInstance: boolean, msg: Attachment): Attachment.AsObject;
    static serializeBinaryToWriter(message: Attachment, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Attachment;
    static deserializeBinaryFromReader(message: Attachment, reader: jspb.BinaryReader): Attachment;
  }

  export namespace Attachment {
    export type AsObject = {
      title: string,
      payload: Uint8Array | string,
      inline: boolean,
      contentType: string,
    }
  }

}

