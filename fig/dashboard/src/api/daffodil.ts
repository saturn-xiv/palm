import { get as detect_locale } from "../i18n";
import { query, ISucceed, EDITOR_TEXTAREA, IPagination, IPager } from ".";
import {
  ICurrentUser,
  IResource,
  mingle_password,
} from "../reducers/current-user";
import { IAuthor, ICnIcp, ICnMps, ISiteInfo } from "../reducers/site";

// ----------------------------------------------------------------------------
const CREATE_TAG = `
mutation call($code: String!){
    createTag(code: $code){
      createdAt
    }
}
`;
export const create_tag = async (code: string): Promise<ISucceed> => {
  const res: { createTag: ISucceed } = await query(CREATE_TAG, {
    code,
  });
  return res.createTag;
};
const UPDATE_TAG = `
mutation call($id: Int!, $code: String!){
    updateTag(id: $id, code: $code){
      createdAt
    }
}
`;
export const update_tag = async (
  id: number,
  code: string
): Promise<ISucceed> => {
  const res: { updateTag: ISucceed } = await query(UPDATE_TAG, { id, code });
  return res.updateTag;
};
const DESTROY_TAG = `
mutation call($id: Int!){
    destroyTag(id: $id){
      createdAt
    }
}
`;
export const destroy_tag = async (id: number): Promise<ISucceed> => {
  const res: { destroyTag: ISucceed } = await query(DESTROY_TAG, { id });
  return res.destroyTag;
};
const INDEX_TAG = `
query call{
    indexTag{id, code, updatedAt}
}
`;
export interface ITag {
  id: number;
  code: string;
  updatedAt: Date;
}
export const index_tag = async (): Promise<ITag[]> => {
  const res: { indexTag: ITag[] } = await query(INDEX_TAG, {});
  return res.indexTag;
};
// ----------------------------------------------------------------------------
const SET_SITE_SMTP = `
mutation call($host: String!, $port: Int!, $account: String!, $password: String!){
    setSiteSmtp(host: $host, port: $port, account: $account, password: $password){
      createdAt
    }
}
`;
export const set_site_smtp = async (
  host: string,
  port: number,
  account: string,
  password: string
): Promise<ISucceed> => {
  const res: { setSiteSmtp: ISucceed } = await query(SET_SITE_SMTP, {
    host,
    port,
    account,
    password,
  });
  return res.setSiteSmtp;
};
const GET_SITE_SMTP = `
query call{
    getSiteSmtp{host, port account}
}
`;
interface IGetSiteSmtpResponse {
  host: string;
  port: number;
  account: string;
}
export const get_site_smtp = async (): Promise<IGetSiteSmtpResponse> => {
  const res: { getSiteSmtp: IGetSiteSmtpResponse } = await query(
    GET_SITE_SMTP,
    {}
  );
  return res.getSiteSmtp;
};

const SET_SITE_CN_ICP = `
mutation call($code: String!){
    setSiteCnIcp(code: $code){
      createdAt
    }
}
`;
export const set_site_cn_icp = async (code: string): Promise<ISucceed> => {
  const res: { setSiteCnIcp: ISucceed } = await query(SET_SITE_CN_ICP, {
    code,
  });
  return res.setSiteCnIcp;
};
const GET_SITE_CN_ICP = `
query call{
    getSiteCnIcp{
      code
    }
}
`;
export const get_site_cn_icp = async (): Promise<ICnIcp> => {
  const res: { getSiteCnIcp: ICnIcp } = await query(GET_SITE_CN_ICP, {});
  return res.getSiteCnIcp;
};

const SET_SITE_CN_MPS = `
mutation call($code: String!, $name: String!){
    setSiteCnMps(code: $code, name: $name){
      createdAt
    }
}
`;
export const set_site_cn_mps = async (
  code: string,
  name: string
): Promise<ISucceed> => {
  const res: { setSiteCnMps: ISucceed } = await query(SET_SITE_CN_MPS, {
    code,
    name,
  });
  return res.setSiteCnMps;
};

