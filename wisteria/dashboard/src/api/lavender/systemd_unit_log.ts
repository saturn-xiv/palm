import graphql from "../../graphql";

export interface IItem {
  host: string;
  unit: string;
  message: string;
  createdAt: Date;
}

export const by_unit = async (unit: string): Promise<IItem[]> => {
  const res: { lavenderSystemdLogByUnit: IItem[] } = await graphql(
    `
      query call($unit: String!) {
        lavenderSystemdLogByUnit(unit: $unit) {
          host
          unit
          message
          createdAt
        }
      }
    `,
    { unit },
  );
  return res.lavenderSystemdLogByUnit;
};
