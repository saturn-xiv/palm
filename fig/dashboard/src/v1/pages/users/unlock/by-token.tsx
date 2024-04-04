import { useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { useIntl } from "react-intl";

import { unlock_by_token } from "../../../api/camelia";
import { IErrorMessage } from "../../../api/graphql";
import { SIGN_IN_PATH } from "../../../reducers/current-user";
import { useAppDispatch } from "../../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../../reducers/message-box";

export function Component() {
  const dispatch = useAppDispatch();
  const { token } = useParams();
  const intl = useIntl();
  const navigate = useNavigate();

  useEffect(() => {
    unlock_by_token(token || "")
      .then(() => {
        dispatch(
          success_box([
            intl.formatMessage({ id: "users.unlock.by-token.succeed" }),
          ])
        );
        navigate(SIGN_IN_PATH);
      })
      .catch((reason: IErrorMessage[]) => {
        dispatch(error_box(reason.map((x) => x.message)));
      });
  });
  return <></>;
}
