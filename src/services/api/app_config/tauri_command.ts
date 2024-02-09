import { Response, sendTauriCommand } from "../../api.ts";
import {
  AppConfigService,
  basePath,
  AppConfigMutable,
  AppConfigBrief,
} from "../app_config.ts";
import i18next from "i18next";

export default class TauriCommand implements AppConfigService {
  async index(): Promise<Response<AppConfigBrief>> {
    return await sendTauriCommand(basePath, "GET", {});
  }

  async update(appConfig: AppConfigMutable): Promise<Response<void>> {
    const resp: Response<any> = await sendTauriCommand(
      basePath,
      "PUT",
      appConfig,
    );
    await i18next.changeLanguage(appConfig.locale);
    return resp;
  }
}
