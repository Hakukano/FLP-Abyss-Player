import "./scss/styles.scss";

import React from "react";
import ReactDOM from "react-dom/client";
import i18next from "i18next";
import { initReactI18next } from "react-i18next";

import App from "./app.tsx";
import { enUS, jaJP } from "./translations.ts";
import { initializeAppConfigService } from "./services/api/app_config.ts";

async function main() {
  const appConfigService = initializeAppConfigService();
  const appConfig = (await appConfigService.index()).body;

  await i18next.use(initReactI18next).init({
    debug: import.meta.env.DEV,
    lng: appConfig.locale,
    fallbackLng: "en-US",
    resources: {
      "en-US": {
        translation: enUS,
      },
      "ja-JP": {
        translation: jaJP,
      },
    },
  });

  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <App appConfigService={appConfigService} />
    </React.StrictMode>,
  );
}

main().catch((err) => console.log(err));
