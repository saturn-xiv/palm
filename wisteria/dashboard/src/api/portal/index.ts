export interface IPage {
  index: number;
  size: number;
  total: number;
  hasNext: boolean;
  hasPrevious: boolean;
}
export interface IPagination {
  index: number;
  size: number;
}

export interface ISucceeded {
  createdAt: Date;
}
