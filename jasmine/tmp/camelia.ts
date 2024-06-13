import moment from "moment-timezone";

import { query } from "../src/api/graphql";
import { home_url } from "../utils";
import { IPermission } from "../reducers/current-user";
import { upload } from ".";

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

export interface IUserDetails {
  nickname: string;
  realName: string;
  avatar: string;
}

export interface ICurrencyOption {
  id: number;
  code: string;
  name: string;
  unit: number;
}

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

export const ping_baidu = async (home: string): Promise<ISucceed> => {
  const res = await query<{
    pingBaidu: ISucceed;
  }>(
    `
query call($home: String!){
  pingBaidu(home: $home){
    createdAt
  }
}
`,
    { home }
  );
  return res.pingBaidu;
};

export const ping_google = async (home: string): Promise<ISucceed> => {
  const res = await query<{
    pingGoogle: ISucceed;
  }>(
    `
query call($home: String!){
  pingGoogle(home: $home){
    createdAt
  }
}
`,
    { home }
  );
  return res.pingGoogle;
};

export const ping_index_now = async (home: string): Promise<ISucceed> => {
  const res = await query<{
    pingIndexNow: ISucceed;
  }>(
    `
query call($home: String!){
  pingIndexNow(home: $home){
    createdAt
  }
}
`,
    { home }
  );
  return res.pingIndexNow;
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

export const currency_options = async (): Promise<ICurrencyOption[]> => {
  const res = await query<{
    currencyOptions: ICurrencyOption[];
  }>(
    `
query call{
  currencyOptions{
    id, code, name, unit
  }
}
`,
    {}
  );
  return res.currencyOptions;
};

export interface IAttachmentShow {
  id: number;
  title: string;
  url: string;
}

export interface IAttachmentShowResponse {
  url: string;
  title: string;
  size: number;
  contentType: string;
  updatedAt: Date;
}
export const show_attachment_by_id = async (
  id: number,
  ttl: number
): Promise<IAttachmentShowResponse> => {
  const res = await query<{ showAttachmentById: IAttachmentShowResponse }>(
    `
query call($id: Int!, $ttl: Int){
  showAttachmentById(id: $id, ttl: $ttl){
    url, title, size, contentType, updatedAt
  }
}
`,
    {
      id,
      ttl,
    }
  );
  return res.showAttachmentById;
};

export const update_attachment = async (
  id: number,
  title: string
): Promise<ISucceed> => {
  const res = await query<{
    updateAttachment: ISucceed;
  }>(
    `
  mutation call($id: Int!, $title: String!){
    updateAttachment(id: $id, title: $title){
      createdAt
    }
  }
  `,
    { id, title }
  );
  return res.updateAttachment;
};
export interface IAttachment {
  id: number;
  title: string;
  bucket: string;
  name: string;
  size: number;
  contentType: string;
  status: string;
  updatedAt: Date;
}
export const upload_attachment = async (
  files: File[]
): Promise<IAttachment[]> => {
  const res = await upload<IAttachment[]>("/api/attachments", files);
  return res;
};

export interface IIndexAttachmentResponse {
  items: IAttachment[];
  pagination: IPagination;
}
export const index_attachment = async (
  page: number,
  size: number
): Promise<IIndexAttachmentResponse> => {
  const res = await query<{ indexAttachment: IIndexAttachmentResponse }>(
    `
query call($pager: Pager!){
  indexAttachment(pager: $pager){
    items{id, title, bucket, name, size, contentType, status, updatedAt},
    pagination{page, size, total, hasNext, hasPrevious}
  }
}
`,
    {
      pager: { page, size },
    }
  );
  return res.indexAttachment;
};
export const index_picture = async (): Promise<IAttachment[]> => {
  const res = await query<{ indexPicture: IAttachment[] }>(
    `
query call{
  indexPicture{
    id, title, bucket, name, size, contentType, status, updatedAt
  }
}
`,
    {}
  );
  return res.indexPicture;
};

export const update_profile = async (
  realName: string,
  avatar: string,
  lang: string,
  timezone: string
): Promise<ISucceed> => {
  const res = await query<{ updateUserProfile: ISucceed }>(
    `
mutation call($realName: String!, $avatar: String!, $lang: String!, $timezone: String!){
  updateUserProfile(realName: $realName, avatar: $avatar, lang: $lang, timezone: $timezone){
    createdAt
  }
}
`,
    {
      realName,
      avatar,
      lang,
      timezone,
    }
  );
  return res.updateUserProfile;
};
export const change_password = async (
  currentPassword: string,
  newPassword: string
): Promise<ISucceed> => {
  const res = await query<{ changeUserPassword: ISucceed }>(
    `
mutation call($currentPassword: String!, $newPassword: String!){
  changeUserPassword(currentPassword: $currentPassword, newPassword: $newPassword){
    createdAt
  }
}
`,
    {
      currentPassword,
      newPassword,
    }
  );
  return res.changeUserPassword;
};

export interface IAuthor {
  name: string;
  email: string;
}

interface ISiteInfoResponse {
  favicon: string;
  title: string;
  subhead: string;
  description: string;
  copyright: string;
  keywords: string[];
  languages: string[];
  authors: IAuthor[];
  icpCode?: IIcpCode;
  gabCode?: IGabCode;
}

export const set_locale = async (
  lang: string,
  code: string,
  message: string
): Promise<ISucceed> => {
  const res = await query<{ setLocale: ISucceed }>(
    `
mutation call($lang: String!, $code: String!, $message: String!){
  setLocale(lang: $lang, code: $code, message: $message){
    createdAt
  }
}
`,
    { lang, code, message }
  );
  return res.setLocale;
};

export const destroy_locale = async (id: number): Promise<ISucceed> => {
  const res = await query<{ destroyLocale: ISucceed }>(
    `
mutation call($id: Int!){
  destroyLocale(id: $id){
    createdAt
  }
}
`,
    { id }
  );
  return res.destroyLocale;
};
