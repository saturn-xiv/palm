import IconButton from "@mui/material/IconButton";
import LogoutOutlinedIcon from "@mui/icons-material/LogoutOutlined";
import { useConfirm } from "material-ui-confirm";
import { useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { useAppDispatch } from "../../hooks";
import { signOut, SIGN_IN_PATH } from "../../reducers/current-user";
import { sign_out } from "../../api/camelia";
import { IErrorMessage } from "../../api/graphql";
import {
  success as success_box,
  error as error_box,
} from "../../reducers/message-box";

const Widget = () => {
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
                dispatch(
                  success_box([
                    intl.formatMessage({ id: "users.sign-out.succeed" }),
                  ])
                );
                navigate(SIGN_IN_PATH);
              })
              .catch((reason: IErrorMessage[]) => {
                dispatch(error_box(reason.map((x) => x.message)));
              });
          })
          .catch(() => {});
      }}
      color="inherit"
    >
      <LogoutOutlinedIcon />
    </IconButton>
  );
};

export default Widget;
