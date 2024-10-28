import { useState } from "react";
import { useIntl } from "react-intl";

import { IAlert } from "../../../components";
import { user_unlock_by_email } from "../../../api/daffodil";
import { IError } from "../../../api";
import { ByEmailForm, IByEmailFormValues } from "../confirm/by-email";

const Widget = () => {
  const [alert, setAlert] = useState<IAlert>();
  const intl = useIntl();
  const handleSubmit = (values: IByEmailFormValues) => {
    user_unlock_by_email(values.user)
      .then(() => {
        setAlert({
          color: "success",
          messages: [
            intl.formatMessage({ id: "pages.users.unlock.instruction" }),
          ],
        });
      })
      .catch((reason: IError[]) => {
        setAlert({
          color: "error",
          messages: reason.map((x) => x.message),
        });
      });
  };
  return (
    <ByEmailForm
      title={intl.formatMessage({ id: "pages.users.unlock.title" })}
      alert={alert}
      handleSubmit={handleSubmit}
    />
  );
};
export default Widget;
