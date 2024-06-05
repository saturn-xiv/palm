import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';
import * as google_protobuf_duration_pb from 'google-protobuf/google/protobuf/duration_pb';


export class PolicyHasRequest extends jspb.Message {
  getUser(): PolicyUsersResponse.Item | undefined;
  setUser(value?: PolicyUsersResponse.Item): PolicyHasRequest;
  hasUser(): boolean;
  clearUser(): PolicyHasRequest;

  getRole(): PolicyRolesResponse.Item | undefined;
  setRole(value?: PolicyRolesResponse.Item): PolicyHasRequest;
  hasRole(): boolean;
  clearRole(): PolicyHasRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyHasRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyHasRequest): PolicyHasRequest.AsObject;
  static serializeBinaryToWriter(message: PolicyHasRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyHasRequest;
  static deserializeBinaryFromReader(message: PolicyHasRequest, reader: jspb.BinaryReader): PolicyHasRequest;
}

export namespace PolicyHasRequest {
  export type AsObject = {
    user?: PolicyUsersResponse.Item.AsObject,
    role?: PolicyRolesResponse.Item.AsObject,
  }
}

export class PolicyCanRequest extends jspb.Message {
  getUser(): PolicyUsersResponse.Item | undefined;
  setUser(value?: PolicyUsersResponse.Item): PolicyCanRequest;
  hasUser(): boolean;
  clearUser(): PolicyCanRequest;

  getResource(): PolicyPermissionsResponse.Item.Resource | undefined;
  setResource(value?: PolicyPermissionsResponse.Item.Resource): PolicyCanRequest;
  hasResource(): boolean;
  clearResource(): PolicyCanRequest;

  getOperation(): PolicyPermissionsResponse.Item.Operation | undefined;
  setOperation(value?: PolicyPermissionsResponse.Item.Operation): PolicyCanRequest;
  hasOperation(): boolean;
  clearOperation(): PolicyCanRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyCanRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyCanRequest): PolicyCanRequest.AsObject;
  static serializeBinaryToWriter(message: PolicyCanRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyCanRequest;
  static deserializeBinaryFromReader(message: PolicyCanRequest, reader: jspb.BinaryReader): PolicyCanRequest;
}

export namespace PolicyCanRequest {
  export type AsObject = {
    user?: PolicyUsersResponse.Item.AsObject,
    resource?: PolicyPermissionsResponse.Item.Resource.AsObject,
    operation?: PolicyPermissionsResponse.Item.Operation.AsObject,
  }
}

export class PolicyUsersResponse extends jspb.Message {
  getItemsList(): Array<PolicyUsersResponse.Item>;
  setItemsList(value: Array<PolicyUsersResponse.Item>): PolicyUsersResponse;
  clearItemsList(): PolicyUsersResponse;
  addItems(value?: PolicyUsersResponse.Item, index?: number): PolicyUsersResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyUsersResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyUsersResponse): PolicyUsersResponse.AsObject;
  static serializeBinaryToWriter(message: PolicyUsersResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyUsersResponse;
  static deserializeBinaryFromReader(message: PolicyUsersResponse, reader: jspb.BinaryReader): PolicyUsersResponse;
}

export namespace PolicyUsersResponse {
  export type AsObject = {
    itemsList: Array<PolicyUsersResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getIid(): number;
    setIid(value: number): Item;

    getLid(): number;
    setLid(value: number): Item;

    getCode(): string;
    setCode(value: string): Item;

    getByCase(): Item.ByCase;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      iid: number,
      lid: number,
      code: string,
    }

    export enum ByCase { 
      BY_NOT_SET = 0,
      IID = 1,
      LID = 2,
      CODE = 9,
    }
  }

}

export class PolicyRolesResponse extends jspb.Message {
  getItemsList(): Array<PolicyRolesResponse.Item>;
  setItemsList(value: Array<PolicyRolesResponse.Item>): PolicyRolesResponse;
  clearItemsList(): PolicyRolesResponse;
  addItems(value?: PolicyRolesResponse.Item, index?: number): PolicyRolesResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyRolesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyRolesResponse): PolicyRolesResponse.AsObject;
  static serializeBinaryToWriter(message: PolicyRolesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyRolesResponse;
  static deserializeBinaryFromReader(message: PolicyRolesResponse, reader: jspb.BinaryReader): PolicyRolesResponse;
}

