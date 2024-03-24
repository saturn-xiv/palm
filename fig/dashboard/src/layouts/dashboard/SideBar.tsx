import * as React from "react";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import StorageOutlinedIcon from "@mui/icons-material/StorageOutlined";
import DashboardIcon from "@mui/icons-material/Dashboard";
import ShoppingCartIcon from "@mui/icons-material/ShoppingCart";
import PeopleIcon from "@mui/icons-material/People";
import BarChartIcon from "@mui/icons-material/BarChart";
import LayersIcon from "@mui/icons-material/Layers";
import SettingsAccessibilityOutlinedIcon from "@mui/icons-material/SettingsAccessibilityOutlined";
import ReceiptLongOutlinedIcon from "@mui/icons-material/ReceiptLongOutlined";
import Divider from "@mui/material/Divider";
import List from "@mui/material/List";
import { FormattedMessage } from "react-intl";
import { useNavigate } from "react-router-dom";

const Widget = () => {
  const navigate = useNavigate();
  return (
    <List component="nav">
      <React.Fragment>
        <ListItemButton onClick={() => navigate("/dashboard/daffodil/ledgers")}>
          <ListItemIcon>
            <ReceiptLongOutlinedIcon />
          </ListItemIcon>
          <ListItemText
            primary={<FormattedMessage id="menus.daffodil.title" />}
          />
        </ListItemButton>
        <ListItemButton>
          <ListItemIcon>
            <ShoppingCartIcon />
          </ListItemIcon>
          <ListItemText primary="Orders" />
        </ListItemButton>
        <ListItemButton>
          <ListItemIcon>
            <PeopleIcon />
          </ListItemIcon>
          <ListItemText primary="Customers" />
        </ListItemButton>
        <ListItemButton>
          <ListItemIcon>
            <BarChartIcon />
          </ListItemIcon>
          <ListItemText primary="Reports" />
        </ListItemButton>
        <ListItemButton>
          <ListItemIcon>
            <LayersIcon />
          </ListItemIcon>
          <ListItemText primary="Integrations" />
        </ListItemButton>
      </React.Fragment>

      <Divider sx={{ my: 1 }} />

      <React.Fragment>
        <ListItemButton onClick={() => navigate("/dashboard/main")}>
          <ListItemIcon>
            <DashboardIcon />
          </ListItemIcon>
          <ListItemText
            primary={<FormattedMessage id="menus.dashboard.title" />}
          />
        </ListItemButton>
        <ListItemButton onClick={() => navigate("/dashboard/self")}>
          <ListItemIcon>
            <SettingsAccessibilityOutlinedIcon />
          </ListItemIcon>
          <ListItemText
            primary={<FormattedMessage id="menus.personal.title" />}
          />
        </ListItemButton>
        <ListItemButton onClick={() => navigate("/dashboard/attachments")}>
          <ListItemIcon>
            <StorageOutlinedIcon />
          </ListItemIcon>
          <ListItemText
            primary={<FormattedMessage id="menus.attachments.title" />}
          />
        </ListItemButton>
      </React.Fragment>
    </List>
  );
};

export default Widget;
