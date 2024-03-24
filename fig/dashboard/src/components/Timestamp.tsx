import moment from "moment";

const DATETIME_FORMAT = "llll";

interface IProps {
  value: Date;
}

const Widget = ({ value }: IProps) => {
  // return moment.unix(it).format(DATETIME_FORMAT);
  return <>{moment(value).format(DATETIME_FORMAT)}</>;
};
export default Widget;
