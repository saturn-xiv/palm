import Router from "./Router";
import { IntlProvider } from "react-intl";

import { DEFAULT as DEFAULT_LANGUAGE } from "./locales";

export interface IProps {
  locale: string;
  messages: Record<string, string>;
}
const Widget = ({ locale, messages }: IProps) => (
  <IntlProvider
    messages={messages}
    locale={locale}
    defaultLocale={DEFAULT_LANGUAGE}
  >
    <Router />
  </IntlProvider>
);

export default Widget;
