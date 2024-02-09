import { AppConfigService } from "./api/app_config.ts";
import I18Next from "./locale/i18next.ts";

export class LocaleService {
  appConfigService: AppConfigService;

  constructor(appConfigService: AppConfigService) {
    this.appConfigService = appConfigService;
  }

  async init(): Promise<void> {
    const locale = (await this.appConfigService.index()).body.locale;
    await this.initExtras(locale);
  }

  protected async initExtras(_locale: string): Promise<void> {
    throw "Not Implemented";
  }

  async setLocale(locale: string): Promise<void> {
    const appConfig = (await this.appConfigService.index()).body;
    appConfig.locale = locale;
    await this.appConfigService.update(appConfig);
    await this.setLocaleExtras(locale);
  }

  protected async setLocaleExtras(_locale: string): Promise<void> {
    throw "Not implemented";
  }
}

export async function initialize(
  appConfigService: AppConfigService,
): Promise<LocaleService> {
  const localeService =
    import.meta.env.MODE === "test"
      ? new I18Next(appConfigService)
      : new I18Next(appConfigService);
  await localeService.init();
  return localeService;
}
