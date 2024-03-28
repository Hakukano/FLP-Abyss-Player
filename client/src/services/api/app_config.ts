import Remote from "./app_config/remote.ts";

export const basePath = ["app_config"];

export interface AppConfigImmutable {}

export interface AppConfigMutable {
  locale: string;
}

export interface AppConfigBrief extends AppConfigImmutable, AppConfigMutable {}

export interface AppConfigDetails
  extends AppConfigImmutable,
    AppConfigMutable {}

export interface AppConfigService {
  index(): Promise<AppConfigBrief>;
  update(appConfig: AppConfigMutable): Promise<void>;
}

export function instantiateAppConfigService(): AppConfigService {
  return import.meta.env.MODE === "test" ? new Remote() : new Remote();
}
