import Card from "@mui/material/Card";
import CardHeader from "@mui/material/CardHeader";
import CardMedia from "@mui/material/CardMedia";
import CardContent from "@mui/material/CardContent";
import CardActions from "@mui/material/CardActions";
import Avatar from "@mui/material/Avatar";
import IconButton from "@mui/material/IconButton";
import Typography from "@mui/material/Typography";
import { red } from "@mui/material/colors";
import FavoriteIcon from "@mui/icons-material/Favorite";
import ShareIcon from "@mui/icons-material/Share";
import MoreVertIcon from "@mui/icons-material/MoreVert";
import FileDownloadIcon from "@mui/icons-material/FileDownload";

import { ILedger } from "../../../api/daffodil";
import { timestamp_to_str } from "../../../utils";
import ledger_cover_img from "../../../assets/market.svg";

interface IProps {
  item: ILedger;
}

const Widget = ({ item }: IProps) => {
  return (
    <Card sx={{ maxWidth: 345 }}>
      <CardHeader
        avatar={
          <Avatar sx={{ bgcolor: red[500] }} aria-label="recipe">
            R
          </Avatar>
        }
        action={
          <IconButton aria-label="settings">
            <MoreVertIcon />
          </IconButton>
        }
        title={item.name}
        subheader={timestamp_to_str(item.updatedAt)}
      />
      <CardMedia
        component="img"
        height="194"
        image={item.cover || ledger_cover_img}
        alt={item.name}
      />
      <CardContent>
        <Typography variant="body2" color="text.secondary">
          {item.summary}
        </Typography>
      </CardContent>
      <CardActions disableSpacing>
        <IconButton aria-label="add to favorites">
          <FavoriteIcon />
        </IconButton>
        <IconButton aria-label="share">
          <ShareIcon />
        </IconButton>
        <IconButton aria-label="export">
          <FileDownloadIcon />
        </IconButton>
      </CardActions>
    </Card>
  );
};

export default Widget;
