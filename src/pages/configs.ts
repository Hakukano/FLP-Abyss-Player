import Page from "../pages.ts";
import AppConfig from "../components/app_config.ts";
import { AppConfigService } from "../services/api/app_config.ts";

export default class Configs extends Page {
  constructor(appConfigService: AppConfigService) {
    const appConfig = new AppConfig(appConfigService);
    super([appConfig]);
  }
}
