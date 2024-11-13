import { Suspense } from "react";
import { Provider } from "react-redux";
import { IntlProvider } from "react-intl";

import Router from "./Router";
import store from "./store";
import { DEFAULT_LANGUAGE } from "./i18n";
import Loading from "./pages/loading";

interface IProps {
  locale: string;
  messages: Record<string, string>;
}

const Widget = ({ locale, messages }: IProps) => {
  return (
    <IntlProvider
      messages={messages}
      locale={locale}
      defaultLocale={DEFAULT_LANGUAGE}
    >
      <Provider store={store}>
        <Suspense fallback={<Loading />}>
          <Router />
        </Suspense>
      </Provider>
    </IntlProvider>
  );
};

export default Widget;
