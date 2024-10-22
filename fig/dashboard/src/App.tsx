import { Provider } from "react-redux";
import { IntlProvider } from "react-intl";

import Router from "./Router";
import store from "./store";
import { DEFAULT_LANGUAGE, get as detect_locale } from "./i18n";

const Widget = () => {
  const locale = detect_locale();
  return (
    <IntlProvider
      messages={{}}
      locale={locale}
      defaultLocale={DEFAULT_LANGUAGE}
    >
      <Provider store={store}>
        <Router />
      </Provider>
    </IntlProvider>
  );
};

export default Widget;
