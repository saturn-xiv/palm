import { IPager, IPagination, query } from ".";

const INDEX_PAGE = `
query call($pager: Pager!){
    indexCmsPage(pager: $pager){
      items{id, lang, slug, title, body, bodyEditor, template, status, lockedAt, deletedAt, updatedAt},
      pagination{total}
    }
}
`;
export interface IPage {
  id: number;
  lang: number;
  slug: string;
  title: string;
  body: string;
  bodyEditor: string;
  template: string;
  status: string;
  lockedAt?: Date;
  deletedAt?: Date;
  updatedAt: Date;
}
interface IIndexPageResponse {
  items: IPage[];
  pagination: IPagination;
}
export const index_page = async (
  pager: IPager
): Promise<IIndexPageResponse> => {
  const res: { indexCmsPage: IIndexPageResponse } = await query(INDEX_PAGE, {
    pager,
  });
  return res.indexCmsPage;
};
