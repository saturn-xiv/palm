import Snackbar from "@mui/material/Snackbar";
import Alert, { AlertColor } from "@mui/material/Alert";

export interface IState {
  severity: AlertColor;
  messages: string[];
}

interface IProps {
  severity: AlertColor;
  messages: string[];
  handleClose: () => void;
}

const Widget = ({ severity, messages, handleClose }: IProps) => {
  return (
    <Snackbar
      open={messages.length > 0}
      autoHideDuration={6000}
      anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
      onClose={handleClose}
    >
      <Alert
        onClose={handleClose}
        severity={severity}
        variant="filled"
        sx={{ width: "100%" }}
      >
        {messages.map((x, i) => (
          <div key={i}>{x}</div>
        ))}
      </Alert>
    </Snackbar>
  );
};

export default Widget;
