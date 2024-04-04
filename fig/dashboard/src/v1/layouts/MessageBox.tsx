import Snackbar from "@mui/material/Snackbar";
import Alert from "@mui/material/Alert";

import { useAppSelector, useAppDispatch } from "../hooks";
import { hide, messageBox as selectMessageBox } from "../reducers/message-box";

const Widget = () => {
  const message_box = useAppSelector(selectMessageBox);
  const dispatch = useAppDispatch();
  const handleClose = () => {
    dispatch(hide());
  };
  return (
    <Snackbar
      open={message_box.messages.length > 0}
      autoHideDuration={6000}
      anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
      onClose={handleClose}
    >
      <Alert
        onClose={handleClose}
        severity={message_box.severity}
        variant="filled"
        sx={{ width: "100%" }}
      >
        {message_box.messages.map((x, i) => (
          <div key={i}>{x}</div>
        ))}
      </Alert>
    </Snackbar>
  );
};

export default Widget;
