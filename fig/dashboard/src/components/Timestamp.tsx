import Moment from "react-moment";

interface IProps {
  value: Date;
}

const Widget = ({ value }: IProps) => {
  return <Moment date={value} format="llll" />;
};
export default Widget;
