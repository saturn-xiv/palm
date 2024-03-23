import { useState } from "react";
import IconButton from "@mui/material/IconButton";
import LogoutOutlinedIcon from "@mui/icons-material/LogoutOutlined";
import { useConfirm } from "material-ui-confirm";
import { useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { useAppDispatch } from "../../hooks";
import { signOut, SIGN_IN_PATH } from "../../reducers/current-user";
import { sign_out } from "../../api/camelia";
import { IErrorMessage } from "../../api/graphql";
import MessageBox, { IState as IMessageBox } from "../../components/MessageBox";

const Widget = () => {
  const [messageBox, setMessageBox] = useState<IMessageBox>({
    messages: [],
    severity: "info",
  });
  const dispatch = useAppDispatch();
  const confirm = useConfirm();
  const intl = useIntl();
  const navigate = useNavigate();
  return (
    <IconButton
      onClick={() => {
        confirm({
          title: intl.formatMessage({
            id: "layouts.are-you-sure",
          }),
          description: intl.formatMessage({
            id: "users.sign-out.confirm",
          }),
        })
          .then(() => {
            sign_out()
              .then(() => {
                dispatch(signOut());
                navigate(SIGN_IN_PATH);
              })
              .catch((reason: IErrorMessage[]) => {
                setMessageBox({
                  messages: reason.map((x) => x.message),
                  severity: "error",
                });
              });
          })
          .catch(() => {});
      }}
      color="inherit"
    >
      <LogoutOutlinedIcon />
      <MessageBox
        severity={messageBox.severity}
        messages={messageBox.messages}
        handleClose={() => setMessageBox({ messages: [], severity: "info" })}
      />
    </IconButton>
  );
};

export default Widget;
