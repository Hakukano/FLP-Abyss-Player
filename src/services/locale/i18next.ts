import i18next from "i18next";

import { LocaleService } from "../locale.ts";
import { AppConfigService } from "../api/app_config.ts";
import { enUS, jaJP } from "./translations.ts";

export default class I18Next implements LocaleService {
  appConfigService: AppConfigService;

  constructor(appConfigService: AppConfigService) {
    this.appConfigService = appConfigService;
  }

  async init(): Promise<void> {
    const locale = (await this.appConfigService.index()).body.locale;
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

  async setLocale(locale: string): Promise<void> {
    const appConfig = (await this.appConfigService.index()).body;
    appConfig.locale = locale;
    await this.appConfigService.update(appConfig);
    await i18next.changeLanguage(locale);
  }
}
