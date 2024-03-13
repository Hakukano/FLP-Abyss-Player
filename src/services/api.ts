import { invoke } from "@tauri-apps/api/core";
import { MetaCmpBy } from "../utils/meta";
import {
  AppConfigService,
  instantiateAppConfigService,
} from "./api/app_config";
import { EntryService, instantiateEntryService } from "./api/entry";
import { GroupService, instantiateGroupService } from "./api/group";
import { PlaylistService, instantiatePlaylistService } from "./api/playlist";
import { ScannerService, instantiateScannerService } from "./api/scanner";
import { SessionService, instantiateSessionService } from "./api/session";

type Method = "POST" | "GET" | "PUT" | "DELETE";

export interface Response<Body> {
  status: number;
  body: Body;
}

export async function sendTauriCommand<Args, Body>(
  path: string[],
  method: Method,
  args: Args,
): Promise<Response<Body>> {
  return await invoke("api", {
    request: {
      path,
      method,
      args,
    },
  });
}

export interface SortArgs {
  by: MetaCmpBy;
  ascend: boolean;
}

export class ApiServices {
  appConfig: AppConfigService;
  session: SessionService;
  scanner: ScannerService;
  playlist: PlaylistService;
  group: GroupService;
  entry: EntryService;

  constructor() {
    this.appConfig = instantiateAppConfigService();
    this.session = instantiateSessionService();
    this.scanner = instantiateScannerService();
    this.playlist = instantiatePlaylistService();
    this.group = instantiateGroupService();
    this.entry = instantiateEntryService();
  }
}
