import { Suspense } from "react";
import { IntlProvider } from "react-intl";

import Loading from "./components/Loading";
import Router from "./Router";
import { detect as detect_locale, messages as get_messages } from "./locales";

const locale = detect_locale();
const messages = get_messages(locale);

const Widget = () => {
  return (
    <Suspense fallback={<Loading />}>
      <IntlProvider locale={locale} messages={messages}>
        <Router />
      </IntlProvider>
    </Suspense>
  );
};

export default Widget;
