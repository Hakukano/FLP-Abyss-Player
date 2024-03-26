import { sendRequest, sendRequestJson } from "../../api.ts";
import {
  AppConfigService,
  basePath,
  AppConfigMutable,
  AppConfigBrief,
} from "../app_config.ts";
import i18next from "i18next";

export default class Remote implements AppConfigService {
  index(): Promise<AppConfigBrief> {
    return sendRequestJson("GET", basePath);
  }

  async update(appConfig: AppConfigMutable): Promise<void> {
    await sendRequest("PUT", basePath, { body: appConfig });
    await i18next.changeLanguage(appConfig.locale);
  }
}
