import { Response, send_tauri_command } from "../../api.ts";
import { AppConfig, basePath, IndexBody } from "../app_config.ts";

export default class TauriCommand implements AppConfig {
  async index(): Promise<Response<IndexBody>> {
    return await send_tauri_command(basePath, "GET", {});
  }
}
