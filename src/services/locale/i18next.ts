import i18next from "i18next";

import { LocaleService } from "../locale.ts";
import { AppConfigService } from "../api/app_config.ts";
import { enUS, jaJP } from "./translations.ts";

export default class I18Next extends LocaleService {
  constructor(appConfigService: AppConfigService) {
    super(appConfigService);
  }

  async initExtras(locale: string): Promise<void> {
    await i18next.init({
      debug: import.meta.env.DEV,
      lng: locale,
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
  }

  async setLocaleExtras(locale: string): Promise<void> {
    await i18next.changeLanguage(locale);
  }
}
