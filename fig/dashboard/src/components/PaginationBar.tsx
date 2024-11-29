import { Pagination } from "antd";

interface IProps {
  defaultCurrent: number;
  total: number;
  handleChange: (page: number, size: number) => void;
}

const Widget = ({ total, defaultCurrent, handleChange }: IProps) => {
  return (
    <Pagination
      showSizeChanger
      showQuickJumper
      onShowSizeChange={handleChange}
      defaultCurrent={defaultCurrent}
      total={total}
    />
  );
};

export default Widget;
