import { options } from "./";

const GRAPHQL_PATH = "/graphql";

export interface IErrorMessage {
  message: string;
  locations: { line: number; column: number }[];
}

interface IError {
  errors: IErrorMessage[];
}
interface ISucceed<R> {
  data: R;
}

export type IResponse<R> = ISucceed<R> | IError;

export const query = async <R>(
  query: string,
  variables: object
): Promise<R> => {
  const data = options("POST");
  data.body = JSON.stringify({ query, variables });
  const response = await fetch(GRAPHQL_PATH, data);
  const res: IResponse<R> = await response.json();
  if ("errors" in res) {
    return Promise.reject(res.errors);
  }

  return res.data;
};
