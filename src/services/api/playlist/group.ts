import { Meta, basePath as playlistPath } from "../playlist";
import { Entry } from "./entry";

export function basePath(playlistId: string): string[] {
  return playlistPath.concat([playlistId, "groups"]);
}

export interface Group {
  meta: Meta;
  entries: Entry[];
}

export interface GroupImmutable {}

export interface GroupMutable {
  groups: Group[];
}

export interface GroupBrief extends GroupImmutable, GroupMutable {}

export interface GroupDetails extends GroupImmutable, GroupMutable {}
