import { sendRequest } from "../../api.ts";
import {
  AppConfigService,
  basePath,
  AppConfigMutable,
  AppConfigBrief,
} from "../app_config.ts";
import i18next from "i18next";

export default class Remote implements AppConfigService {
  async index(): Promise<AppConfigBrief> {
    const resp = await sendRequest("GET", basePath);
    const body = await resp.json();
    return body as AppConfigBrief;
  }

  async update(appConfig: AppConfigMutable): Promise<void> {
    await sendRequest("PUT", basePath, { body: appConfig });
    await i18next.changeLanguage(appConfig.locale);
  }
}