const GET_SITE_GOOGLE_SITE_OWNERSHIP_VERIFYING = `
query call{
    getGoogleSiteOwnershipVerifying{
      code
    }
}
`;
export interface IGoogleSiteOwnershipVerifying {
  code: string;
}
export const get_google_site_ownership_verifying =
  async (): Promise<IGoogleSiteOwnershipVerifying> => {
    const res: {
      getGoogleSiteOwnershipVerifying: IGoogleSiteOwnershipVerifying;
    } = await query(GET_SITE_GOOGLE_SITE_OWNERSHIP_VERIFYING, {});
    return res.getGoogleSiteOwnershipVerifying;
  };

const SET_GOOGLE_SITE_OWNERSHIP_VERIFYING = `
mutation call($code: String!){
    setGoogleSiteOwnershipVerifying(code: $code){
      createdAt
    }
}
`;
export const set_google_site_ownership_verifying = async (
  code: string
): Promise<ISucceed> => {
  const res: { setGoogleSiteOwnershipVerifying: ISucceed } = await query(
    SET_GOOGLE_SITE_OWNERSHIP_VERIFYING,
    {
      code,
    }
  );
  return res.setGoogleSiteOwnershipVerifying;
};

const GET_INDEX_NOW_SITE_OWNERSHIP_VERIFYING = `
query call{
    getIndexNowSiteOwnershipVerifying{
      key
    }
}
`;
export interface IIndexNowSiteOwnershipVerifying {
  key: string;
}
export const get_index_now_site_ownership_verifying =
  async (): Promise<IIndexNowSiteOwnershipVerifying> => {
    const res: {
      getIndexNowSiteOwnershipVerifying: IIndexNowSiteOwnershipVerifying;
    } = await query(GET_INDEX_NOW_SITE_OWNERSHIP_VERIFYING, {});
    return res.getIndexNowSiteOwnershipVerifying;
  };

const SET_INDEX_NOW_OWNERSHIP_VERIFYING = `
mutation call($key: String!){
    setIndexNowSiteOwnershipVerifying(key: $key){
      createdAt
    }
}
`;
export const set_index_now_site_ownership_verifying = async (
  key: string
): Promise<ISucceed> => {
  const res: { setIndexNowSiteOwnershipVerifying: ISucceed } = await query(
    SET_INDEX_NOW_OWNERSHIP_VERIFYING,
    {
      key,
    }
  );
  return res.setIndexNowSiteOwnershipVerifying;
};

const GET_SITE_CN_MPS = `
query call{
    getSiteCnMps{
      code, name
    }
}
`;
export const get_site_cn_mps = async (): Promise<ICnMps> => {
  const res: { getSiteCnMps: ICnMps } = await query(GET_SITE_CN_MPS, {});
  return res.getSiteCnMps;
};

const GET_SITE_AUTHOR = `
query call{
    getSiteAuthor{name, email}
}
`;
export const get_site_author = async (): Promise<IAuthor> => {
  const res: { getSiteAuthor: IAuthor } = await query(GET_SITE_AUTHOR, {});
  return res.getSiteAuthor;
};
const SET_SITE_AUTHOR = `
mutation call($name: String!, $email: String!){
    setSiteAuthor(name: $name, email: $email){
      createdAt
    }
}
`;
export const set_site_author = async (
  name: string,
  email: string
): Promise<ISucceed> => {
  const res: { setSiteAuthor: ISucceed } = await query(SET_SITE_AUTHOR, {
    name,
    email,
  });
  return res.setSiteAuthor;
};

const GET_SITE_KEYWORDS = `
query call{
    getSiteKeywords
}
`;
export const get_site_keywords = async (): Promise<string[]> => {
  const res: { getSiteKeywords: string[] } = await query(GET_SITE_KEYWORDS, {});
  return res.getSiteKeywords;
};
const SET_SITE_KEYWORDS = `
mutation call($items: [String!]!){
    setSiteKeywords(items: $items){
      createdAt
    }
}
`;
export const set_site_keywords = async (items: string[]): Promise<ISucceed> => {
  const res: { setSiteKeywords: ISucceed } = await query(SET_SITE_KEYWORDS, {
    items,
  });
  return res.setSiteKeywords;
};