export namespace PolicyRolesResponse {
  export type AsObject = {
    itemsList: Array<PolicyRolesResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getRoot(): PolicyRolesResponse.Item.Root | undefined;
    setRoot(value?: PolicyRolesResponse.Item.Root): Item;
    hasRoot(): boolean;
    clearRoot(): Item;

    getAdministrator(): PolicyRolesResponse.Item.Administrator | undefined;
    setAdministrator(value?: PolicyRolesResponse.Item.Administrator): Item;
    hasAdministrator(): boolean;
    clearAdministrator(): Item;

    getCode(): string;
    setCode(value: string): Item;

    getByCase(): Item.ByCase;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      root?: PolicyRolesResponse.Item.Root.AsObject,
      administrator?: PolicyRolesResponse.Item.Administrator.AsObject,
      code: string,
    }

    export class Administrator extends jspb.Message {
      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): Administrator.AsObject;
      static toObject(includeInstance: boolean, msg: Administrator): Administrator.AsObject;
      static serializeBinaryToWriter(message: Administrator, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): Administrator;
      static deserializeBinaryFromReader(message: Administrator, reader: jspb.BinaryReader): Administrator;
    }

    export namespace Administrator {
      export type AsObject = {
      }
    }


    export class Root extends jspb.Message {
      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): Root.AsObject;
      static toObject(includeInstance: boolean, msg: Root): Root.AsObject;
      static serializeBinaryToWriter(message: Root, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): Root;
      static deserializeBinaryFromReader(message: Root, reader: jspb.BinaryReader): Root;
    }

    export namespace Root {
      export type AsObject = {
      }
    }


    export enum ByCase { 
      BY_NOT_SET = 0,
      ROOT = 1,
      ADMINISTRATOR = 2,
      CODE = 9,
    }
  }

}

export class PolicyRolesForUserRequest extends jspb.Message {
  getUser(): PolicyUsersResponse.Item | undefined;
  setUser(value?: PolicyUsersResponse.Item): PolicyRolesForUserRequest;
  hasUser(): boolean;
  clearUser(): PolicyRolesForUserRequest;

  getRolesList(): Array<PolicyRolesResponse.Item>;
  setRolesList(value: Array<PolicyRolesResponse.Item>): PolicyRolesForUserRequest;
  clearRolesList(): PolicyRolesForUserRequest;
  addRoles(value?: PolicyRolesResponse.Item, index?: number): PolicyRolesResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyRolesForUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyRolesForUserRequest): PolicyRolesForUserRequest.AsObject;
  static serializeBinaryToWriter(message: PolicyRolesForUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyRolesForUserRequest;
  static deserializeBinaryFromReader(message: PolicyRolesForUserRequest, reader: jspb.BinaryReader): PolicyRolesForUserRequest;
}

export namespace PolicyRolesForUserRequest {
  export type AsObject = {
    user?: PolicyUsersResponse.Item.AsObject,
    rolesList: Array<PolicyRolesResponse.Item.AsObject>,
  }
}

export class PolicyPermissionsForUserRequest extends jspb.Message {
  getUser(): PolicyUsersResponse.Item | undefined;
  setUser(value?: PolicyUsersResponse.Item): PolicyPermissionsForUserRequest;
  hasUser(): boolean;
  clearUser(): PolicyPermissionsForUserRequest;

  getPermissionsList(): Array<PolicyPermissionsResponse.Item>;
  setPermissionsList(value: Array<PolicyPermissionsResponse.Item>): PolicyPermissionsForUserRequest;
  clearPermissionsList(): PolicyPermissionsForUserRequest;
  addPermissions(value?: PolicyPermissionsResponse.Item, index?: number): PolicyPermissionsResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyPermissionsForUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyPermissionsForUserRequest): PolicyPermissionsForUserRequest.AsObject;
  static serializeBinaryToWriter(message: PolicyPermissionsForUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyPermissionsForUserRequest;
  static deserializeBinaryFromReader(message: PolicyPermissionsForUserRequest, reader: jspb.BinaryReader): PolicyPermissionsForUserRequest;
}

export namespace PolicyPermissionsForUserRequest {
  export type AsObject = {
    user?: PolicyUsersResponse.Item.AsObject,
    permissionsList: Array<PolicyPermissionsResponse.Item.AsObject>,
  }
}

export class PolicyPermissionsForRoleRequest extends jspb.Message {
  getRole(): PolicyRolesResponse.Item | undefined;
  setRole(value?: PolicyRolesResponse.Item): PolicyPermissionsForRoleRequest;
  hasRole(): boolean;
  clearRole(): PolicyPermissionsForRoleRequest;

