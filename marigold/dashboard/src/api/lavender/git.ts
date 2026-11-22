import graphql from "../../graphql";

export interface ICommit {
  id: string;
  message: string;
  createdAt: Date;
}

export const index_commit = async (): Promise<ICommit[]> => {
  const res: { lavenderIndexGitCommit: ICommit[] } = await graphql(
    `
      query call {
        lavenderIndexGitCommit {
          id
          description
          version
          args
        }
      }
    `,
    {},
  );
  return res.lavenderIndexGitCommit;
};