const GET_SITE_INFO_BY_LANG = `
query call($lang: String!){
    getSiteInfoByLang(lang: $lang) {
        title, subhead, description, copyright
    }
}
`;
export interface IGetSiteInfoByLangResponse {
  title: string;
  subhead: string;
  description: string;
  copyright: string;
  author: { name: string; email: string };
}
export const get_site_info_by_lang = async (
  lang: string
): Promise<IGetSiteInfoByLangResponse> => {
  const res: { getSiteInfoByLang: IGetSiteInfoByLangResponse } = await query(
    GET_SITE_INFO_BY_LANG,
    {
      lang,
    }
  );
  return res.getSiteInfoByLang;
};

const SET_SITE_BASE_INFO = `
mutation call($lang: String!, $form: SetSiteInfoRequest!){
    setSiteBaseInfo(lang: $lang, form: $form) {
        createdAt
    }
}
`;
export const set_site_base_info = async (
  lang: string,
  form: ISetSiteInfoRequest
): Promise<ISucceed> => {
  const res: { setSiteBaseInfo: ISucceed } = await query(SET_SITE_BASE_INFO, {
    lang,
    form,
  });
  return res.setSiteBaseInfo;
};

const GET_SITE_STATUS = `
query call{
    getSiteStatus {
        postgresql{timestamp, version},
        redis{version},
        rabbitmq{username, virtualHost},
        minio{buckets},
        opensearch{plugins}
    }
}
`;
export interface IPostgreSqlStatus {
  timestamp: Date;
  version: string;
}
export interface IRedisStatus {
  version: string[];
}
export interface IRabbitMQStatus {
  username: string;
  virtualHost: string;
}
export interface IMinioStatus {
  buckets: string[];
}
export interface IOpenSearchStatus {
  plugins: string[];
}
export interface IGetSiteStatusResponse {
  postgresql: IPostgreSqlStatus;
  redis: IRedisStatus;
  rabbitmq: IRabbitMQStatus;
  minio: IMinioStatus;
  opensearch: IOpenSearchStatus;
}
export const get_site_status = async (): Promise<IGetSiteStatusResponse> => {
  const res: { getSiteStatus: IGetSiteStatusResponse } = await query(
    GET_SITE_STATUS,
    {}
  );
  return res.getSiteStatus;
};
// ----------------------------------------------------------------------------

interface IRefreshResponse {
  currentUser?: ICurrentUser;
  siteInfo: ISiteInfo;
}

const REFRESH = `
query call{
    refresh{
        siteInfo{
          favicon, title, subhead, keywords, description, copyright, locale, languages,
          author{name, email},
          cnIcp{code}, cnMps{code, name}
        }
        currentUser{
          realName, providerType, lang, timezone, isAdministrator, isRoot, roles
          permissions {operation, resource {type, id}},
          sideBar{
            label, to, external,
            children{label, to, external}
          }
        }
    }
}
`;
export const refresh = async (): Promise<IRefreshResponse> => {
  const res: { refresh: IRefreshResponse } = await query(REFRESH, {});
  return res.refresh;
};
// ----------------------------------------------------------------------------
const DESTROY_LOCALE = `
mutation call($id: Int!){
    destroyLocale(id: $id) {
        createdAt
    }
}
`;
export const destroy_locale = async (id: number): Promise<ISucceed> => {
  const res: { destroyLocale: ISucceed } = await query(DESTROY_LOCALE, {
    id,
  });
  return res.destroyLocale;
};
const SET_LOCALE = `
mutation call($lang: String!, $code: String!, $message: String!){
    setLocale(lang: $lang, code: $code, message: $message) {
        createdAt
    }
}
`;
export const set_locale = async (
  lang: string,
  code: string,
  message: string
): Promise<ISucceed> => {
  const res: { setLocale: ISucceed } = await query(SET_LOCALE, {
    lang,
    code,
    message,
  });
  return res.setLocale;
};
const INDEX_LOCALE = `
query call($pager: Pager!){
    indexLocale(pager: $pager) {
      pagination{total},
      items{
        id, lang, code, message, updatedAt
      }
    }
}
`;
export interface ILocale {
  id: number;
  lang: string;
  code: string;
  message: string;
  updatedAt: Date;
}
interface IIndexLocaleResponse {
  pagination: IPagination;
  items: ILocale[];
}
export const index_locale = async (
  pager: IPager
): Promise<IIndexLocaleResponse> => {
  const res: { indexLocale: IIndexLocaleResponse } = await query(INDEX_LOCALE, {
    pager,
  });
  return res.indexLocale;
};
// ----------------------------------------------------------------------------
const CLOSE_LEAVE_WORD = `
mutation call($id: Int!){
    closeLeaveWord(id: $id) {
        createdAt
    }
}
`;
export const close_leave_word = async (id: number): Promise<ISucceed> => {
  const res: { closeLeaveWord: ISucceed } = await query(CLOSE_LEAVE_WORD, {
    id,
  });
  return res.closeLeaveWord;
};
const DISABLE_LEAVE_WORD = `
mutation call($id: Int!){
    disableLeaveWord(id: $id) {
        createdAt
    }
}
`;
export const disable_leave_word = async (id: number): Promise<ISucceed> => {
  const res: { disableLeaveWord: ISucceed } = await query(DISABLE_LEAVE_WORD, {
    id,
  });
  return res.disableLeaveWord;
};
const ENABLE_LEAVE_WORD = `
mutation call($id: Int!){
    enableLeaveWord(id: $id) {
        createdAt
    }
}
`;
export const enable_leave_word = async (id: number): Promise<ISucceed> => {
  const res: { enableLeaveWord: ISucceed } = await query(ENABLE_LEAVE_WORD, {
    id,
  });
  return res.enableLeaveWord;
};
const CREATE_LEAVE_WORD = `
mutation call($body: String!, $editor: Editor!){
    createLeaveWord(body: $body, editor: $editor) {
        createdAt
    }
}
`;
export const create_leave_word = async (body: string): Promise<ISucceed> => {
  const res: { createLeaveWord: ISucceed } = await query(CREATE_LEAVE_WORD, {
    body,
    editor: EDITOR_TEXTAREA,
  });
  return res.createLeaveWord;
};

