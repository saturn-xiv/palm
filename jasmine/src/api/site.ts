import { query } from "./graphql";

export interface IRoute {
  path: string;
  name: string;
  children: IRoute[];
}

export const routes = async (): Promise<IRoute[]> => {
  const res = await query<{
    indexRoute: IRoute[];
  }>(
    `
query call{
  indexRoute{
    path, name,
    children {
      path, name, children{
        path, name, children{
          path, name
        }
      }
    }
  }
}
`,
    {}
  );
  return res.indexRoute;
};

interface IGab {
  code: string;
  name: string;
}
interface IIcp {
  code: string;
}
interface IAuthor {
  name: string;
  email: string;
}
export interface ILayout {
  favicon: string;
  title: string;
  subhead: string;
  description: string;
  author?: IAuthor;
  keywords: string[];
  copyright: string;
  locale: string;
  languages: string[];
  icp?: IIcp;
  gab?: IGab;
}

export interface ILayoutResponse {
  apiVersion: string;
  layout: ILayout;
}
export const fetch_layout = async (): Promise<ILayoutResponse> => {
  const res = await query<ILayoutResponse>(
    `
query call{
  apiVersion
  layout{
    favicon, title, subhead, description, keywords, copyright, 
    locale, languages, 
    author{name, email}, 
    icp{code}, 
    gab{code, name},
  }
}
`,
    {}
  );
  return res;
};
