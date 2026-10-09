import graphql from "../../graphql";

export interface IItem {
  node: string;
  name: string;
  container: string;
  owners: IOwner[];
  message: string;
  createdAt: Date;
}
export interface IOwner {
  kind: string;
  name: string;
  uid: string;
}

export const by_namespace = async (namespace: string): Promise<IItem[]> => {
  const res: { lavenderKubernetesPodLogByNamespace: IItem[] } = await graphql(
    `
      query call($namespace: String!) {
        lavenderKubernetesPodLogByNamespace(namespace: $namespace) {
          node
          name
          container
          owners {
            kind
            name
            uid
          }
          message
          createdAt
        }
      }
    `,
    { namespace },
  );
  return res.lavenderKubernetesPodLogByNamespace;
};

export const by_owner_uid = async (ownerUid: string): Promise<IItem[]> => {
  const res: { lavenderKubernetesPodLogByOwnerUid: IItem[] } = await graphql(
    `
      query call($ownerUid: String!) {
        lavenderKubernetesPodLogByOwnerUid(ownerUid: $ownerUid) {
          node
          name
          container
          owners {
            kind
            name
            uid
          }
          message
          createdAt
        }
      }
    `,
    { ownerUid },
  );
  return res.lavenderKubernetesPodLogByOwnerUid;
};
