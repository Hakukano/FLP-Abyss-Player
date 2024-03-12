import { Meta } from "../../utils/meta";
import { Response, SortArgs } from "../api";
import TauriCommand from "./group/tauri_command";

export const basePath = ["groups"];

export interface GroupImmutable {
  id: string;
  meta: Meta;
  playlist_id: string;
}

export interface GroupMutable {}

export interface GroupBrief extends GroupImmutable, GroupMutable {}

export interface GroupDetails extends GroupImmutable, GroupMutable {}

export interface IndexArgs {
  playlist_id: string | null;
}

export interface CreateArgs {
  playlist_id: string;
  path: string;
}

export interface GroupService {
  index(args: IndexArgs): Promise<Response<GroupBrief[]>>;

  create(args: CreateArgs): Promise<Response<GroupDetails>>;

  sort(args: SortArgs): Promise<Response<void>>;

  show(id: string): Promise<Response<GroupDetails>>;

  destroy(id: string): Promise<Response<void>>;
}

export function instantiateGroupService(): GroupService {
  return import.meta.env.MODE === "test"
    ? new TauriCommand()
    : new TauriCommand();
}
