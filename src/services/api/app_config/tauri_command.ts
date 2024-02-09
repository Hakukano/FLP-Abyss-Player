import { Response, send_tauri_command } from "../../api.ts";
import {
  AppConfig,
  basePath,
  AppConfigMutable,
  AppConfigBrief,
} from "../app_config.ts";

export default class TauriCommand implements AppConfig {
  async index(): Promise<Response<AppConfigBrief>> {
    return await send_tauri_command(basePath, "GET", {});
  }

  async update(app_config: AppConfigMutable): Promise<Response<void>> {
    return await send_tauri_command(basePath, "PUT", app_config);
  }
}
