import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb'; // proto import: "google/protobuf/empty.proto"


export class Object extends jspb.Message {
  getType(): string;
  setType(value: string): Object;

  getId(): number;
  setId(value: number): Object;
  hasId(): boolean;
  clearId(): Object;

  getCode(): string;
  setCode(value: string): Object;
  hasCode(): boolean;
  clearCode(): Object;

  getAll(): google_protobuf_empty_pb.Empty | undefined;
  setAll(value?: google_protobuf_empty_pb.Empty): Object;
  hasAll(): boolean;
  clearAll(): Object;

  getByCase(): Object.ByCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Object.AsObject;
  static toObject(includeInstance: boolean, msg: Object): Object.AsObject;
  static serializeBinaryToWriter(message: Object, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Object;
  static deserializeBinaryFromReader(message: Object, reader: jspb.BinaryReader): Object;
}

export namespace Object {
  export type AsObject = {
    type: string;
    id?: number;
    code?: string;
    all?: google_protobuf_empty_pb.Empty.AsObject;
  };

  export enum ByCase {
    BY_NOT_SET = 0,
    ID = 11,
    CODE = 12,
    ALL = 19,
  }
}

export class Subject extends jspb.Message {
  getUser(): Subject.User | undefined;
  setUser(value?: Subject.User): Subject;
  hasUser(): boolean;
  clearUser(): Subject;

  getRole(): Subject.Role | undefined;
  setRole(value?: Subject.Role): Subject;
  hasRole(): boolean;
  clearRole(): Subject;

  getByCase(): Subject.ByCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Subject.AsObject;
  static toObject(includeInstance: boolean, msg: Subject): Subject.AsObject;
  static serializeBinaryToWriter(message: Subject, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Subject;
  static deserializeBinaryFromReader(message: Subject, reader: jspb.BinaryReader): Subject;
}

export namespace Subject {
  export type AsObject = {
    user?: Subject.User.AsObject;
    role?: Subject.Role.AsObject;
  };

  export class Role extends jspb.Message {
    getRoot(): Subject.Role.Root | undefined;
    setRoot(value?: Subject.Role.Root): Role;
    hasRoot(): boolean;
    clearRoot(): Role;

    getAdministrator(): Subject.Role.Administrator | undefined;
    setAdministrator(value?: Subject.Role.Administrator): Role;
    hasAdministrator(): boolean;
    clearAdministrator(): Role;

    getId(): number;
    setId(value: number): Role;
    hasId(): boolean;
    clearId(): Role;

    getCode(): string;
    setCode(value: string): Role;
    hasCode(): boolean;
    clearCode(): Role;

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
      root?: Subject.Role.Root.AsObject;
      administrator?: Subject.Role.Administrator.AsObject;
      id?: number;
      code?: string;
    };

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
      };
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
      };
    }


    export enum ByCase {
      BY_NOT_SET = 0,
      ROOT = 1,
      ADMINISTRATOR = 2,
      ID = 8,
      CODE = 9,
    }
  }


  export class User extends jspb.Message {
    getId(): number;
    setId(value: number): User;
    hasId(): boolean;
    clearId(): User;

    getCode(): string;
    setCode(value: string): User;
    hasCode(): boolean;
    clearCode(): User;

    getByCase(): User.ByCase;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): User.AsObject;
    static toObject(includeInstance: boolean, msg: User): User.AsObject;
    static serializeBinaryToWriter(message: User, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): User;
    static deserializeBinaryFromReader(message: User, reader: jspb.BinaryReader): User;
  }

  export namespace User {
    export type AsObject = {
      id?: number;
      code?: string;
    };

    export enum ByCase {
      BY_NOT_SET = 0,
      ID = 1,
      CODE = 2,
    }
  }


  export enum ByCase {
    BY_NOT_SET = 0,
    USER = 1,
    ROLE = 2,
  }
}

export class Action extends jspb.Message {
  getRead(): Action.Read | undefined;
  setRead(value?: Action.Read): Action;
  hasRead(): boolean;
  clearRead(): Action;

  getWrite(): Action.Write | undefined;
  setWrite(value?: Action.Write): Action;
  hasWrite(): boolean;
  clearWrite(): Action;

  getAppend(): Action.Append | undefined;
  setAppend(value?: Action.Append): Action;
  hasAppend(): boolean;
  clearAppend(): Action;

  getExecute(): Action.Execute | undefined;
  setExecute(value?: Action.Execute): Action;
  hasExecute(): boolean;
  clearExecute(): Action;

  getCredit(): Action.Credit | undefined;
  setCredit(value?: Action.Credit): Action;
  hasCredit(): boolean;
  clearCredit(): Action;

  getDebit(): Action.Debit | undefined;
  setDebit(value?: Action.Debit): Action;
  hasDebit(): boolean;
  clearDebit(): Action;

