import "./scss/styles.scss";

import React from "react";
import ReactDOM from "react-dom/client";
import i18next from "i18next";
import { initReactI18next } from "react-i18next";

import App from "./app.tsx";
import { ApiServices } from "./services/api.ts";

import translations from "./translations.ts";

async function main() {
  localStorage.clear();

  const apiServices = new ApiServices();

  const appConfig = await apiServices.appConfig.index();

  await i18next.use(initReactI18next).init({
    debug: import.meta.env.DEV,
    lng: appConfig.locale,
    fallbackLng: "en-US",
    resources: {
      "en-US": {
        translation: translations["en-US"],
      },
      "ja-JP": {
        translation: translations["ja-JP"],
      },
    },
  });

  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <App apiServices={apiServices} />
    </React.StrictMode>,
  );
}

main().catch((err) => console.log(err));
