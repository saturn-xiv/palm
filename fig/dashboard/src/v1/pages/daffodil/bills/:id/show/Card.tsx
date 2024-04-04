import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Typography from "@mui/material/Typography";
import { FormattedMessage } from "react-intl";

import { IBill } from "../../../../../api/daffodil";
import AmountShow from "../../../../../components/AmountShow";
import Moment from "../../../../../components/Moment";

interface IProps {
  item: IBill;
}

const Widget = ({ item }: IProps) => {
  return (
    <Card sx={{ minWidth: 250 }}>
      <CardContent>
        <Typography sx={{ fontSize: 14 }} color="text.secondary" gutterBottom>
          <Moment date={item.paidAt} />
        </Typography>
        <Typography variant="h5" component="div">
          <AmountShow amount={item.amount} currency={item.currency} />
        </Typography>
        <Typography sx={{ mb: 1.5 }} color="text.secondary">
          <FormattedMessage id="form.fields.merchant.label" />: {item.merchant}
          <br />
          <FormattedMessage id="form.fields.category.label" />: {item.category}
          <br />
          <FormattedMessage id="form.fields.paid-by.label" />: {item.paidBy}
        </Typography>
        <Typography variant="body2">{item.summary}</Typography>
      </CardContent>
      <CardActions />
    </Card>
  );
};

export default Widget;