const INDEX_LEAVE_WORD = `
query call($pager: Pager!){
    indexLeaveWord(pager: $pager) {
      pagination{total},
      items{
        id, lang, ip, body, bodyEditor, status, deletedAt, updatedAt
      }
    }
}
`;
export interface ILeaveWord {
  id: number;
  lang: string;
  ip: string;
  body: string;
  bodyEditor: string;
  status: string;
  deletedAt?: Date;
  updatedAt: Date;
}
interface IIndexLeaveWordResponse {
  pagination: IPagination;
  items: ILeaveWord[];
}
export const index_leave_word = async (
  pager: IPager
): Promise<IIndexLeaveWordResponse> => {
  const res: { indexLeaveWord: IIndexLeaveWordResponse } = await query(
    INDEX_LEAVE_WORD,
    {
      pager,
    }
  );
  return res.indexLeaveWord;
};

// ----------------------------------------------------------------------------
const POLICY_ROLES = `
query call{
    policyRoles
}
`;
export const policy_roles = async (): Promise<string[]> => {
  const res: { policyRoles: string[] } = await query(POLICY_ROLES, {});
  return res.policyRoles;
};

const POLICY_USERS = `
query call{
    policyUsers {
        id, providerType, providerId, label, lang, timezone
    }
}
`;
export interface IUserSelectOption {
  id: number;
  label: string;
  providerType: string;
  providerId: number;
  lang: string;
  timezone: string;
}
export const policy_users = async (): Promise<IUserSelectOption[]> => {
  const res: { policyUsers: IUserSelectOption[] } = await query(
    POLICY_USERS,
    {}
  );
  return res.policyUsers;
};

const POLICY_USER_ROLE_RELATIONS = `
query call{
    policyUserRoleRelations {
        users{id, providerType, providerId, label, lang, timezone},
        role
    }
}
`;
export interface IUserRoleRelation {
  users: IUserSelectOption[];
  role: string;
}
export const policy_user_role_relations = async (): Promise<
  IUserRoleRelation[]
> => {
  const res: { policyUserRoleRelations: IUserRoleRelation[] } = await query(
    POLICY_USER_ROLE_RELATIONS,
    {}
  );
  return res.policyUserRoleRelations;
};

