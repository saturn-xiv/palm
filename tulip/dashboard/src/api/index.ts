import type { Metadata } from "grpc-web";

export const BACKEND = import.meta.env.VITE_BACKEND;

export const metadata = (): Metadata => {
  return {};
};
