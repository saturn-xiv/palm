import { useState } from "react";
import { useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { EmailForm, IEmailFormValues } from "./users/sign-up";
import { install } from "../api/daffodil";
import { guess_timezone } from "../utils";
import { SIGN_IN_PATH } from "../reducers/current-user";
import { IAlert } from "../components";
import { IError } from "../api";

const Widget = () => {
  const [alert, setAlert] = useState<IAlert>();
  const navigate = useNavigate();
  const intl = useIntl();
  const handleSubmit = (values: IEmailFormValues) => {
    install(
      {
        title: "Demo site",
        subhead: "Demo",
        description: "Demo information",
        copyright: `~ ${new Date().getFullYear()}`,
      },
      {
        realName: values.realName,
        nickname: values.nickname,
        email: values.email,
        password: values.password,
        timezone: guess_timezone(),
      }
    )
      .then(() => {
        setAlert({
          color: "success",
          messages: [intl.formatMessage({ id: "flashes.succeed" })],
        });
        // navigate(SIGN_IN_PATH);
      })
      .catch((reason: IError[]) => {
        setAlert({
          color: "error",
          messages: reason.map((x) => x.message),
        });
      });
  };
  return (
    <EmailForm
      title={intl.formatMessage({ id: "pages.install.title" })}
      alert={alert}
      handleSubmit={handleSubmit}
    />
  );
};
export default Widget;
