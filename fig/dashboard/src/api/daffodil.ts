import { get as detect_locale } from "../i18n";
import { query, ISucceed, EDITOR_TEXTAREA, IPagination, IPager } from ".";
import {
  ICurrentUser,
  IResource,
  mingle_password,
} from "../reducers/current-user";
import { ISiteInfo } from "../reducers/site";

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
          cnBi{code}, cnIcp{code}, cnGab{code}
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
