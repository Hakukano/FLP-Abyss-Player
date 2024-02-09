import { AppConfigService } from "./api/app_config.ts";
import I18Next from "./locale/i18next.ts";

export interface LocaleService {
  init(): Promise<void>;
  setLocale(locale: string): Promise<void>;
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
