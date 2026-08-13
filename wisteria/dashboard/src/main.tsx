import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import App from "./App.tsx";
import { get as get_locale, dayjs } from "./locales";
import { by_lang as locales_by_lang } from "./api/portal/locale";

import "./index.css";

const lang = get_locale();
dayjs(lang);

locales_by_lang(lang).then((r) => {
  if (r.data) {
    createRoot(document.getElementById("root")!).render(
      <StrictMode>
        <App />
      </StrictMode>,
    );
  }
});
