import { get as http_get } from ".";

interface IGab {
  code: string;
  name: string;
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
  version: string;
  icp?: string;
  gab?: IGab;
}

export const fetch_layout = async (): Promise<ILayout> => {
  return await http_get(`/api/layout`);
};
