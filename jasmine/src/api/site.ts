import { query } from "./graphql";

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
