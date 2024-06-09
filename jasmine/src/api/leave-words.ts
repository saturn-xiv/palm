import { post as http_post } from ".";

export const create_leave_word = async (
  content: string,
  editor: string
): Promise<Record<string, string>> => {
  return await http_post(`/api/leave-words`, {
    content,
    editor,
  });
};
