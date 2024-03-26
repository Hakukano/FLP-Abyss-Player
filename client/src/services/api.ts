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

interface RequestOptions {
  searchParams?: string;
  body?: any;
}

export async function sendRequest(
  method: "POST" | "GET" | "PUT" | "DELETE",
  path: string[],
  options: RequestOptions = {},
): Promise<Response> {
  const url = new URL(`/${path.join("/")}`);
  if (options.searchParams) url.search = options.searchParams;
  return await fetch(url, {
    method: method,
    body: options.body,
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