  getInquiry(): Action.Inquiry | undefined;
  setInquiry(value?: Action.Inquiry): Action;
  hasInquiry(): boolean;
  clearInquiry(): Action;

  getCode(): string;
  setCode(value: string): Action;
  hasCode(): boolean;
  clearCode(): Action;

  getByCase(): Action.ByCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Action.AsObject;
  static toObject(includeInstance: boolean, msg: Action): Action.AsObject;
  static serializeBinaryToWriter(message: Action, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Action;
  static deserializeBinaryFromReader(message: Action, reader: jspb.BinaryReader): Action;
}

export namespace Action {
  export type AsObject = {
    read?: Action.Read.AsObject;
    write?: Action.Write.AsObject;
    append?: Action.Append.AsObject;
    execute?: Action.Execute.AsObject;
    credit?: Action.Credit.AsObject;
    debit?: Action.Debit.AsObject;
    inquiry?: Action.Inquiry.AsObject;
    code?: string;
  };

  export class Read extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Read.AsObject;
    static toObject(includeInstance: boolean, msg: Read): Read.AsObject;
    static serializeBinaryToWriter(message: Read, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Read;
    static deserializeBinaryFromReader(message: Read, reader: jspb.BinaryReader): Read;
  }

  export namespace Read {
    export type AsObject = {
    };
  }


  export class Write extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Write.AsObject;
    static toObject(includeInstance: boolean, msg: Write): Write.AsObject;
    static serializeBinaryToWriter(message: Write, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Write;
    static deserializeBinaryFromReader(message: Write, reader: jspb.BinaryReader): Write;
  }

  export namespace Write {
    export type AsObject = {
    };
  }


  export class Append extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Append.AsObject;
    static toObject(includeInstance: boolean, msg: Append): Append.AsObject;
    static serializeBinaryToWriter(message: Append, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Append;
    static deserializeBinaryFromReader(message: Append, reader: jspb.BinaryReader): Append;
  }

  export namespace Append {
    export type AsObject = {
    };
  }


  export class Execute extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Execute.AsObject;
    static toObject(includeInstance: boolean, msg: Execute): Execute.AsObject;
    static serializeBinaryToWriter(message: Execute, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Execute;
    static deserializeBinaryFromReader(message: Execute, reader: jspb.BinaryReader): Execute;
  }

  export namespace Execute {
    export type AsObject = {
    };
  }


  export class Credit extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Credit.AsObject;
    static toObject(includeInstance: boolean, msg: Credit): Credit.AsObject;
    static serializeBinaryToWriter(message: Credit, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Credit;
    static deserializeBinaryFromReader(message: Credit, reader: jspb.BinaryReader): Credit;
  }

  export namespace Credit {
    export type AsObject = {
    };
  }


  export class Debit extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Debit.AsObject;
    static toObject(includeInstance: boolean, msg: Debit): Debit.AsObject;
    static serializeBinaryToWriter(message: Debit, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Debit;
    static deserializeBinaryFromReader(message: Debit, reader: jspb.BinaryReader): Debit;
  }

  export namespace Debit {
    export type AsObject = {
    };
  }


  export class Inquiry extends jspb.Message {
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Inquiry.AsObject;
    static toObject(includeInstance: boolean, msg: Inquiry): Inquiry.AsObject;
    static serializeBinaryToWriter(message: Inquiry, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Inquiry;
    static deserializeBinaryFromReader(message: Inquiry, reader: jspb.BinaryReader): Inquiry;
  }

  export namespace Inquiry {
    export type AsObject = {
    };
  }


  export enum ByCase {
    BY_NOT_SET = 0,
    READ = 1,
    WRITE = 2,
    APPEND = 3,
    EXECUTE = 4,
    CREDIT = 5,
    DEBIT = 6,
    INQUIRY = 7,
    CODE = 9,
  }
}

export class Permission extends jspb.Message {
  getSubject(): Subject | undefined;
  setSubject(value?: Subject): Permission;
  hasSubject(): boolean;
  clearSubject(): Permission;

  getObject(): Object | undefined;
  setObject(value?: Object): Permission;
  hasObject(): boolean;
  clearObject(): Permission;

  getAction(): Action | undefined;
  setAction(value?: Action): Permission;
  hasAction(): boolean;
  clearAction(): Permission;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Permission.AsObject;
  static toObject(includeInstance: boolean, msg: Permission): Permission.AsObject;
  static serializeBinaryToWriter(message: Permission, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Permission;
  static deserializeBinaryFromReader(message: Permission, reader: jspb.BinaryReader): Permission;
}

export namespace Permission {
  export type AsObject = {
    subject?: Subject.AsObject;
    object?: Object.AsObject;
    action?: Action.AsObject;
  };
}

export class UserRoleRequest extends jspb.Message {
  getUser(): Subject.User | undefined;
  setUser(value?: Subject.User): UserRoleRequest;
  hasUser(): boolean;
  clearUser(): UserRoleRequest;

