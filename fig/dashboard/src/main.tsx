import React from "react";
import ReactDOM from "react-dom/client";
import { IntlProvider } from "react-intl";
import { Provider as ReduxProvider } from "react-redux";

import "./assets/dropzone.css";

import Router from "./Router";
import { store } from "./store";
import { get as get_locale } from "./locales";
import { index_locale } from "./api/camelia";

const locale = get_locale();

index_locale(locale).then((messages) => {
  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
      <ReduxProvider store={store}>
        <IntlProvider locale={locale} messages={messages}>
          <Router />
        </IntlProvider>
      </ReduxProvider>
    </React.StrictMode>
  );
});
