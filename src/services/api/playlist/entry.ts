import { Meta } from "../playlist";
import { basePath as groupPath } from "./group";

export function basePath(playlistId: string, groupId: string): string[] {
  return groupPath(playlistId).concat([groupId, "entries"]);
}

export interface Entry {
  meta: Meta;
  mime: string;
}
