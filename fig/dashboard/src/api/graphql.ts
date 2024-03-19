import { options } from "./";

const GRAPHQL_PATH = "/graphql";

interface IError {
  errors: {
    message: string;
    locations: { line: number; column: number }[];
  }[];
}
interface ISucceed<R> {
  data: R;
}

export type IResponse<R> = ISucceed<R> | IError;

export const query = async <R>(query: string): Promise<IResponse<R>> => {
  const data = options("POST");
  data.body = JSON.stringify({ query });
  const response = await fetch(GRAPHQL_PATH, data);
  const res: IResponse<R> = await response.json();
  return res;
};
