import graphql from "../../graphql";

export interface ILayout {
  favicon?: string;
  title: string;
  subhead: string;
  author: IAuthor;
  keywords: string[];
  description: string;
  copyright: string;
  languages: string[];
  version: string;
}

export interface IAuthor {
  name: string;
  email: string;
}

interface IApiVersionResponse {
  apiVersion: string;
}

export const version = async (): Promise<string> => {
  const res: IApiVersionResponse = await graphql(
    `
      query call {
        apiVersion
      }
    `,
    {},
  );
  return res.apiVersion;
};

interface IBuildTimeResponse {
  buildTime: string;
}

export const build_time = async (): Promise<string> => {
  const res: IBuildTimeResponse = await graphql(
    `
      query call {
        buildTime
      }
    `,
    {},
  );
  return res.buildTime;
};
