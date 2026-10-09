import graphql from "../../graphql";

export interface IItem {
  from: string;
  url: string;
  status_code?: number;
  content_type?: number;
  elapsed: number;
  body: string;
  createdAt: Date;
}

export const by_unit = async (unit: string): Promise<IItem[]> => {
  const res: { lavenderSystemdLogByUnit: IItem[] } = await graphql(
    `
      query call($unit: String!) {
        lavenderSystemdLogByUnit(unit: $unit) {
          from
          url
          status_code
          content_type
          elapsed
          body
          createdAt
        }
      }
    `,
    { unit },
  );
  return res.lavenderSystemdLogByUnit;
};
