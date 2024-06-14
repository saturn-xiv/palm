import { ISucceed, IPagination, query } from "./graphql";

export const update_attachment = async (
  id: number,
  title: string
): Promise<ISucceed> => {
  const res = await query<{
    updateAttachment: ISucceed;
  }>(
    `
    mutation call($id: Int!, $title: String!){
      updateAttachment(id: $id, title: $title){
        createdAt
      }
    }
    `,
    { id, title }
  );
  return res.updateAttachment;
};
export interface IAttachment {
  id: number;
  title: string;
  bucket: string;
  object: string;
  size: number;
  contentType: string;
  updatedAt?: Date;
  deletedAt?: Date;
}

export interface IIndexAttachmentResponse {
  items: IAttachment[];
  pagination: IPagination;
}
export const index_attachment = async (
  page: number,
  size: number
): Promise<IIndexAttachmentResponse> => {
  const res = await query<{ indexAttachment: IIndexAttachmentResponse }>(
    `
  query call($pager: Pager!){
    indexAttachment(pager: $pager){
      items{id, title, bucket, object, size, contentType, updatedAt, deletedAt},
      pagination{page, size, total, hasNext, hasPrevious}
    }
  }
  `,
    {
      pager: { page, size },
    }
  );
  return res.indexAttachment;
};
export const index_picture = async (): Promise<IAttachment[]> => {
  const res = await query<{ indexPicture: IAttachment[] }>(
    `
  query call{
    indexPicture{
      id, title, bucket, object, size, contentType, updatedAt, deletedAt
    }
  }
  `,
    {}
  );
  return res.indexPicture;
};

export interface IAttachmentShow {
  id: number;
  title: string;
  url: string;
}

export interface IAttachmentShowResponse {
  url: string;
  title: string;
  size: number;
  contentType: string;
  updatedAt: Date;
}
export const show_attachment_by_id = async (
  id: number,
  ttl: number
): Promise<IAttachmentShowResponse> => {
  const res = await query<{ showAttachmentById: IAttachmentShowResponse }>(
    `
  query call($id: Int!, $ttl: Int){
    showAttachmentById(id: $id, ttl: $ttl){
      url, title, size, contentType, updatedAt
    }
  }
  `,
    {
      id,
      ttl,
    }
  );
  return res.showAttachmentById;
};

interface IUploadUrlResponse {
  bucket: string;
  object: string;
  url: string;
}
export const upload_url = async (
  title: string,
  content_type: string,
  size: number
): Promise<IUploadUrlResponse> => {
  const res = await query<{ uploadAttachmentUrl: IUploadUrlResponse }>(
    `
  mutation call($title: String!, $content_type: String!, $size: Int!, $ttl: Int!){
    uploadAttachmentUrl(title: $title, contentType: $content_type, size: $size, ttl: $ttl){
      bucket, object, url
    }
  }
  `,
    {
      title,
      content_type,
      size,
      ttl: 60 * 60 * 10,
    }
  );
  return res.uploadAttachmentUrl;
};