const ADMINISTRATORS = `
query call{
    administrators {
        id, providerType, providerId, label, lang, timezone
    }
}
`;
export const administrators = async (): Promise<IUserSelectOption[]> => {
  const res: { administrators: IUserSelectOption[] } = await query(
    ADMINISTRATORS,
    {}
  );
  return res.administrators;
};
const ENABLE_ADMINISTRATOR = `
mutation call($user: Int!){
    enableAdministrator(user: $user) {
        createdAt
    }
}
`;
export const enable_administrator = async (user: number): Promise<ISucceed> => {
  const res: { enableAdministrator: ISucceed } = await query(
    ENABLE_ADMINISTRATOR,
    {
      user,
    }
  );
  return res.enableAdministrator;
};
const DISABLE_ADMINISTRATOR = `
mutation call($user: Int!){
    disableAdministrator(user: $user) {
        createdAt
    }
}
`;
export const disable_administrator = async (
  user: number
): Promise<ISucceed> => {
  const res: { disableAdministrator: ISucceed } = await query(
    DISABLE_ADMINISTRATOR,
    {
      user,
    }
  );
  return res.disableAdministrator;
};

const USERS_FOR_ROLE = `
query call($code: String!){
    usersForRole(code: $code) {
        id, providerType, providerId, label, lang, timezone
    }
}
`;
export const users_for_role = async (
  code: string
): Promise<IUserSelectOption[]> => {
  const res: { usersForRole: IUserSelectOption[] } = await query(
    USERS_FOR_ROLE,
    { code }
  );
  return res.usersForRole;
};
const ADD_ROLE_TO_USER = `
mutation call($user: Int!, $role: String!){
    addRoleToUser(user: $user, role: $role) {
        createdAt
    }
}
`;
export const add_role_to_user = async (
  user: number,
  role: string
): Promise<ISucceed> => {
  const res: { addRoleToUser: ISucceed } = await query(ADD_ROLE_TO_USER, {
    user,
    role,
  });
  return res.addRoleToUser;
};
const REMOVE_ROLE_FROM_USER = `
mutation call($user: Int!, $role: String!){
    removeRoleFromUser(user: $user, role: $role) {
        createdAt
    }
}
`;
export const remove_role_from_user = async (
  user: number,
  role: string
): Promise<ISucceed> => {
  const res: { removeRoleFromUser: ISucceed } = await query(
    REMOVE_ROLE_FROM_USER,
    {
      user,
      role,
    }
  );
  return res.removeRoleFromUser;
};
// ----------------------------------------------------------------------------

const ENABLE_SESSION = `
mutation call($id: Int!){
    enableSession(id: $id) {
        createdAt
    }
}
`;
export const enable_session = async (id: number): Promise<ISucceed> => {
  const res: { enableSession: ISucceed } = await query(ENABLE_SESSION, {
    id,
  });
  return res.enableSession;
};
const DISABLE_SESSION = `
mutation call($id: Int!){
    disableSession(id: $id) {
        createdAt
    }
}
`;
export const disable_session = async (id: number): Promise<ISucceed> => {
  const res: { disableSession: ISucceed } = await query(DISABLE_SESSION, {
    id,
  });
  return res.disableSession;
};
const INDEX_SESSION = `
query call($pager: Pager!){
    indexSession(pager: $pager) {
      pagination{total},
      items{
        id, realName, uid, providerType, providerId, ip, expiresAt, deletedAt, createdAt,
        detail{id, lang, timezone, signInCount, lastSignInAt, lastSignInIp, currentSignInAt, currentSignInIp, lockedAt, deletedAt, updatedAt}
      }
    }
}
`;

export interface ISession {
  id: number;
  detail: IUser;
  realName: string;
  uid: string;
  providerType: string;
  providerId: number;
  ip: string;
  expiresAt: Date;
  deletedAt?: Date;
  createdAt: Date;
}
interface IIndexSessionResponse {
  pagination: IPagination;
  items: ISession[];
}
export const index_session = async (
  pager: IPager
): Promise<IIndexSessionResponse> => {
  const res: { indexSession: IIndexSessionResponse } = await query(
    INDEX_SESSION,
    {
      pager,
    }
  );
  return res.indexSession;
};
// ----------------------------------------------------------------------------

