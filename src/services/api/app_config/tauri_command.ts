import { Response, sendTauriCommand } from "../../api.ts";
import {
  AppConfigService,
  basePath,
  AppConfigMutable,
  AppConfigBrief,
} from "../app_config.ts";

export default class TauriCommand implements AppConfigService {
  async index(): Promise<Response<AppConfigBrief>> {
    return await sendTauriCommand(basePath, "GET", {});
  }

  async update(app_config: AppConfigMutable): Promise<Response<void>> {
    return await sendTauriCommand(basePath, "PUT", app_config);
  }
}
