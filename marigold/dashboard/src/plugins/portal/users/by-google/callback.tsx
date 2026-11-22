import { FormattedMessage, useIntl } from "react-intl";

import Title from "../../../../layouts/Title";

const Widget = () => {
  const intl = useIntl();
  // TODO
  // https://oauth2.example.com/auth?error=access_denied
  // https://oauth2.example.com/auth?code=4/P7q7W91a-oMsCeLvIaQm6bTrgtp7&state=xxx
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
