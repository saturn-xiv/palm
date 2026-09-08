import { post as http_post } from "./request";

const GRAPHQL: string = import.meta.env.VITE_GRAPHQL_PATH || "/graphql";

interface Response<T> {
  data?: T;
  errors?: Array<{ message: string }>;
}

const handle = async <V, T>(query: string, variables: V): Promise<T> => {
  const res: Response<T> = await http_post(GRAPHQL, { query, variables });
  if (res.data) {
    return res.data;
  }
  if (res.errors) {
    throw new Error(res.errors.join("\n"));
  }
  throw new Error("Empty response.");
};

export default handle;
