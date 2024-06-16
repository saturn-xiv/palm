import { ISucceed, IPagination, query } from "./graphql";

export const update_attachment = async (
  id: number,
  title: string
): Promise<ISucceed> => {
  const res = await query<{
    updateAttachment: ISucceed;
  }>(
    `
    mutation call($id: ID!, $title: String!){
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
  uploadedAt?: Date;
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
      items{id, title, bucket, object, size, contentType, uploadedAt, updatedAt, deletedAt},
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
      id, title, bucket, object, size, contentType, uploadedAt, updatedAt, deletedAt
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
  url?: string;
  item: IAttachment;
}
export const show_attachment_by_id = async (
  id: number,
  ttl: number
): Promise<IAttachmentShowResponse> => {
  const res = await query<{ showAttachment: IAttachmentShowResponse }>(
    `
  query call($id: ID!, $ttl: Int!){
    showAttachment(id: $id, ttl: $ttl){
      url, 
      item{id, title, bucket, object, size, contentType, uploadedAt, updatedAt, deletedAt}
    }
  }
  `,
    {
      id,
      ttl,
    }
  );
  return res.showAttachment;
};

interface IUploadUrlResponse {
  id: number;
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
      id, bucket, object, url
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
export const set_uploaded = async (
  id: number,
  succeed: boolean
): Promise<ISucceed> => {
  const res = await query<{ setAttachmentUploaded: ISucceed }>(
    `
  mutation call($id: ID!, $succeed: Boolean!){
    setAttachmentUploaded(id: $id, succeed: $succeed){
      createdAt
    }
  }
  `,
    {
      id,
      succeed,
    }
  );
  return res.setAttachmentUploaded;
};

export const attach = async (
  id: number,
  resource_type: string,
  resource_id: number
): Promise<ISucceed> => {
  const res = await query<{ attachAttachmentToResource: ISucceed }>(
    `
  mutation call($id: ID!, $object: resource_type!, $resource_id: ID!){
    attachAttachmentToResource(bucket: $bucket, resource_type: $resource_type, resource_id: $resource_id){
      createdAt
    }
  }
  `,
    {
      id,
      resource_type,
      resource_id,
    }
  );
  return res.attachAttachmentToResource;
};

export const detach = async (
  id: number,
  resource_type: string,
  resource_id: number
): Promise<ISucceed> => {
  const res = await query<{ detachAttachmentFromResource: ISucceed }>(
    `
  mutation call($id: ID!, $object: resource_type!, $resource_id: ID!){
    detachAttachmentFromResource(bucket: $bucket, resource_type: $resource_type, resource_id: $resource_id){
      createdAt
    }
  }
  `,
    {
      id,
      resource_type,
      resource_id,
    }
  );
  return res.detachAttachmentFromResource;
};

export const destroy = async (id: number): Promise<ISucceed> => {
  const res = await query<{ destroyAttachment: ISucceed }>(
    `
  mutation call($id: ID!){
    destroyAttachment(id: $id){
      createdAt
    }
  }
  `,
    {
      id,
    }
  );
  return res.destroyAttachment;
};
