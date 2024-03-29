import { IAuthor } from "../../api/camelia";

interface IProps {
  authors: IAuthor[];
  handleRefresh: () => void;
}

const Widget = ({ authors }: IProps) => {
  return <>author</>;
};
export default Widget;
