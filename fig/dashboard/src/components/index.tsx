import { AlertColor } from "@mui/material/Alert";

export const PASSWORD_PLACEHOLDER = "••••••";
export const PASSWORD_MIN_LENGTH = 6;
export const PASSWORD_MAX_LENGTH = 31;
export const EMAIL_MIN_LENGTH = 5;
export const EMAIL_MAX_LENGTH = 63;
export const NAME_MIN_LENGTH = 2;
export const NAME_MAX_LENGTH = 31;

export interface IAlert {
  color: AlertColor;
  messages: string[];
}
