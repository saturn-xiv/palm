import { query, ISucceed } from "./graphql";

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

export interface IPostgreSqlStatus {
  version: string;
  migrations: string[];
}
export interface IRedisStatus {
  nodes: string[];
}

export interface ISystemStatusResponse {
  postgresql: IPostgreSqlStatus;
  redis: IRedisStatus;
}

export const system_status = async (): Promise<ISystemStatusResponse> => {
  const res = await query<{
    postgresqlStatus: IPostgreSqlStatus;
    redisStatus: IRedisStatus;
  }>(
    `
query call{
  postgresqlStatus{
    version, migrations
  }
  redisStatus{
    nodes
  }
}
`,
    {}
  );
  return { postgresql: res.postgresqlStatus, redis: res.redisStatus };
};

export const set_site_favicon = async (url: string): Promise<ISucceed> => {
  const res = await query<{
    setSiteFavicon: ISucceed;
  }>(
    `
mutation call($url: String!){
  setSiteFavicon(url: $url){
    createdAt
  }
}
`,
    { url }
  );
  return res.setSiteFavicon;
};
export const delete_site_gab_code = async (): Promise<ISucceed> => {
  const res = await query<{
    deleteSiteGabCode: ISucceed;
  }>(
    `
mutation call{
  deleteSiteGabCode{
    createdAt
  }
}
`,
    {}
  );
  return res.deleteSiteGabCode;
};
export const set_site_gab_code = async (
  code: string,
  name: string
): Promise<ISucceed> => {
  const res = await query<{
    setSiteGabCode: ISucceed;
  }>(
    `
mutation call($code: String!, $name: String!){
  setSiteGabCode(code: $code, name: $name){
    createdAt
  }
}
`,
    { code, name }
  );
  return res.setSiteGabCode;
};
export const delete_site_icp_code = async (): Promise<ISucceed> => {
  const res = await query<{
    deleteSiteIcpCode: ISucceed;
  }>(
    `
mutation call{
  deleteSiteIcpCode{
    createdAt
  }
}
`,
    {}
  );
  return res.deleteSiteIcpCode;
};
export const set_site_icp_code = async (code: string): Promise<ISucceed> => {
  const res = await query<{
    setSiteIcpCode: ISucceed;
  }>(
    `
mutation call($code: String!){
  setSiteIcpCode(code: $code){
    createdAt
  }
}
`,
    { code }
  );
  return res.setSiteIcpCode;
};
export const set_site_keywords = async (items: string[]): Promise<ISucceed> => {
  const res = await query<{
    setSiteKeywords: ISucceed;
  }>(
    `
mutation call($items: [String!]!){
  setSiteKeywords(items: $items){
    createdAt
  }
}
`,
    { items }
  );
  return res.setSiteKeywords;
};
export const set_site_authors = async (items: IAuthor[]): Promise<ISucceed> => {
  const res = await query<{
    setSiteAuthors: ISucceed;
  }>(
    `
mutation call($items: [SiteAuthorRequestItem!]!){
  setSiteAuthors(items: $items){
    createdAt
  }
}
`,
    { items }
  );
  return res.setSiteAuthors;
};
export const set_site_info = async (
  title: string,
  subhead: string,
  description: string,
  copyright: string
): Promise<ISucceed> => {
  const res = await query<{
    setSiteInfo: ISucceed;
  }>(
    `
mutation call($title: String!, $subhead: String!, $description: String!, $copyright: String!){
  setSiteInfo(title: $title, subhead: $subhead, description: $description, copyright: $copyright){
    createdAt
  }
}
`,
    { title, subhead, description, copyright }
  );
  return res.setSiteInfo;
};