const UNLOCK_USER = `
mutation call($id: Int!){
    unlockUser(id: $id) {
        createdAt
    }
}
`;
export const unlock_user = async (id: number): Promise<ISucceed> => {
  const res: { unlockUser: ISucceed } = await query(UNLOCK_USER, {
    id,
  });
  return res.unlockUser;
};
const LOCK_USER = `
mutation call($id: Int!){
    lockUser(id: $id) {
        createdAt
    }
}
`;
export const lock_user = async (id: number): Promise<ISucceed> => {
  const res: { lockUser: ISucceed } = await query(LOCK_USER, {
    id,
  });
  return res.lockUser;
};

const ENABLE_USER = `
mutation call($id: Int!){
    enableUser(id: $id) {
        createdAt
    }
}
`;
export const enable_user = async (id: number): Promise<ISucceed> => {
  const res: { enableUser: ISucceed } = await query(ENABLE_USER, {
    id,
  });
  return res.enableUser;
};
const DISABLE_USER = `
mutation call($id: Int!){
    disableUser(id: $id) {
        createdAt
    }
}
`;
export const disable_user = async (id: number): Promise<ISucceed> => {
  const res: { disableUser: ISucceed } = await query(DISABLE_USER, {
    id,
  });
  return res.disableUser;
};

const ENABLE_EMAIL_USER = `
mutation call($id: Int!){
    enableEmailUser(id: $id) {
        createdAt
    }
}
`;
export const enable_email_user = async (id: number): Promise<ISucceed> => {
  const res: { enableEmailUser: ISucceed } = await query(ENABLE_EMAIL_USER, {
    id,
  });
  return res.enableEmailUser;
};
const DISABLE_EMAIL_USER = `
mutation call($id: Int!){
    disableEmailUser(id: $id) {
        createdAt
    }
}
`;
export const disable_email_user = async (id: number): Promise<ISucceed> => {
  const res: { disableEmailUser: ISucceed } = await query(DISABLE_EMAIL_USER, {
    id,
  });
  return res.disableEmailUser;
};
const CONFIRM_EMAIL_USER = `
mutation call($id: Int!){
    confirmEmailUser(id: $id) {
        createdAt
    }
}
`;
export const confirm_email_user = async (id: number): Promise<ISucceed> => {
  const res: { confirmEmailUser: ISucceed } = await query(CONFIRM_EMAIL_USER, {
    id,
  });
  return res.confirmEmailUser;
};

const INDEX_EMAIL_USER = `
query call($pager: Pager!){
    indexEmailUser(pager: $pager) {
      pagination{total},
      items{
        id, realName, nickname, email, avatar, confirmedAt, deletedAt, updatedAt,
        detail{id, lang, timezone, signInCount, lastSignInAt, lastSignInIp, currentSignInAt, currentSignInIp, lockedAt, deletedAt, updatedAt}
      }
    }
}
`;

export interface IUser {
  id: number;
  lang: string;
  timezone: string;
  signInCount: number;
  lastSignInAt?: Date;
  lastSignInIp?: string;
  currentSignInAt?: Date;
  currentSignInIp?: string;
  lockedAt?: Date;
  deletedAt?: Date;
  updatedAt: Date;
}
export interface IEmailUser {
  id: number;
  realName: string;
  nickname: string;
  email: string;
  avatar: string;
  detail: IUser;
  resource: IResource;
  confirmedAt?: Date;
  deletedAt?: Date;
  updatedAt: Date;
}
interface IIndexEmailUserResponse {
  pagination: IPagination;
  items: IEmailUser[];
}
export const index_email_user = async (
  pager: IPager
): Promise<IIndexEmailUserResponse> => {
  const res: { indexEmailUser: IIndexEmailUserResponse } = await query(
    INDEX_EMAIL_USER,
    {
      pager,
    }
  );
  return res.indexEmailUser;
};

const INDEX_LOG = `
query call($pager: Pager!){
    indexLog(pager: $pager) {
      pagination{total},
      items{
        id, plugin, message, level, ip, createdAt,
        resource{type, id}
      }
    }
}
`;
export interface ILog {
  id: number;
  plugin: string;
  message: string;
  level: string;
  ip: string;
  resource: IResource;
  createdAt: Date;
}
interface IIndexLogResponse {
  pagination: IPagination;
  items: ILog[];
}
export const index_log = async (pager: IPager): Promise<IIndexLogResponse> => {
  const res: { indexLog: IIndexLogResponse } = await query(INDEX_LOG, {
    pager,
  });
  return res.indexLog;
};

