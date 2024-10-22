import { get as get_token } from "./reducers/current-user";

// https://github.github.io/fetch/#options
export const query = async <V, R>(query: string, args: V): Promise<R> => {
  return graphql<{ query: string; variables: V }, R>({
    query,
    variables: args,
  });
};

const graphql = async <Q, R>(body: Q): Promise<R> => {
  const response = await fetch("/graphql", {
    credentials: "include",
    mode: "cors",
    headers: {
      Authorization: `Bearer ${get_token()}`,
      "Content-Type": "application/json; charset=utf-8",
    },
    method: "POST",
    body: JSON.stringify(body),
  });
  const res: { data: R } = await response.json();
  return res.data;
};

