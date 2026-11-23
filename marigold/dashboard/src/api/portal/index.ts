import { z } from "zod";

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

export const EMAIL_MIN_LEN = 5;
export const EMAIL_MAX_LEN = 31;
export const EMAIL_PLACEHOLDER = "your@email.com";
export const NAME_MIN_LEN = 2;
export const NAME_MAX_LEN = 31;
export const PASSWORD_MIN_LEN = 6;
export const PASSWORD_MAX_LEN = 31;
export const PASSWORD_PLACEHOLDER = "••••••";
export const PHONE_NUMBER_MAX = 15;
export const PHONE_NUMBER_MIN = 7;

export const SignUpBySmsFormSchema = z
  .object({
    phoneNumber: z
      .string()
      .trim()
      .min(PHONE_NUMBER_MIN)
      .max(PHONE_NUMBER_MAX)
      .toLowerCase(),
    name: z.string().trim().min(NAME_MIN_LEN).max(NAME_MAX_LEN),
    password: z.string().min(PASSWORD_MIN_LEN).max(PASSWORD_MAX_LEN),
    passwordConfirmation: z.string(),
  })
  .refine((data) => data.password === data.passwordConfirmation, {
    path: ["passwordConfirmation"],
  });

export type SignUpBySmsFormData = z.infer<typeof SignUpBySmsFormSchema>;

export const SignUpByEmailFormSchema = z
  .object({
    email: z.email().trim().min(EMAIL_MIN_LEN).max(EMAIL_MAX_LEN).toLowerCase(),
    name: z.string().trim().min(NAME_MIN_LEN).max(NAME_MAX_LEN),
    password: z.string().min(PASSWORD_MIN_LEN).max(PASSWORD_MAX_LEN),
    passwordConfirmation: z.string(),
  })
  .refine((data) => data.password === data.passwordConfirmation, {
    path: ["passwordConfirmation"],
  });

export type SignUpByEmailFormData = z.infer<typeof SignUpByEmailFormSchema>;
