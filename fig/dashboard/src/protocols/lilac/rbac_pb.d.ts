import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"


export class User extends jspb.Message {
  getId(): number;
  setId(value: number): User;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): User.AsObject;
  static toObject(includeInstance: boolean, msg: User): User.AsObject;
  static serializeBinaryToWriter(message: User, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): User;
  static deserializeBinaryFromReader(message: User, reader: jspb.BinaryReader): User;
}

export namespace User {
  export type AsObject = {
    id: number,
  }
}

export class UserDetail extends jspb.Message {
  getId(): number;
  setId(value: number): UserDetail;

  getProvider(): UserDetail.Provider | undefined;
  setProvider(value?: UserDetail.Provider): UserDetail;
  hasProvider(): boolean;
  clearProvider(): UserDetail;

  getRealName(): string;
  setRealName(value: string): UserDetail;

  getAvatar(): string;
  setAvatar(value: string): UserDetail;
  hasAvatar(): boolean;
  clearAvatar(): UserDetail;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserDetail.AsObject;
  static toObject(includeInstance: boolean, msg: UserDetail): UserDetail.AsObject;
  static serializeBinaryToWriter(message: UserDetail, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserDetail;
  static deserializeBinaryFromReader(message: UserDetail, reader: jspb.BinaryReader): UserDetail;
}

export namespace UserDetail {
  export type AsObject = {
    id: number,
    provider?: UserDetail.Provider.AsObject,
    realName: string,
    avatar?: string,
  }

  export class Provider extends jspb.Message {
    getType(): UserDetail.Provider.Type;
    setType(value: UserDetail.Provider.Type): Provider;

    getId(): string;
    setId(value: string): Provider;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Provider.AsObject;
    static toObject(includeInstance: boolean, msg: Provider): Provider.AsObject;
    static serializeBinaryToWriter(message: Provider, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Provider;
    static deserializeBinaryFromReader(message: Provider, reader: jspb.BinaryReader): Provider;
  }

  export namespace Provider {
    export type AsObject = {
      type: UserDetail.Provider.Type,
      id: string,
    }

    export enum Type { 
      EMAIL = 0,
      GOOGLE = 1,
      FACEBOOK = 2,
      APPLE = 3,
      WECHATMINIPROGRAM = 91,
      WECHATOAUTH = 92,
    }
  }


  export enum AvatarCase { 
    _AVATAR_NOT_SET = 0,
    AVATAR = 9,
  }
}

export class Role extends jspb.Message {
  getRoot(): Role.Root | undefined;
  setRoot(value?: Role.Root): Role;
  hasRoot(): boolean;
  clearRoot(): Role;

  getAdministrator(): Role.Administrator | undefined;
  setAdministrator(value?: Role.Administrator): Role;
  hasAdministrator(): boolean;
  clearAdministrator(): Role;

  getMember(): string;
  setMember(value: string): Role;

