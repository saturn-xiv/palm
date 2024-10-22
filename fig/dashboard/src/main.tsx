import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import App from "./App";
import { index_locale_by_lang } from "./api/daffodil";
import { get as detect_locale } from "./i18n";

import "./index.css";

const locale = detect_locale();

index_locale_by_lang(locale).then((res) => {
  const messages = res.reduce((obj, it) => {
    obj[it.code] = it.message;
    return obj;
  }, {} as Record<string, string>);

  createRoot(document.getElementById("root")!).render(
    <StrictMode>
      <App messages={messages} locale={locale} />
    </StrictMode>
  );
});
