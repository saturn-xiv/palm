import graphql from "../../../graphql";

export interface ILayout {
  favicon?: string;
  title: string;
  subhead: string;
  author: IAuthor;
  keywords: string[];
  description: string;
  copyright: string;
  locale: string;
  languages: string[];
  buildTime: Date;
  project: IProject;
}

export interface IAuthor {
  name: string;
  email: string;
}
export interface IProject {
  name: string;
  url: string;
  version: string;
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
