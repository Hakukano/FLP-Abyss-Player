// noinspection ES6ConvertVarToLetConst

import { AppConfigService } from "./services/api/app_config.ts";
import { LocaleService } from "./services/locale.ts";
import Page from "./pages.ts";

declare global {
  var appConfigService: AppConfigService;
  var localeService: LocaleService;
  var page: Page;
}
