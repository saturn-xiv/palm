import * as jspb from 'google-protobuf'



export class PasswordBox extends jspb.Message {
  constructor();
  constructor(opt_data?: PasswordBox.AsObject);
  getPayload(): string;
  setPayload(value: string): PasswordBox;

  getSalt(): number;
  setSalt(value: number): PasswordBox;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PasswordBox.AsObject;
  static toObject(includeInstance: boolean, msg: PasswordBox): PasswordBox.AsObject;
  static serializeBinaryToWriter(message: PasswordBox, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PasswordBox;
  static deserializeBinaryFromReader(message: PasswordBox, reader: jspb.BinaryReader): PasswordBox;
}

export namespace PasswordBox {
  export type AsObject = {
    payload: string;
    salt: number;
  };
}

export class Oauth2State extends jspb.Message {
  constructor();
  constructor(opt_data?: Oauth2State.AsObject);
  getType(): Oauth2State.Type;
  setType(value: Oauth2State.Type): Oauth2State;

  getUser(): string;
  setUser(value: string): Oauth2State;
  hasUser(): boolean;
  clearUser(): Oauth2State;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Oauth2State.AsObject;
  static toObject(includeInstance: boolean, msg: Oauth2State): Oauth2State.AsObject;
  static serializeBinaryToWriter(message: Oauth2State, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Oauth2State;
  static deserializeBinaryFromReader(message: Oauth2State, reader: jspb.BinaryReader): Oauth2State;
}

export namespace Oauth2State {
  export type AsObject = {
    type: Oauth2State.Type;
    user?: string;
  };

  export enum Type {
    GOOGLE = 0,
    WECHAT = 1,
  }

  export enum UserCase {
    _USER_NOT_SET = 0,
    USER = 2,
  }
}

export class UserIndexResponse extends jspb.Message {
  constructor();
  constructor(opt_data?: UserIndexResponse.AsObject);
  getItemsList(): Array<UserIndexResponse.Item>;
  setItemsList(value: Array<UserIndexResponse.Item>): UserIndexResponse;
  clearItemsList(): UserIndexResponse;
  addItems(value?: UserIndexResponse.Item, index?: number): UserIndexResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserIndexResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UserIndexResponse): UserIndexResponse.AsObject;
  static serializeBinaryToWriter(message: UserIndexResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserIndexResponse;
  static deserializeBinaryFromReader(message: UserIndexResponse, reader: jspb.BinaryReader): UserIndexResponse;
}

export namespace UserIndexResponse {
  export type AsObject = {
    itemsList: Array<UserIndexResponse.Item.AsObject>;
  };

  export class Item extends jspb.Message {
    constructor();
    constructor(opt_data?: Item.AsObject);
    getSn(): string;
    setSn(value: string): Item;

    getType(): UserIndexResponse.Item.Type;
    setType(value: UserIndexResponse.Item.Type): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      sn: string;
      type: UserIndexResponse.Item.Type;
    };

    export enum Type {
      EMAIL = 0,
      SMS = 1,
      GOOGLE_OAUTH2 = 11,
      WECHAT_OAUTH2 = 12,
      WECHAT_MINI_PROGRAM = 13,
    }
  }

}

export class UserToken extends jspb.Message {
  constructor();
  constructor(opt_data?: UserToken.AsObject);
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserToken.AsObject;
  static toObject(includeInstance: boolean, msg: UserToken): UserToken.AsObject;
  static serializeBinaryToWriter(message: UserToken, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserToken;
  static deserializeBinaryFromReader(message: UserToken, reader: jspb.BinaryReader): UserToken;
}

export namespace UserToken {
  export type AsObject = {
  };

  export class Payload extends jspb.Message {
    constructor();
    constructor(opt_data?: Payload.AsObject);
    getType(): UserIndexResponse.Item.Type;
    setType(value: UserIndexResponse.Item.Type): Payload;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Payload.AsObject;
    static toObject(includeInstance: boolean, msg: Payload): Payload.AsObject;
    static serializeBinaryToWriter(message: Payload, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Payload;
    static deserializeBinaryFromReader(message: Payload, reader: jspb.BinaryReader): Payload;
  }

  export namespace Payload {
    export type AsObject = {
      type: UserIndexResponse.Item.Type;
    };
  }


  export enum Audience {
    SIGN_IN = 0,
    HYDRANGEA_JOB = 1,
  }
}