  getPermissionsList(): Array<PolicyPermissionsResponse.Item>;
  setPermissionsList(value: Array<PolicyPermissionsResponse.Item>): PolicyPermissionsForRoleRequest;
  clearPermissionsList(): PolicyPermissionsForRoleRequest;
  addPermissions(value?: PolicyPermissionsResponse.Item, index?: number): PolicyPermissionsResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyPermissionsForRoleRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyPermissionsForRoleRequest): PolicyPermissionsForRoleRequest.AsObject;
  static serializeBinaryToWriter(message: PolicyPermissionsForRoleRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyPermissionsForRoleRequest;
  static deserializeBinaryFromReader(message: PolicyPermissionsForRoleRequest, reader: jspb.BinaryReader): PolicyPermissionsForRoleRequest;
}

export namespace PolicyPermissionsForRoleRequest {
  export type AsObject = {
    role?: PolicyRolesResponse.Item.AsObject,
    permissionsList: Array<PolicyPermissionsResponse.Item.AsObject>,
  }
}

export class PolicyPermissionsResponse extends jspb.Message {
  getItemsList(): Array<PolicyPermissionsResponse.Item>;
  setItemsList(value: Array<PolicyPermissionsResponse.Item>): PolicyPermissionsResponse;
  clearItemsList(): PolicyPermissionsResponse;
  addItems(value?: PolicyPermissionsResponse.Item, index?: number): PolicyPermissionsResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyPermissionsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyPermissionsResponse): PolicyPermissionsResponse.AsObject;
  static serializeBinaryToWriter(message: PolicyPermissionsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyPermissionsResponse;
  static deserializeBinaryFromReader(message: PolicyPermissionsResponse, reader: jspb.BinaryReader): PolicyPermissionsResponse;
}

export namespace PolicyPermissionsResponse {
  export type AsObject = {
    itemsList: Array<PolicyPermissionsResponse.Item.AsObject>,
  }

  export class Item extends jspb.Message {
    getResource(): PolicyPermissionsResponse.Item.Resource | undefined;
    setResource(value?: PolicyPermissionsResponse.Item.Resource): Item;
    hasResource(): boolean;
    clearResource(): Item;

    getOperation(): PolicyPermissionsResponse.Item.Operation | undefined;
    setOperation(value?: PolicyPermissionsResponse.Item.Operation): Item;
    hasOperation(): boolean;
    clearOperation(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      resource?: PolicyPermissionsResponse.Item.Resource.AsObject,
      operation?: PolicyPermissionsResponse.Item.Operation.AsObject,
    }

    export class Resource extends jspb.Message {
      getType(): string;
      setType(value: string): Resource;

      getId(): number;
      setId(value: number): Resource;
      hasId(): boolean;
      clearId(): Resource;

      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): Resource.AsObject;
      static toObject(includeInstance: boolean, msg: Resource): Resource.AsObject;
      static serializeBinaryToWriter(message: Resource, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): Resource;
      static deserializeBinaryFromReader(message: Resource, reader: jspb.BinaryReader): Resource;
    }

    export namespace Resource {
      export type AsObject = {
        type: string,
        id?: number,
      }

      export enum IdCase { 
        _ID_NOT_SET = 0,
        ID = 2,
      }
    }


    export class Operation extends jspb.Message {
      getCreate(): PolicyPermissionsResponse.Item.Operation.Create | undefined;
      setCreate(value?: PolicyPermissionsResponse.Item.Operation.Create): Operation;
      hasCreate(): boolean;
      clearCreate(): Operation;

      getShow(): PolicyPermissionsResponse.Item.Operation.Show | undefined;
      setShow(value?: PolicyPermissionsResponse.Item.Operation.Show): Operation;
      hasShow(): boolean;
      clearShow(): Operation;

      getUpdate(): PolicyPermissionsResponse.Item.Operation.Update | undefined;
      setUpdate(value?: PolicyPermissionsResponse.Item.Operation.Update): Operation;
      hasUpdate(): boolean;
      clearUpdate(): Operation;

      getDestroy(): PolicyPermissionsResponse.Item.Operation.Destroy | undefined;
      setDestroy(value?: PolicyPermissionsResponse.Item.Operation.Destroy): Operation;
      hasDestroy(): boolean;
      clearDestroy(): Operation;

      getManage(): PolicyPermissionsResponse.Item.Operation.Manage | undefined;
      setManage(value?: PolicyPermissionsResponse.Item.Operation.Manage): Operation;
      hasManage(): boolean;
      clearManage(): Operation;

      getCode(): string;
      setCode(value: string): Operation;

