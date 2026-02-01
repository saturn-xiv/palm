import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"
import * as portal_pb from './portal_pb'; // proto import: "portal.proto"


export class Bootstrap5Theme extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Bootstrap5Theme.AsObject;
  static toObject(includeInstance: boolean, msg: Bootstrap5Theme): Bootstrap5Theme.AsObject;
  static serializeBinaryToWriter(message: Bootstrap5Theme, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Bootstrap5Theme;
  static deserializeBinaryFromReader(message: Bootstrap5Theme, reader: jspb.BinaryReader): Bootstrap5Theme;
}

export namespace Bootstrap5Theme {
  export type AsObject = {
  };
}

export class BulmaTheme extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BulmaTheme.AsObject;
  static toObject(includeInstance: boolean, msg: BulmaTheme): BulmaTheme.AsObject;
  static serializeBinaryToWriter(message: BulmaTheme, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BulmaTheme;
  static deserializeBinaryFromReader(message: BulmaTheme, reader: jspb.BinaryReader): BulmaTheme;
}

export namespace BulmaTheme {
  export type AsObject = {
  };
}

export class Settings extends jspb.Message {
  getTitle(): string;
  setTitle(value: string): Settings;

  getDescription(): string;
  setDescription(value: string): Settings;

  getLanguage(): string;
  setLanguage(value: string): Settings;

  getFavicon(): portal_pb.File | undefined;
  setFavicon(value?: portal_pb.File): Settings;
  hasFavicon(): boolean;
  clearFavicon(): Settings;

  getBootstrap5(): Bootstrap5Theme | undefined;
  setBootstrap5(value?: Bootstrap5Theme): Settings;
  hasBootstrap5(): boolean;
  clearBootstrap5(): Settings;

  getBulma(): BulmaTheme | undefined;
  setBulma(value?: BulmaTheme): Settings;
  hasBulma(): boolean;
  clearBulma(): Settings;

  getThemeCase(): Settings.ThemeCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Settings.AsObject;
  static toObject(includeInstance: boolean, msg: Settings): Settings.AsObject;
  static serializeBinaryToWriter(message: Settings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Settings;
  static deserializeBinaryFromReader(message: Settings, reader: jspb.BinaryReader): Settings;
}

export namespace Settings {
  export type AsObject = {
    title: string;
    description: string;
    language: string;
    favicon?: portal_pb.File.AsObject;
    bootstrap5?: Bootstrap5Theme.AsObject;
    bulma?: BulmaTheme.AsObject;
  };

  export enum ThemeCase {
    THEME_NOT_SET = 0,
    BOOTSTRAP5 = 91,
    BULMA = 92,
  }
}

export class IndexPostResponse extends jspb.Message {
  getPagination(): portal_pb.Pagination | undefined;
  setPagination(value?: portal_pb.Pagination): IndexPostResponse;
  hasPagination(): boolean;
  clearPagination(): IndexPostResponse;

  getItemsList(): Array<IndexPostResponse.Item>;
  setItemsList(value: Array<IndexPostResponse.Item>): IndexPostResponse;
  clearItemsList(): IndexPostResponse;
  addItems(value?: IndexPostResponse.Item, index?: number): IndexPostResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IndexPostResponse.AsObject;
  static toObject(includeInstance: boolean, msg: IndexPostResponse): IndexPostResponse.AsObject;
  static serializeBinaryToWriter(message: IndexPostResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IndexPostResponse;
  static deserializeBinaryFromReader(message: IndexPostResponse, reader: jspb.BinaryReader): IndexPostResponse;
}

export namespace IndexPostResponse {
  export type AsObject = {
    pagination?: portal_pb.Pagination.AsObject;
    itemsList: Array<IndexPostResponse.Item.AsObject>;
  };

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getTitle(): string;
    setTitle(value: string): Item;

    getBody(): portal_pb.RichText | undefined;
    setBody(value?: portal_pb.RichText): Item;
    hasBody(): boolean;
    clearBody(): Item;

    getLabelsList(): Array<string>;
    setLabelsList(value: Array<string>): Item;
    clearLabelsList(): Item;
    addLabels(value: string, index?: number): Item;

    getPublishedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setPublishedAt(value?: google_protobuf_timestamp_pb.Timestamp): Item;
    hasPublishedAt(): boolean;
    clearPublishedAt(): Item;

