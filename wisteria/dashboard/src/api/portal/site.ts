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