const GET_EMAIL_USER_PROFILE = `
query call{
    getEmailUserProfile {
        realName, nickname, email, avatar, lang, timezone
    }
}
`;
interface IEmailUserProfile {
  realName: string;
  nickname: string;
  email: string;
  avatar: string;
  lang: string;
  timezone: string;
}
export const get_email_user_profile = async (): Promise<IEmailUserProfile> => {
  const res: { getEmailUserProfile: IEmailUserProfile } = await query(
    GET_EMAIL_USER_PROFILE,
    {}
  );
  return res.getEmailUserProfile;
};
const SET_EMAIL_USER_PROFILE = `
mutation call($realName: String!, $lang: String!, $timezone: String!){
    setEmailUserProfile(realName: $realName, lang: $lang, timezone: $timezone) {
        createdAt
    }
}
`;
export const set_email_user_profile = async (
  realName: string,
  lang: string,
  timezone: string
): Promise<ISucceed> => {
  const res: { setEmailUserProfile: ISucceed } = await query(
    SET_EMAIL_USER_PROFILE,
    { realName, lang, timezone }
  );
  return res.setEmailUserProfile;
};
const CANCEL_MY_EMAIL_ACCOUNT = `
mutation call($password: String!, $reason: String!){
    cancelMyEmailAccount(password: $password, reason: $reason) {
        createdAt
    }
}
`;
export const cancel_my_email_account = async (
  password: string,
  reason: string
): Promise<ISucceed> => {
  const res: { cancelMyEmailAccount: ISucceed } = await query(
    CANCEL_MY_EMAIL_ACCOUNT,
    { password: mingle_password(password), reason }
  );
  return res.cancelMyEmailAccount;
};
const CHANGE_EMAIL_USER_PASSWORD = `
mutation call($currentPassword: String!, $newPassword: String!){
    changeEmailUserPassword(currentPassword: $currentPassword, newPassword: $newPassword) {
        createdAt
    }
}
`;
export const change_email_user_password = async (
  currentPassword: string,
  newPassword: string
): Promise<ISucceed> => {
  const res: { changeEmailUserPassword: ISucceed } = await query(
    CHANGE_EMAIL_USER_PASSWORD,
    {
      currentPassword: mingle_password(currentPassword),
      newPassword: mingle_password(newPassword),
    }
  );
  return res.changeEmailUserPassword;
};

const USER_SIGN_OUT = `
mutation call{
    userSignOut {
        createdAt
    }
}
`;
export const user_sign_out = async (): Promise<ISucceed> => {
  const res: { userSignOut: ISucceed } = await query(USER_SIGN_OUT, {});
  return res.userSignOut;
};

const RESET_EMAIL_USER_PASSWORD_BY_TOKEN = `
mutation call($token: String!, $password: String!){
    resetEmailUserPasswordByToken(token: $token, password: $password) {
        createdAt
    }
}
`;
export const reset_email_user_password_by_token = async (
  token: string,
  password: string
): Promise<ISucceed> => {
  const res: { resetEmailUserPasswordByToken: ISucceed } = await query(
    RESET_EMAIL_USER_PASSWORD_BY_TOKEN,
    {
      token,
      password: mingle_password(password),
    }
  );
  return res.resetEmailUserPasswordByToken;
};
const SEND_FORGOT_PASSWORD_EMAIL_FOR_USER = `
query call($user: String!){
    sendForgotPasswordEmailForUser(user: $user) {
        createdAt
    }
}
`;
export const send_forgot_password_email_for_user = async (
  user: string
): Promise<ISucceed> => {
  const res: { sendForgotPasswordEmailForUser: ISucceed } = await query(
    SEND_FORGOT_PASSWORD_EMAIL_FOR_USER,
    {
      user,
    }
  );
  return res.sendForgotPasswordEmailForUser;
};
const UNLOCK_EMAIL_USER_BY_TOKEN = `
mutation call($token: String!){
    unlockEmailUserByToken(token: $token) {
        createdAt
    }
}
`;
export const unlock_email_user_by_token = async (
  token: string
): Promise<ISucceed> => {
  const res: { unlockEmailUserByToken: ISucceed } = await query(
    UNLOCK_EMAIL_USER_BY_TOKEN,
    {
      token,
    }
  );
  return res.unlockEmailUserByToken;
};
const SEND_UNLOCK_EMAIL_FOR_USER = `
query call($user: String!){
    sendUnlockEmailForUser(user: $user) {
        createdAt
    }
}
`;
export const send_unlock_email_for_user = async (
  user: string
): Promise<ISucceed> => {
  const res: { sendUnlockEmailForUser: ISucceed } = await query(
    SEND_UNLOCK_EMAIL_FOR_USER,
    {
      user,
    }
  );
  return res.sendUnlockEmailForUser;
};
const CONFIRM_EMAIL_USER_BY_TOKEN = `
mutation call($token: String!){
    confirmEmailUserByToken(token: $token) {
        createdAt
    }
}
`;
export const confirm_email_user_by_token = async (
  token: string
): Promise<ISucceed> => {
  const res: { confirmEmailUserByToken: ISucceed } = await query(
    CONFIRM_EMAIL_USER_BY_TOKEN,
    {
      token,
    }
  );
  return res.confirmEmailUserByToken;
};
const SEND_CONFIRM_EMAIL_FOR_USER = `
query call($user: String!){
    sendConfirmEmailForUser(user: $user) {
        createdAt
    }
}
`;
export const send_confirm_email_for_user = async (
  user: string
): Promise<ISucceed> => {
  const res: { sendConfirmEmailForUser: ISucceed } = await query(
    SEND_CONFIRM_EMAIL_FOR_USER,
    {
      user,
    }
  );
  return res.sendConfirmEmailForUser;
};

