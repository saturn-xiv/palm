import moment from "moment";

interface IProps {
  date: Date;
  format?: string;
  toNow?: boolean;
}

const Widget = ({ date, format, toNow }: IProps) => {
  if (toNow) {
    return <>{moment(date).toNow()}</>;
  }
  return <>{moment(date).format(format || "llll")}</>;
};
export default Widget;
