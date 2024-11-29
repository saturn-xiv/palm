import { get as get_token } from "../reducers/current-user";

export const EDITOR_TEXTAREA = "TEXTAREA";

export const query = async <V, R>(query: string, args: V): Promise<R> => {
  return graphql<{ query: string; variables: V }, R>({
    query,
    variables: args,
  });
};

export const mutation = async <V, R>(mutation: string, args: V): Promise<R> => {
  return graphql<{ mutation: string; variables: V }, R>({
    mutation,
    variables: args,
  });
};

// https://github.github.io/fetch/#options
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
  const res: { data: R } | { errors: IError[] } = await response.json();
  if ("errors" in res) {
    return Promise.reject(res.errors);
  }
  return res.data;
};

export interface ISucceed {
  createdAt: Date;
}

export interface IError {
  message: string;
  locations: { line: number; column: number }[];
}

export interface IPagination {
  total: number;
  page: number;
  size: number;
  hasNext: boolean;
  hasPrevious: boolean;
}

export interface IPager {
  page: number;
  size: number;
}