    getPermalink(): string;
    setPermalink(value: string): Item;

    getLocation(): portal_pb.Location | undefined;
    setLocation(value?: portal_pb.Location): Item;
    hasLocation(): boolean;
    clearLocation(): Item;

    getOptions(): IndexPostResponse.Item.Options | undefined;
    setOptions(value?: IndexPostResponse.Item.Options): Item;
    hasOptions(): boolean;
    clearOptions(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      id: number;
      title: string;
      body?: portal_pb.RichText.AsObject;
      labelsList: Array<string>;
      publishedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject;
      permalink: string;
      location?: portal_pb.Location.AsObject;
      options?: IndexPostResponse.Item.Options.AsObject;
    };

    export class Options extends jspb.Message {
      getReaderComments(): ReaderComments;
      setReaderComments(value: ReaderComments): Options;

      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): Options.AsObject;
      static toObject(includeInstance: boolean, msg: Options): Options.AsObject;
      static serializeBinaryToWriter(message: Options, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): Options;
      static deserializeBinaryFromReader(message: Options, reader: jspb.BinaryReader): Options;
    }

    export namespace Options {
      export type AsObject = {
        readerComments: ReaderComments;
      };
    }


    export enum LocationCase {
      _LOCATION_NOT_SET = 0,
      LOCATION = 14,
    }
  }

}

export class IndexPageResponse extends jspb.Message {
  getPagination(): portal_pb.Pagination | undefined;
  setPagination(value?: portal_pb.Pagination): IndexPageResponse;
  hasPagination(): boolean;
  clearPagination(): IndexPageResponse;

  getItemsList(): Array<IndexPageResponse.Item>;
  setItemsList(value: Array<IndexPageResponse.Item>): IndexPageResponse;
  clearItemsList(): IndexPageResponse;
  addItems(value?: IndexPageResponse.Item, index?: number): IndexPageResponse.Item;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IndexPageResponse.AsObject;
  static toObject(includeInstance: boolean, msg: IndexPageResponse): IndexPageResponse.AsObject;
  static serializeBinaryToWriter(message: IndexPageResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IndexPageResponse;
  static deserializeBinaryFromReader(message: IndexPageResponse, reader: jspb.BinaryReader): IndexPageResponse;
}

export namespace IndexPageResponse {
  export type AsObject = {
    pagination?: portal_pb.Pagination.AsObject;
    itemsList: Array<IndexPageResponse.Item.AsObject>;
  };

  export class Item extends jspb.Message {
    getId(): number;
    setId(value: number): Item;

    getTitle(): string;
    setTitle(value: string): Item;

    getBody(): portal_pb.RichText | undefined;
    setBody(value?: portal_pb.RichText): Item;
    hasBody(): boolean;
    clearBody(): Item;

    getPermalink(): string;
    setPermalink(value: string): Item;

    getOptions(): IndexPageResponse.Item.Options | undefined;
    setOptions(value?: IndexPageResponse.Item.Options): Item;
    hasOptions(): boolean;
    clearOptions(): Item;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Item.AsObject;
    static toObject(includeInstance: boolean, msg: Item): Item.AsObject;
    static serializeBinaryToWriter(message: Item, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Item;
    static deserializeBinaryFromReader(message: Item, reader: jspb.BinaryReader): Item;
  }

  export namespace Item {
    export type AsObject = {
      id: number;
      title: string;
      body?: portal_pb.RichText.AsObject;
      permalink: string;
      options?: IndexPageResponse.Item.Options.AsObject;
    };

    export class Options extends jspb.Message {
      getReaderComments(): ReaderComments;
      setReaderComments(value: ReaderComments): Options;

      serializeBinary(): Uint8Array;
      toObject(includeInstance?: boolean): Options.AsObject;
      static toObject(includeInstance: boolean, msg: Options): Options.AsObject;
      static serializeBinaryToWriter(message: Options, writer: jspb.BinaryWriter): void;
      static deserializeBinary(bytes: Uint8Array): Options;
      static deserializeBinaryFromReader(message: Options, reader: jspb.BinaryReader): Options;
    }

    export namespace Options {
      export type AsObject = {
        readerComments: ReaderComments;
      };
    }

  }

}

export enum ReaderComments {
  ALLOW = 0,
  SHOW = 1,
  HIDE = 2,
}