      getByCase(): Operation.ByCase;

      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): Operation.AsObject;
      static toObject(includeInstance: boolean, msg: Operation): Operation.AsObject;
      static serializeBinaryToWriter(message: Operation, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): Operation;
      static deserializeBinaryFromReader(message: Operation, reader: jspb.BinaryReader): Operation;
    }

    export namespace Operation {
      export type AsObject = {
        create?: PolicyPermissionsResponse.Item.Operation.Create.AsObject,
        show?: PolicyPermissionsResponse.Item.Operation.Show.AsObject,
        update?: PolicyPermissionsResponse.Item.Operation.Update.AsObject,
        destroy?: PolicyPermissionsResponse.Item.Operation.Destroy.AsObject,
        manage?: PolicyPermissionsResponse.Item.Operation.Manage.AsObject,
        code: string,
      }

      export class Create extends jspb.Message {
        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): Create.AsObject;
        static toObject(includeInstance: boolean, msg: Create): Create.AsObject;
        static serializeBinaryToWriter(message: Create, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): Create;
        static deserializeBinaryFromReader(message: Create, reader: jspb.BinaryReader): Create;
      }

      export namespace Create {
        export type AsObject = {
        }
      }


      export class Show extends jspb.Message {
        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): Show.AsObject;
        static toObject(includeInstance: boolean, msg: Show): Show.AsObject;
        static serializeBinaryToWriter(message: Show, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): Show;
        static deserializeBinaryFromReader(message: Show, reader: jspb.BinaryReader): Show;
      }

      export namespace Show {
        export type AsObject = {
        }
      }


      export class Update extends jspb.Message {
        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): Update.AsObject;
        static toObject(includeInstance: boolean, msg: Update): Update.AsObject;
        static serializeBinaryToWriter(message: Update, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): Update;
        static deserializeBinaryFromReader(message: Update, reader: jspb.BinaryReader): Update;
      }

      export namespace Update {
        export type AsObject = {
        }
      }


      export class Destroy extends jspb.Message {
        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): Destroy.AsObject;
        static toObject(includeInstance: boolean, msg: Destroy): Destroy.AsObject;
        static serializeBinaryToWriter(message: Destroy, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): Destroy;
        static deserializeBinaryFromReader(message: Destroy, reader: jspb.BinaryReader): Destroy;
      }

      export namespace Destroy {
        export type AsObject = {
        }
      }


      export class Manage extends jspb.Message {
        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): Manage.AsObject;
        static toObject(includeInstance: boolean, msg: Manage): Manage.AsObject;
        static serializeBinaryToWriter(message: Manage, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): Manage;
        static deserializeBinaryFromReader(message: Manage, reader: jspb.BinaryReader): Manage;
      }

      export namespace Manage {
        export type AsObject = {
        }
      }


      export enum ByCase { 
        BY_NOT_SET = 0,
        CREATE = 1,
        SHOW = 2,
        UPDATE = 3,
        DESTROY = 4,
        MANAGE = 9,
        CODE = 99,
      }
    }

  }

}

export class S3CreateBucketRequest extends jspb.Message {
  getName(): string;
  setName(value: string): S3CreateBucketRequest;

  getPublic(): boolean;
  setPublic(value: boolean): S3CreateBucketRequest;

  getExpirationDays(): number;
  setExpirationDays(value: number): S3CreateBucketRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): S3CreateBucketRequest.AsObject;
  static toObject(includeInstance: boolean, msg: S3CreateBucketRequest): S3CreateBucketRequest.AsObject;
  static serializeBinaryToWriter(message: S3CreateBucketRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): S3CreateBucketRequest;
  static deserializeBinaryFromReader(message: S3CreateBucketRequest, reader: jspb.BinaryReader): S3CreateBucketRequest;
}

export namespace S3CreateBucketRequest {
  export type AsObject = {
    name: string,
    pb_public: boolean,
    expirationDays: number,
  }
}

export class S3CreateBucketResponse extends jspb.Message {
  getName(): string;
  setName(value: string): S3CreateBucketResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): S3CreateBucketResponse.AsObject;
  static toObject(includeInstance: boolean, msg: S3CreateBucketResponse): S3CreateBucketResponse.AsObject;
  static serializeBinaryToWriter(message: S3CreateBucketResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): S3CreateBucketResponse;
  static deserializeBinaryFromReader(message: S3CreateBucketResponse, reader: jspb.BinaryReader): S3CreateBucketResponse;
}

export namespace S3CreateBucketResponse {
  export type AsObject = {
    name: string,
  }
}

export class S3UploadRequest extends jspb.Message {
  getBucket(): string;
  setBucket(value: string): S3UploadRequest;

