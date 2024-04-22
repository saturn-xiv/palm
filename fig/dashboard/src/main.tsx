import React from "react";
import ReactDOM from "react-dom/client";
import { IntlProvider } from "react-intl";
import { Provider as ReduxProvider } from "react-redux";
import { ConfigProvider as AntdConfigProvider } from "antd";
import { ProConfigProvider as AntdProConfigProvider } from "@ant-design/pro-components";

import "./main.css";

import Router from "./Router";
import { store } from "./store";
import { get as get_locale, antd as antd_locale } from "./locales";
import { locales as fetch_locales_by_lang } from "./api/camelia";
import { LocaleClient } from "./protocols/lilac/AuthServiceClientPb";
import { LocaleByLangRequest } from "./protocols/lilac/auth_pb";
import { GRPC_HOST, grpc_metadata } from "./api";

const lang = get_locale();
const LAYOUT_ID = "root-pro-layout";

const client = new LocaleClient(GRPC_HOST);
const request = new LocaleByLangRequest();
request.setLang(lang);
client.byLang(request, grpc_metadata()).then((res) => {
  console.log(res);
  const messages = {};
  ReactDOM.createRoot(document.getElementById("root")!).render(
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

// fetch_locales_by_lang(lang).then((messages) => {
//   ReactDOM.createRoot(document.getElementById("root")!).render(
//     <React.StrictMode>
//       <ReduxProvider store={store}>
//         <IntlProvider locale={lang} messages={messages}>
//           <div
//             id={LAYOUT_ID}
//             style={{
//               height: "100vh",
//               overflow: "auto",
//             }}
//           >
//             <AntdProConfigProvider hashed={false}>
//               <AntdConfigProvider
//                 locale={antd_locale(lang)}
//                 getTargetContainer={() => {
//                   return document.getElementById(LAYOUT_ID) || document.body;
//                 }}
//               >
//                 <Router />
//               </AntdConfigProvider>
//             </AntdProConfigProvider>
//           </div>
//         </IntlProvider>
//       </ReduxProvider>
//     </React.StrictMode>
//   );
// });
