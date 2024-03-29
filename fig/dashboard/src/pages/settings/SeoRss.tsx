import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemText from "@mui/material/ListItemText";
import Typography from "@mui/material/Typography";
import { useIntl } from "react-intl";

interface IProps {
  languages: string[];
}

const Widget = ({ languages }: IProps) => {
  const intl = useIntl();
  return (
    <>
      <Typography variant="h6" gutterBottom>
        RSS Feeds
      </Typography>
      <List>
        {languages.map((x) => (
          <ListItem key={x}>
            <ListItemButton
              component="a"
              href={`/rss/${x}.xml`}
              target="_blank"
            >
              <ListItemText
                primary={intl.formatMessage({ id: `languages.${x}` })}
              />
            </ListItemButton>
          </ListItem>
        ))}
      </List>
    </>
  );
};
export default Widget;