  getObject(): string;
  setObject(value: string): S3UploadRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): S3UploadRequest;
  hasTtl(): boolean;
  clearTtl(): S3UploadRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): S3UploadRequest.AsObject;
  static toObject(includeInstance: boolean, msg: S3UploadRequest): S3UploadRequest.AsObject;
  static serializeBinaryToWriter(message: S3UploadRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): S3UploadRequest;
  static deserializeBinaryFromReader(message: S3UploadRequest, reader: jspb.BinaryReader): S3UploadRequest;
}

export namespace S3UploadRequest {
  export type AsObject = {
    bucket: string,
    object: string,
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class S3UploadUrlResponse extends jspb.Message {
  getUrl(): string;
  setUrl(value: string): S3UploadUrlResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): S3UploadUrlResponse.AsObject;
  static toObject(includeInstance: boolean, msg: S3UploadUrlResponse): S3UploadUrlResponse.AsObject;
  static serializeBinaryToWriter(message: S3UploadUrlResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): S3UploadUrlResponse;
  static deserializeBinaryFromReader(message: S3UploadUrlResponse, reader: jspb.BinaryReader): S3UploadUrlResponse;
}

export namespace S3UploadUrlResponse {
  export type AsObject = {
    url: string,
  }
}

export class PresignedUrlRequest extends jspb.Message {
  getBucket(): string;
  setBucket(value: string): PresignedUrlRequest;

  getObject(): string;
  setObject(value: string): PresignedUrlRequest;

  getTitle(): string;
  setTitle(value: string): PresignedUrlRequest;

  getTtl(): google_protobuf_duration_pb.Duration | undefined;
  setTtl(value?: google_protobuf_duration_pb.Duration): PresignedUrlRequest;
  hasTtl(): boolean;
  clearTtl(): PresignedUrlRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PresignedUrlRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PresignedUrlRequest): PresignedUrlRequest.AsObject;
  static serializeBinaryToWriter(message: PresignedUrlRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PresignedUrlRequest;
  static deserializeBinaryFromReader(message: PresignedUrlRequest, reader: jspb.BinaryReader): PresignedUrlRequest;
}

export namespace PresignedUrlRequest {
  export type AsObject = {
    bucket: string,
    object: string,
    title: string,
    ttl?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class PermanentUrlRequest extends jspb.Message {
  getBucket(): string;
  setBucket(value: string): PermanentUrlRequest;

  getObject(): string;
  setObject(value: string): PermanentUrlRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PermanentUrlRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PermanentUrlRequest): PermanentUrlRequest.AsObject;
  static serializeBinaryToWriter(message: PermanentUrlRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PermanentUrlRequest;
  static deserializeBinaryFromReader(message: PermanentUrlRequest, reader: jspb.BinaryReader): PermanentUrlRequest;
}

export namespace PermanentUrlRequest {
  export type AsObject = {
    bucket: string,
    object: string,
  }
}

export class SmsSendRequest extends jspb.Message {
  getToList(): Array<string>;
  setToList(value: Array<string>): SmsSendRequest;
  clearToList(): SmsSendRequest;
  addTo(value: string, index?: number): SmsSendRequest;

  getBody(): string;
  setBody(value: string): SmsSendRequest;

  getCallbackUri(): string;
  setCallbackUri(value: string): SmsSendRequest;
  hasCallbackUri(): boolean;
  clearCallbackUri(): SmsSendRequest;

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
    body: string,
    callbackUri?: string,
  }

  export enum CallbackUriCase { 
    _CALLBACK_URI_NOT_SET = 0,
    CALLBACK_URI = 3,
  }
}

export class EmailSendRequest extends jspb.Message {
  getSubject(): string;
  setSubject(value: string): EmailSendRequest;

  getBody(): EmailSendRequest.Body | undefined;
  setBody(value?: EmailSendRequest.Body): EmailSendRequest;
  hasBody(): boolean;
  clearBody(): EmailSendRequest;

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
    subject: string,
    body?: EmailSendRequest.Body.AsObject,
    to?: EmailSendRequest.Address.AsObject,
    ccList: Array<EmailSendRequest.Address.AsObject>,
    bccList: Array<EmailSendRequest.Address.AsObject>,
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
    getText(): string;
    setText(value: string): Body;

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
      text: string,
      html: boolean,
    }
  }


  export class Attachment extends jspb.Message {
    getTitle(): string;
    setTitle(value: string): Attachment;

    getContentType(): string;
    setContentType(value: string): Attachment;

    getInline(): boolean;
    setInline(value: boolean): Attachment;

    getBody(): Uint8Array | string;
    getBody_asU8(): Uint8Array;
    getBody_asB64(): string;
    setBody(value: Uint8Array | string): Attachment;

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
      contentType: string,
      inline: boolean,
      body: Uint8Array | string,
    }
  }

}