  getByCase(): Role.ByCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Role.AsObject;
  static toObject(includeInstance: boolean, msg: Role): Role.AsObject;
  static serializeBinaryToWriter(message: Role, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Role;
  static deserializeBinaryFromReader(message: Role, reader: jspb.BinaryReader): Role;
}

export namespace Role {
  export type AsObject = {
    root?: Role.Root.AsObject,
    administrator?: Role.Administrator.AsObject,
    member: string,
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


  export enum ByCase { 
    BY_NOT_SET = 0,
    ROOT = 1,
    ADMINISTRATOR = 2,
    MEMBER = 3,
  }
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

export class Permission extends jspb.Message {
  getOperation(): string;
  setOperation(value: string): Permission;

  getResource(): Resource | undefined;
  setResource(value?: Resource): Permission;
  hasResource(): boolean;
  clearResource(): Permission;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Permission.AsObject;
  static toObject(includeInstance: boolean, msg: Permission): Permission.AsObject;
  static serializeBinaryToWriter(message: Permission, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Permission;
  static deserializeBinaryFromReader(message: Permission, reader: jspb.BinaryReader): Permission;
}

export namespace Permission {
  export type AsObject = {
    operation: string,
    resource?: Resource.AsObject,
  }
}

export class PolicyRoleRequest extends jspb.Message {
  getName(): string;
  setName(value: string): PolicyRoleRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyRoleRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyRoleRequest): PolicyRoleRequest.AsObject;
  static serializeBinaryToWriter(message: PolicyRoleRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyRoleRequest;
  static deserializeBinaryFromReader(message: PolicyRoleRequest, reader: jspb.BinaryReader): PolicyRoleRequest;
}

export namespace PolicyRoleRequest {
  export type AsObject = {
    name: string,
  }
}

export class PolicyRolesForUserRequest extends jspb.Message {
  getUser(): number;
  setUser(value: number): PolicyRolesForUserRequest;

  getRolesList(): Array<Role>;
  setRolesList(value: Array<Role>): PolicyRolesForUserRequest;
  clearRolesList(): PolicyRolesForUserRequest;
  addRoles(value?: Role, index?: number): Role;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyRolesForUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyRolesForUserRequest): PolicyRolesForUserRequest.AsObject;
  static serializeBinaryToWriter(message: PolicyRolesForUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyRolesForUserRequest;
  static deserializeBinaryFromReader(message: PolicyRolesForUserRequest, reader: jspb.BinaryReader): PolicyRolesForUserRequest;
}

export namespace PolicyRolesForUserRequest {
  export type AsObject = {
    user: number,
    rolesList: Array<Role.AsObject>,
  }
}

export class PolicyPermissionsForUserRequest extends jspb.Message {
  getUser(): number;
  setUser(value: number): PolicyPermissionsForUserRequest;

  getPermissionsList(): Array<Permission>;
  setPermissionsList(value: Array<Permission>): PolicyPermissionsForUserRequest;
  clearPermissionsList(): PolicyPermissionsForUserRequest;
  addPermissions(value?: Permission, index?: number): Permission;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyPermissionsForUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyPermissionsForUserRequest): PolicyPermissionsForUserRequest.AsObject;
  static serializeBinaryToWriter(message: PolicyPermissionsForUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyPermissionsForUserRequest;
  static deserializeBinaryFromReader(message: PolicyPermissionsForUserRequest, reader: jspb.BinaryReader): PolicyPermissionsForUserRequest;
}

export namespace PolicyPermissionsForUserRequest {
  export type AsObject = {
    user: number,
    permissionsList: Array<Permission.AsObject>,
  }
}

export class PolicyUsersForRoleRequest extends jspb.Message {
  getRole(): Role | undefined;
  setRole(value?: Role): PolicyUsersForRoleRequest;
  hasRole(): boolean;
  clearRole(): PolicyUsersForRoleRequest;

  getUsersList(): Array<number>;
  setUsersList(value: Array<number>): PolicyUsersForRoleRequest;
  clearUsersList(): PolicyUsersForRoleRequest;
  addUsers(value: number, index?: number): PolicyUsersForRoleRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyUsersForRoleRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyUsersForRoleRequest): PolicyUsersForRoleRequest.AsObject;
  static serializeBinaryToWriter(message: PolicyUsersForRoleRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyUsersForRoleRequest;
  static deserializeBinaryFromReader(message: PolicyUsersForRoleRequest, reader: jspb.BinaryReader): PolicyUsersForRoleRequest;
}

export namespace PolicyUsersForRoleRequest {
  export type AsObject = {
    role?: Role.AsObject,
    usersList: Array<number>,
  }
}

export class PolicyPermissionsForRoleRequest extends jspb.Message {
  getRole(): string;
  setRole(value: string): PolicyPermissionsForRoleRequest;

  getPermissionsList(): Array<Permission>;
  setPermissionsList(value: Array<Permission>): PolicyPermissionsForRoleRequest;
  clearPermissionsList(): PolicyPermissionsForRoleRequest;
  addPermissions(value?: Permission, index?: number): Permission;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyPermissionsForRoleRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyPermissionsForRoleRequest): PolicyPermissionsForRoleRequest.AsObject;
  static serializeBinaryToWriter(message: PolicyPermissionsForRoleRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyPermissionsForRoleRequest;
  static deserializeBinaryFromReader(message: PolicyPermissionsForRoleRequest, reader: jspb.BinaryReader): PolicyPermissionsForRoleRequest;
}

export namespace PolicyPermissionsForRoleRequest {
  export type AsObject = {
    role: string,
    permissionsList: Array<Permission.AsObject>,
  }
}

export class PolicyRolesResponse extends jspb.Message {
  getItemsList(): Array<Role>;
  setItemsList(value: Array<Role>): PolicyRolesResponse;
  clearItemsList(): PolicyRolesResponse;
  addItems(value?: Role, index?: number): Role;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyRolesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyRolesResponse): PolicyRolesResponse.AsObject;
  static serializeBinaryToWriter(message: PolicyRolesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyRolesResponse;
  static deserializeBinaryFromReader(message: PolicyRolesResponse, reader: jspb.BinaryReader): PolicyRolesResponse;
}

export namespace PolicyRolesResponse {
  export type AsObject = {
    itemsList: Array<Role.AsObject>,
  }
}

export class PolicyUsersResponse extends jspb.Message {
  getItemsList(): Array<UserDetail>;
  setItemsList(value: Array<UserDetail>): PolicyUsersResponse;
  clearItemsList(): PolicyUsersResponse;
  addItems(value?: UserDetail, index?: number): UserDetail;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyUsersResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyUsersResponse): PolicyUsersResponse.AsObject;
  static serializeBinaryToWriter(message: PolicyUsersResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyUsersResponse;
  static deserializeBinaryFromReader(message: PolicyUsersResponse, reader: jspb.BinaryReader): PolicyUsersResponse;
}

export namespace PolicyUsersResponse {
  export type AsObject = {
    itemsList: Array<UserDetail.AsObject>,
  }
}

export class PolicyPermissionsResponse extends jspb.Message {
  getItemsList(): Array<Permission>;
  setItemsList(value: Array<Permission>): PolicyPermissionsResponse;
  clearItemsList(): PolicyPermissionsResponse;
  addItems(value?: Permission, index?: number): Permission;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PolicyPermissionsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PolicyPermissionsResponse): PolicyPermissionsResponse.AsObject;
  static serializeBinaryToWriter(message: PolicyPermissionsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PolicyPermissionsResponse;
  static deserializeBinaryFromReader(message: PolicyPermissionsResponse, reader: jspb.BinaryReader): PolicyPermissionsResponse;
}

export namespace PolicyPermissionsResponse {
  export type AsObject = {
    itemsList: Array<Permission.AsObject>,
  }
}

