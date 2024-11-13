import { Suspense } from "react";
import { Provider } from "react-redux";
import { IntlProvider } from "react-intl";
import { ConfigProvider } from "antd";

import Router from "./Router";
import store from "./store";
import { DEFAULT_LANGUAGE, Locale } from "./i18n";
import Loading from "./pages/loading";

interface IProps {
  locale: Locale;
  lang: string;
  messages: Record<string, string>;
}

const Widget = ({ lang, locale, messages }: IProps) => {
  return (
    <IntlProvider
      messages={messages}
      locale={lang}
      defaultLocale={DEFAULT_LANGUAGE}
    >
      <ConfigProvider locale={locale}>
        <Provider store={store}>
          <Suspense fallback={<Loading />}>
            <Router />
          </Suspense>
        </Provider>
      </ConfigProvider>
    </IntlProvider>
  );
};

export default Widget;
