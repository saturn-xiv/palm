import React from "react";
import ReactDOM from "react-dom/client";
import { IntlProvider } from "react-intl";
import { Provider as ReduxProvider } from "react-redux";
import { ConfigProvider as AntdConfigProvider } from "antd";
import { ProConfigProvider as AntdProConfigProvider } from "@ant-design/pro-components";

import "./index.css";

import Router from "./Router";
import { store } from "./store";
import { get as get_locale, antd as antd_locale } from "./locales";
import { intl_messages_by_lang } from "./api/locales";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);

const lang = get_locale();
const LAYOUT_ID = "root-pro-layout";

intl_messages_by_lang(lang).then((messages) => {
  root.render(
    <React.StrictMode>
      <ReduxProvider store={store}>
        <IntlProvider locale={lang} messages={messages}>
          <div
            id={LAYOUT_ID}
            style={{
              height: "100vh",
              overflow: "auto",
            }}
          >
            <AntdProConfigProvider hashed={false}>
              <AntdConfigProvider
                locale={antd_locale(lang)}
                getTargetContainer={() => {
                  return document.getElementById(LAYOUT_ID) || document.body;
                }}
              >
                <Router />
              </AntdConfigProvider>
            </AntdProConfigProvider>
          </div>
        </IntlProvider>
      </ReduxProvider>
    </React.StrictMode>
  );
});
