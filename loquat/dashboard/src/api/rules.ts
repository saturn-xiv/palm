import type { IOk } from ".";
import { graphql, type IGraphqlResponse } from "../request";

interface IDisableResponse {
  disableFirewallRule: IOk;
}
export const disable = async (
  id: string
): Promise<IGraphqlResponse<IDisableResponse>> => {
  const res: IGraphqlResponse<IDisableResponse> = await graphql(
    `
      mutation call($id: ID!) {
        disableFirewallRule(id: $id) {
          createdAt
        }
      }
    `,
    { id }
  );
  return res;
};
interface IEnableResponse {
  enableFirewallRule: IOk;
}
export const enable = async (
  id: string
): Promise<IGraphqlResponse<IEnableResponse>> => {
  const res: IGraphqlResponse<IEnableResponse> = await graphql(
    `
      mutation call($id: ID!) {
        enableFirewallRule(id: $id) {
          createdAt
        }
      }
    `,
    { id }
  );
  return res;
};

export interface IInputRule {
  __typename: string;
  id: string;
  device: string;
  tcp: boolean;
  port: number;
  sortOrder: number;
  memo: string;
  updatedAt: Date;
  deletedAt?: Date;
}
export interface INatRule {
  __typename: string;
  id: string;
  device: string;
  port: number;
  tcp: boolean;
  destinationIp: string;
  destinationPort: number;
  sortOrder: number;
  memo: string;
  updatedAt: Date;
  deletedAt?: Date;
}
export interface IPingRule {
  __typename: string;
  id: string;
  device: string;
  sortOrder: number;
  memo: string;
  updatedAt: Date;
  deletedAt?: Date;
}

export type IRule = IInputRule | IPingRule | INatRule;

export interface IIndexResponse {
  indexFirewallRule: IRule[];
}

export const index = async (): Promise<IGraphqlResponse<IIndexResponse>> => {
  const res: IGraphqlResponse<IIndexResponse> = await graphql(
    `
      query call {
        indexFirewallRule {
          __typename
          ... on Ping {
            id
            device
            sortOrder
            memo
            updatedAt
            deletedAt
          }
          ... on Input {
            id
            device
            tcp
            port
            sortOrder
            memo
            updatedAt
            deletedAt
          }
          ... on Nat {
            id
            device
            tcp
            port
            destinationIp
            destinationPort
            sortOrder
            memo
            updatedAt
            deletedAt
          }
        }
      }
    `,
    {}
  );
  return res;
};
