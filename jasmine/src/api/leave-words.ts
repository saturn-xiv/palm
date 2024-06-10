import { ISucceed, query } from "./graphql";

export const create_leave_word = async (
  content: string,
  editor: string
): Promise<ISucceed> => {
  const res = await query<{ createLeaveWord: ISucceed }>(
    `
mutation call($content: String!, $editor: TextEditor!){
  createLeaveWord(content: $content, editor: $editor){
    createdAt
  }
}
`,
    { content, editor }
  );
  return res.createLeaveWord;
};
