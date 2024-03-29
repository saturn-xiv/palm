interface IProps {
  title: string;
  subhead: string;
  description: string;
  copyright: string;
  handleRefresh: () => void;
}
const Widget = ({ title, subhead, description, copyright }: IProps) => {
  return <>base info</>;
};
export default Widget;
