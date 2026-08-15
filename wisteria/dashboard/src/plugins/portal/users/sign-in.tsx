import { FormattedMessage, useIntl } from "react-intl";

import Title from "../../../layouts/Title";

const Widget = () => {
  const intl = useIntl();
  // TODO
  return (
    <>
      <Title
        value={intl.formatMessage({ id: "auth.devise.shared.links.sign_in" })}
      />
      <FormattedMessage id="buttons.ok" />
    </>
  );
};

export default Widget;
