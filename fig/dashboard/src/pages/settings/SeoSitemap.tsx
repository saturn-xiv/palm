import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemText from "@mui/material/ListItemText";
import Typography from "@mui/material/Typography";

interface IProps {
  languages: string[];
}

const Widget = ({ languages }: IProps) => {
  return (
    <>
      <Typography variant="h6" gutterBottom>
        Sitemap
      </Typography>
      <List>
        {["/robots.txt", "/sitemap.xml"]
          .concat(languages.map((x) => `/sitemap/${x}.xml`))
          .map((x) => (
            <ListItem key={x}>
              <ListItemButton component="a" href={x} target="_blank">
                <ListItemText primary={x} />
              </ListItemButton>
            </ListItem>
          ))}
      </List>
    </>
  );
};
export default Widget;
