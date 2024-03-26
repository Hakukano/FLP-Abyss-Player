import fetch, { Response } from "node-fetch";

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

export async function sendRequest(
  path: string[],
  searchParams: { [key: string]: string },
  method: string,
  body?: any,
): Promise<Response> {
  const url = new URL(`/${path.join("/")}`);
  Object.entries(searchParams).forEach(([k, v]) =>
    url.searchParams.append(k, v),
  );
  return await fetch(url, {
    method,
    body,
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
