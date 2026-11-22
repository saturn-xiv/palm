import * as jspb from 'google-protobuf'



export class BillPullRequest extends jspb.Message {
  constructor();
  constructor(opt_data?: BillPullRequest.AsObject);
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BillPullRequest.AsObject;
  static toObject(includeInstance: boolean, msg: BillPullRequest): BillPullRequest.AsObject;
  static serializeBinaryToWriter(message: BillPullRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BillPullRequest;
  static deserializeBinaryFromReader(message: BillPullRequest, reader: jspb.BinaryReader): BillPullRequest;
}

export namespace BillPullRequest {
  export type AsObject = {
  };
}

export class BillPullResponse extends jspb.Message {
  constructor();
  constructor(opt_data?: BillPullResponse.AsObject);
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BillPullResponse.AsObject;
  static toObject(includeInstance: boolean, msg: BillPullResponse): BillPullResponse.AsObject;
  static serializeBinaryToWriter(message: BillPullResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BillPullResponse;
  static deserializeBinaryFromReader(message: BillPullResponse, reader: jspb.BinaryReader): BillPullResponse;
}

export namespace BillPullResponse {
  export type AsObject = {
  };
}

