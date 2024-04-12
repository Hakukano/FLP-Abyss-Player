import { Meta } from "../../utils/meta";
import { SortArgs } from "../api";
import Remote from "./group/remote";

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

export interface ShiftArgs {
  offset: number;
}

export interface GroupService {
  index(args: IndexArgs): Promise<GroupBrief[]>;

  create(args: CreateArgs): Promise<void>;

  sort(args: SortArgs): Promise<void>;

  show(id: string): Promise<GroupDetails>;

  destroy(id: string): Promise<void>;

  shift(id: string, args: ShiftArgs): Promise<void>;
}

export function instantiateGroupService(): GroupService {
  return import.meta.env.MODE === "test" ? new Remote() : new Remote();
}
