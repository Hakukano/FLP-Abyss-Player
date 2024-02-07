import { Response } from "../api.ts";
import TauriCommand from "./app_config/tauri_command.ts";

export const basePath = ["app_config"];

export interface IndexBody {
  locale: string;
  root_path: string | null;
}

export interface AppConfig {
  index(): Promise<Response<IndexBody>>;
}

const appConfig =
  import.meta.env.MODE === "test" ? new TauriCommand() : new TauriCommand();

export default appConfig;