const USER_SIGN_IN_BY_EMAIL = `
mutation call($user: String!, $password: String!){
    userSignInByEmail(user: $user, password: $password) {
        token,
        profile{
          realName, providerType, lang, timezone, isAdministrator, isRoot, roles
          permissions {operation, resource {type, id}},
          sideBar{
            label, to, external,
            children{label, to, external}
          }
        }
    }
}
`;

interface IUserSignInResponse {
  token: string;
  profile: ICurrentUser;
}
export const user_sign_in_by_email = async (
  user: string,
  password: string
): Promise<IUserSignInResponse> => {
  const res: { userSignInByEmail: IUserSignInResponse } = await query(
    USER_SIGN_IN_BY_EMAIL,
    {
      user,
      password: mingle_password(password),
    }
  );
  return res.userSignInByEmail;
};

export interface IUserSignUpByEmailRequest {
  email: string;
  realName: string;
  nickname: string;
  password: string;
  timezone: string;
}

const USER_SIGN_UP_BY_EMAIL = `
mutation call($lang: String!, $form: UserSignUpByEmailRequest!){
    userSignUpByEmail(lang: $lang, form: $form) {
        createdAt
    }
}
`;

export const user_sign_up_by_email = async (
  form: IUserSignUpByEmailRequest
): Promise<ISucceed> => {
  form.password = mingle_password(form.password);
  const res: { userSignUpByEmail: ISucceed } = await query(
    USER_SIGN_UP_BY_EMAIL,
    {
      lang: detect_locale(),
      form,
    }
  );
  return res.userSignUpByEmail;
};

// ----------------------------------------------------------------------------
const INSTALL = `
mutation call($lang: String!, $site: SetSiteInfoRequest!, $user: UserSignUpByEmailRequest!){
    install(lang: $lang, site: $site, user: $user) {
        createdAt
    }
}
`;

export interface ISetSiteInfoRequest {
  title: string;
  subhead: string;
  description: string;
  copyright: string;
}
export const install = async (
  site: ISetSiteInfoRequest,
  user: IUserSignUpByEmailRequest
): Promise<ISucceed> => {
  user.password = mingle_password(user.password);
  const res: { install: ISucceed } = await query(INSTALL, {
    lang: detect_locale(),
    site,
    user,
  });
  return res.install;
};

const INDEX_LOCALE_BY_LANG = `
query call($lang: String!){
    indexLocaleByLang(lang: $lang) {
        code, message
    }
}
`;

export const index_locale_by_lang = async (
  lang: string
): Promise<{ code: string; message: string }[]> => {
  const res: {
    indexLocaleByLang: { code: string; message: string }[];
  } = await query(INDEX_LOCALE_BY_LANG, { lang });
  return res.indexLocaleByLang;
};