  getRole(): Subject.Role | undefined;
  setRole(value?: Subject.Role): UserRoleRequest;
  hasRole(): boolean;
  clearRole(): UserRoleRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserRoleRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UserRoleRequest): UserRoleRequest.AsObject;
  static serializeBinaryToWriter(message: UserRoleRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserRoleRequest;
  static deserializeBinaryFromReader(message: UserRoleRequest, reader: jspb.BinaryReader): UserRoleRequest;
}

export namespace UserRoleRequest {
  export type AsObject = {
    user?: Subject.User.AsObject;
    role?: Subject.Role.AsObject;
  };
}

export class RolesResponse extends jspb.Message {
  getItemsList(): Array<Subject.Role>;
  setItemsList(value: Array<Subject.Role>): RolesResponse;
  clearItemsList(): RolesResponse;
  addItems(value?: Subject.Role, index?: number): Subject.Role;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RolesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: RolesResponse): RolesResponse.AsObject;
  static serializeBinaryToWriter(message: RolesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RolesResponse;
  static deserializeBinaryFromReader(message: RolesResponse, reader: jspb.BinaryReader): RolesResponse;
}

export namespace RolesResponse {
  export type AsObject = {
    itemsList: Array<Subject.Role.AsObject>;
  };
}

export class UsersResponse extends jspb.Message {
  getItemsList(): Array<Subject.User>;
  setItemsList(value: Array<Subject.User>): UsersResponse;
  clearItemsList(): UsersResponse;
  addItems(value?: Subject.User, index?: number): Subject.User;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UsersResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UsersResponse): UsersResponse.AsObject;
  static serializeBinaryToWriter(message: UsersResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UsersResponse;
  static deserializeBinaryFromReader(message: UsersResponse, reader: jspb.BinaryReader): UsersResponse;
}

export namespace UsersResponse {
  export type AsObject = {
    itemsList: Array<Subject.User.AsObject>;
  };
}

export class SubjectsResponse extends jspb.Message {
  getItemsList(): Array<Subject>;
  setItemsList(value: Array<Subject>): SubjectsResponse;
  clearItemsList(): SubjectsResponse;
  addItems(value?: Subject, index?: number): Subject;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SubjectsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SubjectsResponse): SubjectsResponse.AsObject;
  static serializeBinaryToWriter(message: SubjectsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SubjectsResponse;
  static deserializeBinaryFromReader(message: SubjectsResponse, reader: jspb.BinaryReader): SubjectsResponse;
}

export namespace SubjectsResponse {
  export type AsObject = {
    itemsList: Array<Subject.AsObject>;
  };
}

export class ObjectsResponse extends jspb.Message {
  getItemsList(): Array<Object>;
  setItemsList(value: Array<Object>): ObjectsResponse;
  clearItemsList(): ObjectsResponse;
  addItems(value?: Object, index?: number): Object;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ObjectsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ObjectsResponse): ObjectsResponse.AsObject;
  static serializeBinaryToWriter(message: ObjectsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ObjectsResponse;
  static deserializeBinaryFromReader(message: ObjectsResponse, reader: jspb.BinaryReader): ObjectsResponse;
}

export namespace ObjectsResponse {
  export type AsObject = {
    itemsList: Array<Object.AsObject>;
  };
}

export class ActionsResponse extends jspb.Message {
  getItemsList(): Array<Action>;
  setItemsList(value: Array<Action>): ActionsResponse;
  clearItemsList(): ActionsResponse;
  addItems(value?: Action, index?: number): Action;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ActionsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ActionsResponse): ActionsResponse.AsObject;
  static serializeBinaryToWriter(message: ActionsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ActionsResponse;
  static deserializeBinaryFromReader(message: ActionsResponse, reader: jspb.BinaryReader): ActionsResponse;
}

export namespace ActionsResponse {
  export type AsObject = {
    itemsList: Array<Action.AsObject>;
  };
}

export class PermissionsResponse extends jspb.Message {
  getItemsList(): Array<Permission>;
  setItemsList(value: Array<Permission>): PermissionsResponse;
  clearItemsList(): PermissionsResponse;
  addItems(value?: Permission, index?: number): Permission;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PermissionsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PermissionsResponse): PermissionsResponse.AsObject;
  static serializeBinaryToWriter(message: PermissionsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PermissionsResponse;
  static deserializeBinaryFromReader(message: PermissionsResponse, reader: jspb.BinaryReader): PermissionsResponse;
}

export namespace PermissionsResponse {
  export type AsObject = {
    itemsList: Array<Permission.AsObject>;
  };
}

