import { options } from "./";

const GRAPHQL_PATH = "/graphql";

export interface IErrorMessage {
  message: string;
  locations: { line: number; column: number }[];
}

interface IErrorResponse {
  errors: IErrorMessage[];
}
interface ISucceedResponse<R> {
  data: R;
}

export type IResponse<R> = ISucceedResponse<R> | IErrorResponse;

export const query = async <R>(
  query: string,
  variables: object
): Promise<R> => {
  const data = options("POST");
  data.body = JSON.stringify({ query, variables });
  const response = await fetch(GRAPHQL_PATH, data);
  if (!response.ok) {
    const reason = await response.text();
    const items: IErrorMessage[] = [{ message: reason, locations: [] }];
    return Promise.reject(items);
  }
  const res: IResponse<R> = await response.json();
  if ("errors" in res) {
    return Promise.reject(res.errors);
  }

  return res.data;
};

// ----------------------------------------------------------------------------

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
