import { post as http_post } from "./request";

const GRAPHQL: string = import.meta.env.VITE_GRAPHQL_PATH || "/graphql";

export const query = () => {};
export const mutation = () => {};

export interface Response<T> {
  data?: T;
  errors?: Array<{ message: string }>;
}

const handle = async <T, V>(query: string, variables: V): Promise<Response<T>> => {
  const res: Response<T> = await http_post(GRAPHQL, { query, variables });
  return res;
};

export default handle;
