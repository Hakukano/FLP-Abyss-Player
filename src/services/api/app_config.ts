import { Response } from "../api.ts";
import TauriCommand from "./app_config/tauri_command.ts";

export const basePath = ["app_config"];

export interface AppConfigImmutable {}

export interface AppConfigMutable {
  locale: string;
  root_path: string | null;
}

export interface AppConfigBrief extends AppConfigImmutable, AppConfigMutable {}

export interface AppConfigDetails
  extends AppConfigImmutable,
    AppConfigMutable {}

export interface AppConfig {
  index(): Promise<Response<AppConfigBrief>>;
  update(app_config: AppConfigMutable): Promise<Response<void>>;
}

const appConfig =
  import.meta.env.MODE === "test" ? new TauriCommand() : new TauriCommand();

export default appConfig;
