import { Suspense } from "react";
import { Provider } from "react-redux";
import { IntlProvider } from "react-intl";
import RefreshRoundedIcon from "@mui/icons-material/RefreshRounded";

import Router from "./Router";
import store from "./store";
import { DEFAULT_LANGUAGE } from "./i18n";

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
        <Suspense fallback={<RefreshRoundedIcon />}>
          <Router />
        </Suspense>
      </Provider>
    </IntlProvider>
  );
};

export default Widget;
