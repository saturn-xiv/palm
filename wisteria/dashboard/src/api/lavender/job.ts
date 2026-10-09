import graphql from "../../graphql";

import { type ISucceeded } from "../portal";

export const vacuum = async (months: number): Promise<ISucceeded> => {
  const res: { lavenderVacuum: ISucceeded } = await graphql(
    `
      mutation call($months: Int!) {
        lavenderVacuum(months: $months) {
          createdAt
        }
      }
    `,
    { months },
  );
  return res.lavenderVacuum;
};

export const launch = async (
  name: string,
  args: string[],
): Promise<ISucceeded> => {
  const res: { lavenderLaunchJob: ISucceeded } = await graphql(
    `
      mutation call($name: String!, $args: [String!]!) {
        lavenderLaunchJob(name: $name, args: $args) {
          createdAt
        }
      }
    `,
    { name, args },
  );
  return res.lavenderLaunchJob;
};

export interface IItem {
  id: number;
  description: string;
  version: string;
  args: string[];
}

export const index = async (): Promise<IItem[]> => {
  const res: { lavenderIndexJob: IItem[] } = await graphql(
    `
      query call {
        lavenderIndexJob {
          id
          description
          version
          args
        }
      }
    `,
    {},
  );
  return res.lavenderIndexJob;
};

export const show = async (id: number): Promise<IItem> => {
  const res: { lavenderShowJob: IItem } = await graphql(
    `
      query call($id: Int!) {
        lavenderShowJob(id: $id) {
          id
          description
          version
          args
        }
      }
    `,
    { id },
  );
  return res.lavenderShowJob;
};
