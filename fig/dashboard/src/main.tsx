import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import "./main.css";

import App from "./App.tsx";
import { index_locale_by_lang } from "./api/daffodil";
import { get as detect_locale, load as i18n_load } from "./i18n";

const lang = detect_locale();
const locale = i18n_load(lang);

index_locale_by_lang(lang).then((res) => {
  const messages = res.reduce((obj, it) => {
    obj[it.code] = it.message;
    return obj;
  }, {} as Record<string, string>);

  createRoot(document.getElementById("root")!).render(
    <StrictMode>
      <App messages={messages} lang={lang} locale={locale} />
    </StrictMode>
  );
});
