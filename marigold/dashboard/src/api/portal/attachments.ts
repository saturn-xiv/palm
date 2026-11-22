import graphql from "../../graphql";
import { type IPagination, type ISucceeded } from ".";

export interface IItem {
  id: number;
  bucket: string;
  object: string;
  title: string;
  contentType: string;
  size: number;
  uploadedAt?: Date;
  updatedAt: Date;
}

export const show = async (id: number): Promise<IItem> => {
  const res: { showAttacchment: IItem } = await graphql(
    `
      query call($id: Int!) {
        showAttacchment(id: $id) {
          id
          bucket
          object
          title
          contentType
          size
          uploadedAt
          updatedAt
        }
      }
    `,
    { id },
  );
  return res.showAttacchment;
};
interface IIndexResponse {
  items: IItem[];
  pagination: IPagination;
}

export const index = async (
  index: number,
  size: number,
): Promise<IIndexResponse> => {
  const res: { indexAttacchment: IIndexResponse } = await graphql(
    `
      query call($page: Page!) {
        indexAttachment(page: $page) {
          items {
            id
            bucket
            object
            title
            contentType
            size
            uploadedAt
            updatedAt
          }
          pagination {
            index
            size
            total
            hasNext
            hasPrevious
          }
        }
      }
    `,
    { page: { index, size } },
  );
  return res.indexAttacchment;
};

export const destroy = async (id: number): Promise<ISucceeded> => {
  const res: { destroyAttachment: ISucceeded } = await graphql(
    `
      mutation call($id: Int!) {
        destroyAttachment(id: $id) {
          createdAt
        }
      }
    `,
    { id },
  );
  return res.destroyAttachment;
};
