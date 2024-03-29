import moment from "moment-timezone";

import { query } from "./graphql";
import { home_url } from "../utils";
import { IPermission } from "../reducers/current-user";
import { upload } from ".";

export const EDITOR_TEXTAREA = "TEXTAREA";
export const UTC = "UTC";

export interface ISucceed {
  createdAt: Date;
}

export interface IPagination {
  page: number;
  size: number;
  total: number;
  hasNext: boolean;
  hasPrevious: boolean;
}

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

export const ping_baidu = async (home: string): Promise<ISucceed> => {
  const res = await query<{
    pingBaidu: ISucceed;
  }>(
    `
mutation call($home: String!){
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
mutation call($home: String!){
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
mutation call($home: String!){
  pingIndexNow(home: $home){
    createdAt
  }
}
`,
    { home }
  );
  return res.pingIndexNow;
};

interface IBaiduVerification {
  content: string;
  code: string;
}
export const set_baidu_verification = async (
  content: string,
  code: string
): Promise<ISucceed> => {
  const res = await query<{
    setBaiduSiteVerification: ISucceed;
  }>(
    `
mutation call($content: String!, $code: String!){
  setBaiduSiteVerification(content: $content, code: $code){
    createdAt
  }
}
`,
    { content, code }
  );
  return res.setBaiduSiteVerification;
};
export const get_baidu_verification = async (): Promise<IBaiduVerification> => {
  const res = await query<{
    getBaiduSiteVerification: IBaiduVerification;
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
interface ILog {
  id: number;
  plugin: string;
  level: string;
  ip: string;
  resourceType: string;
  resourceId: number;
  message: string;
  createdAt: Date;
}
export interface IIndexLogResponse {
  items: ILog[];
  pagination: IPagination;
}
export const logs = async (
  page: number,
  size: number
): Promise<IIndexLogResponse> => {
  const res = await query<{ logs: IIndexLogResponse }>(
    `
query call($pager: Pager!){
  logs(pager: $pager){
    items{id, plugin, level, ip, resourceType, resourceId, message, createdAt},
    pagination{page, size, total, hasNext, hasPrevious}
  }
}
`,
    {
      pager: { page, size },
    }
  );
  return res.logs;
};

export interface ICurrentUser {
  realName: string;
  avatar: string;
  isAdministrator: boolean;
  isRoot: boolean;
  roles: string[];
  permissions: IPermission[];
  hasWechatMiniProgram: boolean;
  hasWechatOauth2: boolean;
  hasGoogle: boolean;
  providerType: string;
  lang: string;
  timezone: string;
}

export interface ISignInResponse {
  token: string;
  user: ICurrentUser;
}

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

export const sign_out = async (): Promise<ISucceed> => {
  const res = await query<{ signOutUser: ISucceed }>(
    `
mutation call{
  signOutUser{ createdAt }
}
`,
    {}
  );
  return res.signOutUser;
};

export const current_user = async (): Promise<ICurrentUser> => {
  const res = await query<{ currentUser: ICurrentUser }>(
    `
query call{
  currentUser{
    realName, avatar, providerType, lang, timezone,
    isAdministrator, isRoot,
    roles, 
    permissions{ 
      resource{type, sid, iid},
      action
    },
    hasWechatMiniProgram, hasWechatOauth2, hasGoogle
  }
}
`,
    {}
  );
  return res.currentUser;
};

export const sign_in_by_email = async (
  user: string,
  password: string
): Promise<ISignInResponse> => {
  const res = await query<{ signInUserByEmail: ISignInResponse }>(
    `
mutation call($user: String!, $password: String!){
  signInUserByEmail(user: $user, password: $password){    
    token, 
    user{
      realName, avatar, providerType, lang, timezone,
      isAdministrator, isRoot,
      roles, 
      permissions{ 
        resource{type, sid, iid},
        action
      },
      hasWechatMiniProgram, hasWechatOauth2, hasGoogle
    }
  }
}
`,
    {
      user,
      password,
      ttl: 60 * 60 * 24,
    }
  );
  return res.signInUserByEmail;
};

export const reset_password = async (
  token: string,
  password: string
): Promise<ISucceed> => {
  const res = await query<{ resetUserPassword: ISucceed }>(
    `
mutation call($token: String!, $password: String!){
  resetUserPassword(token: $token, password: $password){
    createdAt
  }
}
`,
    {
      token,
      password,
    }
  );
  return res.resetUserPassword;
};
export const forgot_password = async (user: string): Promise<ISucceed> => {
  const res = await query<{ forgotUserPassword: ISucceed }>(
    `
mutation call($user: String!, $home: String!){
  forgotUserPassword(user: $user, home: $home){
    createdAt
  }
}
`,
    {
      user,
      home: home_url(),
    }
  );
  return res.forgotUserPassword;
};
export const unlock_by_email = async (user: string): Promise<ISucceed> => {
  const res = await query<{ unlockUserByEmail: ISucceed }>(
    `
mutation call($user: String!, $home: String!){
  unlockUserByEmail(user: $user, home: $home){
    createdAt
  }
}
`,
    {
      user,
      home: home_url(),
    }
  );
  return res.unlockUserByEmail;
};

export const unlock_by_token = async (token: string): Promise<ISucceed> => {
  const res = await query<{ unlockUserByToken: ISucceed }>(
    `
mutation call($token: String!){
  unlockUserByToken(token: $token){
    createdAt
  }
}
`,
    {
      token,
    }
  );
  return res.unlockUserByToken;
};
export const confirm_by_email = async (user: string): Promise<ISucceed> => {
  const res = await query<{ confirmUserByEmail: ISucceed }>(
    `
mutation call($user: String!, $home: String!){
  confirmUserByEmail(user: $user, home: $home){
    createdAt
  }
}
`,
    {
      user,
      home: home_url(),
    }
  );
  return res.confirmUserByEmail;
};

export const confirm_by_token = async (token: string): Promise<ISucceed> => {
  const res = await query<{ confirmUserByToken: ISucceed }>(
    `
mutation call($token: String!){
  confirmUserByToken(token: $token){
    createdAt
  }
}
`,
    {
      token,
    }
  );
  return res.confirmUserByToken;
};
export const sign_up_by_email = async (
  realName: string,
  nickname: string,
  email: string,
  password: string
): Promise<ISucceed> => {
  const res = await query<{ signUpUserByEmail: ISucceed }>(
    `
mutation call($realName: String!, $nickname: String!, $email: String!, $password: String!, $home: String!, $timezone: String!){
  signUpUserByEmail(realName: $realName, nickname: $nickname, email: $email, password: $password, home: $home, timezone: $timezone){
    createdAt
  }
}
`,
    {
      realName,
      nickname,
      email,
      password,
      home: home_url(),
      timezone: moment.tz.guess(),
    }
  );
  return res.signUpUserByEmail;
};

export const create_leave_word = async (
  content: string,
  editor: string
): Promise<ISucceed> => {
  const res = await query<{ createLeaveWord: ISucceed }>(
    `
mutation call($content: String!, $editor: MediaTextEditor!){
  createLeaveWord(content: $content, editor: $editor){
    createdAt
  }
}
`,
    { content, editor }
  );
  return res.createLeaveWord;
};

export interface IAuthor {
  name: string;
  email: string;
}

export interface IGabCode {
  code: string;
  name: string;
}
interface ISiteInfoResponse {
  title: string;
  subhead: string;
  description: string;
  keywords: string[];
  languages: string[];
  authors: IAuthor[];
  icpCode?: string;
  gabCode?: IGabCode;
}
export interface ILayoutResponse {
  apiVersion: string;
  siteInfo: ISiteInfoResponse;
}
export const fetch_layout = async (lang: string): Promise<ILayoutResponse> => {
  const res = await query<ILayoutResponse>(
    `
query call($lang: String!){
  apiVersion
  siteInfo(lang: $lang){
    title, subhead, description, keywords, languages, 
    authors{name, email}
  }
}
`,
    { lang }
  );
  return res;
};

interface IIndexLocaleResponse {
  indexLocaleByLang: { code: string; message: string }[];
}

export const index_locale = async (
  lang: string
): Promise<Record<string, string>> => {
  const res = await query<IIndexLocaleResponse>(
    `
query call($lang: String!){
  indexLocaleByLang(lang: $lang){
    code, message
  }
}
`,
    { lang }
  );

  const messages = res.indexLocaleByLang.reduce(
    (ac, it) => ({ ...ac, [it.code]: it.message }),
    {}
  );
  return messages;
};
