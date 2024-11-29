export const PASSWORD_PLACEHOLDER = "••••••";
export const PASSWORD_MIN_LENGTH = 6;
export const PASSWORD_MAX_LENGTH = 31;
export const EMAIL_MIN_LENGTH = 5;
export const EMAIL_MAX_LENGTH = 63;
export const NAME_MIN_LENGTH = 2;
export const NAME_MAX_LENGTH = 31;

export const MEMO_MIN_LENGTH = 6;
export const MEMO_MAX_LENGTH = 1023;
export const TITLE_MIN_LENGTH = 1;
export const TITLE_MAX_LENGTH = 127;
export const SLUG_MIN_LENGTH = 1;
export const SLUG_MAX_LENGTH = 127;

export const DEFAULT_PAGE_SIZE = 60;

export interface IAlert {
  color: string;
  messages: string[];
}
