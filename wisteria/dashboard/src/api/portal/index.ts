export interface IPage {
  index: number;
  size: number;
  total: number;
  hasNext: boolean;
  hasPrevious: boolean;
}
export interface IPagination {
  index: number;
  size: number;
}

export interface ISucceeded {
  createdAt: Date;
}

export const USERS_SIGN_IN_PATH = "/anonymous/users/sign-in";
export const PERSONAL_LOGS_PATH = "/dashboard/personal/logs";
