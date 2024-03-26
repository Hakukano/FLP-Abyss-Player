import { sendRequest } from "../../api.ts";
import {
  AppConfigService,
  basePath,
  AppConfigMutable,
  AppConfigBrief,
} from "../app_config.ts";
import i18next from "i18next";

export default class Fetch implements AppConfigService {
  async index(): Promise<AppConfigBrief> {
    const resp = await sendRequest(basePath, {}, "GET");
    const body = await resp.json();
    return body as AppConfigBrief;
  }

  async update(appConfig: AppConfigMutable): Promise<Response<void>> {
    const resp: Response<any> = await sendRequest(basePath, "PUT", appConfig);
    await i18next.changeLanguage(appConfig.locale);
    return resp;
  }
}
