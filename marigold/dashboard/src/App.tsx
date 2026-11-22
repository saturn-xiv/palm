import { IntlProvider } from "react-intl";
import { Provider } from "react-redux";

import Router from "./Router";
import { DEFAULT as DEFAULT_LANGUAGE } from "./locales";
import store from "./store";

export interface IProps {
  locale: string;
  messages: Record<string, string>;
}
const Widget = ({ locale, messages }: IProps) => (
  <Provider store={store}>
    <IntlProvider
      messages={messages}
      locale={locale}
      defaultLocale={DEFAULT_LANGUAGE}
    >
      <Router />
    </IntlProvider>
  </Provider>
);

export default Widget;
