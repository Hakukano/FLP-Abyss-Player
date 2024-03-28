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
  query?: string;
  body?: any;
}

export async function sendRequest(
  method: "POST" | "GET" | "PUT" | "DELETE",
  path: string[],
  options: RequestOptions = {},
): Promise<Response> {
  let url = `/api/${path.join("/")}`;
  if (options.query) url = url + "?" + options.query;
  const resp = await fetch(url, {
    headers: { "Content-Type": "application/json" },
    method: method,
    body: options.body ? JSON.stringify(options.body) : undefined,
  });
  if (resp.status >= 400) {
    throw { status: resp.status, body: await resp.text() };
  }
  return resp;
}

export async function sendRequestVoid(
  method: "POST" | "GET" | "PUT" | "DELETE",
  path: string[],
  options: RequestOptions = {},
): Promise<void> {
  await sendRequest(method, path, options);
}

export async function sendRequestJson<T>(
  method: "POST" | "GET" | "PUT" | "DELETE",
  path: string[],
  options: RequestOptions = {},
): Promise<T> {
  const resp = await sendRequest(method, path, options);
  const body = await resp.json();
  return body as T;
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
