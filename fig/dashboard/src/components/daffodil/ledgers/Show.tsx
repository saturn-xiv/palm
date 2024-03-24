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
import { useNavigate } from "react-router-dom";

import { ILedger } from "../../../api/daffodil";
import Timestamp from "../../../components/Timestamp";

interface IProps {
  item: ILedger;
}

const Widget = ({ item }: IProps) => {
  const navigate = useNavigate();
  return (
    <Card sx={{ maxWidth: 420 }}>
      <CardHeader
        avatar={
          <Avatar sx={{ bgcolor: red[500] }} aria-label="recipe">
            {item.name.charAt(0)}
          </Avatar>
        }
        action={
          <IconButton aria-label="settings">
            <MoreVertIcon />
          </IconButton>
        }
        title={item.name}
        subheader={<Timestamp value={item.updatedAt} />}
      />
      <CardMedia
        component="img"
        height="194"
        image={item.cover.url}
        alt={item.cover.title}
        onClick={() => {
          navigate(`/dashboard/daffodil/ledgers/${item.id}`);
        }}
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
