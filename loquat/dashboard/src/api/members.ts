import type { IPage, IPagination } from ".";

export interface ILogsTableItem {
  id: number;
  ip: string;
  message: string;
  createdAt: Date;
}

export interface ILogsResponse {
  items: ILogsTableItem[];
  pagination: IPagination;
}

export const get_logs = async (page: IPage): Promise<ILogsResponse> => {
  const items: ILogsTableItem[] = [];
  for (
    let i = 1 + page.size * page.index;
    i <= page.size * (page.index + 1);
    i++
  ) {
    items.push({
      id: i + 10000,
      message: `message ${i}`,
      ip: "n/a",
      createdAt: new Date(),
    });
  }
  return {
    items,
    pagination: {
      index: page.index,
      size: page.size,
      total: page.size * 24 + 1,
    },
  };
};
