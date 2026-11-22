import graphql from "../../graphql";

export interface IItem {
  code: string;
  icon?: string;
  children?: IItem[];
}

export const dashboard = async (): Promise<IItem[]> => {
  const res: { dashboardMenus: IItem[] } = await graphql(
    `
      query call {
        dashboardMenus {
          code
          icon
          children {
            code
            icon
            children {
              code
              icon
            }
          }
        }
      }
    `,
    {},
  );
  return res.dashboardMenus;
};
