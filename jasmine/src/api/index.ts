import { get as getToken } from "../reducers/current-user";

export const options = (method: string): RequestInit => {
  return {
    credentials: "include",
    headers: {
      Authorization: `Bearer ${getToken()}`,
      Accept: "application/json",
      "Content-Type": "application/json; charset=utf-8",
    },
    mode: "cors",
    method,
  };
};

export const get = async <R>(path: string): Promise<R> => {
  const response = await fetch(path, options("GET"));
  if (response.ok) {
    const res: R = await response.json();
    return res;
  }
  const reason = await response.text();
  throw reason;
};

export const delete_ = async <R>(path: string): Promise<R> => {
  const response = await fetch(path, options("DELETE"));
  if (response.ok) {
    const res: R = await response.json();
    return res;
  }
  const reason = await response.text();
  throw reason;
};

export const upload = async <R>(path: string, files: File[]): Promise<R> => {
  const form = new FormData();
  for (const file of files) {
    form.append("file", file);
  }

  const data: RequestInit = {
    credentials: "include",
    headers: {
      Authorization: `Bearer ${getToken()}`,
    },
    mode: "cors",
    method: "POST",
  };
  data.body = form;
  const response = await fetch(path, data);
  if (response.ok) {
    const res: R = await response.json();
    return res;
  }
  const reason = await response.text();
  throw reason;
};

// https://github.github.io/fetch/#options
export const post = async <Q, R>(path: string, body: Q): Promise<R> => {
  const data = options("POST");
  data.body = JSON.stringify(body);
  const response = await fetch(path, data);
  if (response.ok) {
    const res: R = await response.json();
    return res;
  }
  const reason = await response.text();
  throw reason;
};

export const patch = async <Q, R>(path: string, body: Q): Promise<R> => {
  const data = options("PATCH");
  data.body = JSON.stringify(body);
  const response = await fetch(path, data);
  if (response.ok) {
    const res: R = await response.json();
    return res;
  }
  const reason = await response.text();
  throw reason;
};

export const put = async <Q, R>(path: string, body: Q): Promise<R> => {
  const data = options("PUT");
  data.body = JSON.stringify(body);
  const response = await fetch(path, data);
  if (response.ok) {
    const res: R = await response.json();
    return res;
  }
  const reason = await response.text();
  throw reason;
};

export const download = (path: string, name: string) => {
  const data = options("GET");
  fetch(path, data)
    .then((response) => response.blob())
    .then((blob) => {
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = name;
      document.body.appendChild(a); // for firefox
      a.click();
      a.remove();
    });
};