interface IBaiduSiteVerification {
  content: string;
  code: string;
}
export const set_baidu_site_verification = async (
  code: string,
  content: string
): Promise<ISucceed> => {
  const res = await query<{
    setBaiduSiteVerification: ISucceed;
  }>(
    `
mutation call($code: String!, $content: String!){
  setBaiduSiteVerification(code: $code, content: $content){
    createdAt
  }
}
`,
    { content, code }
  );
  return res.setBaiduSiteVerification;
};
export const delete_baidu_site_verification = async (): Promise<ISucceed> => {
  const res = await query<{
    deleteBaiduSiteVerification: ISucceed;
  }>(
    `
mutation call{
  deleteBaiduSiteVerification{
    createdAt
  }
}
`,
    {}
  );
  return res.deleteBaiduSiteVerification;
};
export const get_baidu_site_verification =
  async (): Promise<IBaiduSiteVerification> => {
    const res = await query<{
      getBaiduSiteVerification: IBaiduSiteVerification;
    }>(
      `
query call{
  getBaiduSiteVerification{
    content, code
  }
}
`,
      {}
    );
    return res.getBaiduSiteVerification;
  };

interface IGoogleSiteVerification {
  code: string;
}
export const set_google_site_verification = async (
  code: string
): Promise<ISucceed> => {
  const res = await query<{
    setGoogleSiteVerification: ISucceed;
  }>(
    `
mutation call($code: String!){
  setGoogleSiteVerification(code: $code){
    createdAt
  }
}
`,
    { code }
  );
  return res.setGoogleSiteVerification;
};
export const delete_google_site_verification = async (): Promise<ISucceed> => {
  const res = await query<{
    deleteGoogleSiteVerification: ISucceed;
  }>(
    `
mutation call{
  deleteGoogleSiteVerification{
    createdAt
  }
}
`,
    {}
  );
  return res.deleteGoogleSiteVerification;
};
export const get_google_site_verification =
  async (): Promise<IGoogleSiteVerification> => {
    const res = await query<{
      getGoogleSiteVerification: IGoogleSiteVerification;
    }>(
      `
query call{
  getGoogleSiteVerification{
    code
  }
}
`,
      {}
    );
    return res.getGoogleSiteVerification;
  };

interface IGoogleRecaptcha {
  siteKey: string;
  secret: string;
}
export const set_google_recaptcha = async (
  siteKey: string,
  secret: string
): Promise<ISucceed> => {
  const res = await query<{
    setGoogleRecaptcha: ISucceed;
  }>(
    `
  mutation call($siteKey: String!, $secret: String!){
    setGoogleRecaptcha(siteKey: $siteKey, secret: $secret){
      createdAt
    }
  }
  `,
    { siteKey, secret }
  );
  return res.setGoogleRecaptcha;
};
export const delete_google_recaptcha = async (): Promise<ISucceed> => {
  const res = await query<{
    deleteGoogleRecaptcha: ISucceed;
  }>(
    `
  mutation call{
    deleteGoogleRecaptcha{
      createdAt
    }
  }
  `,
    {}
  );
  return res.deleteGoogleRecaptcha;
};
export const get_google_recaptcha = async (): Promise<IGoogleRecaptcha> => {
  const res = await query<{
    getGoogleRecaptcha: IGoogleRecaptcha;
  }>(
    `
  query call{
    getGoogleRecaptcha{
      siteKey, secret
    }
  }
  `,
    {}
  );
  return res.getGoogleRecaptcha;
};

interface IIndexNowSiteVerification {
  key: string;
}
export const set_index_now_site_verification = async (
  key: string
): Promise<ISucceed> => {
  const res = await query<{
    setIndexNowSiteVerification: ISucceed;
  }>(
    `
  mutation call($key: String!){
    setIndexNowSiteVerification(key: $key){
      createdAt
    }
  }
  `,
    { key }
  );
  return res.setIndexNowSiteVerification;
};
export const delete_index_now_site_verification =
  async (): Promise<ISucceed> => {
    const res = await query<{
      deleteIndexNowSiteVerification: ISucceed;
    }>(
      `
  mutation call{
    deleteIndexNowSiteVerification{
      createdAt
    }
  }
  `,
      {}
    );
    return res.deleteIndexNowSiteVerification;
  };
export const get_index_now_site_verification =
  async (): Promise<IIndexNowSiteVerification> => {
    const res = await query<{
      getIndexNowSiteVerification: IIndexNowSiteVerification;
    }>(
      `
  query call{
    getIndexNowSiteVerification{
      key
    }
  }
  `,
      {}
    );
    return res.getIndexNowSiteVerification;
  };
