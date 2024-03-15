import { Meta } from "../../utils/meta";
import { Response, SortArgs } from "../api";
import TauriCommand from "./entry/tauri_command";

export const basePath = ["entries"];

export interface EntryImmutable {
  id: string;
  mime: string;
  meta: Meta;
  group_id: string;
}

export interface EntryMutable {}

export interface EntryBrief extends EntryImmutable, EntryMutable {}

export interface EntryDetails extends EntryImmutable, EntryMutable {}

export interface IndexArgs {
  group_id: string | null;
}

export interface CreateArgs {
  group_id: string;
  path: string;
}

export interface ShiftArgs {
  offset: number;
}

export interface EntryService {
  index(args: IndexArgs): Promise<Response<EntryBrief[]>>;

  create(args: CreateArgs): Promise<Response<EntryDetails>>;

  sort(args: SortArgs): Promise<Response<void>>;

  show(id: string): Promise<Response<EntryDetails>>;

  destroy(id: string): Promise<Response<void>>;

  shift(id: string, args: ShiftArgs): Promise<Response<void>>;
}

export function instantiateEntryService(): EntryService {
  return import.meta.env.MODE === "test"
    ? new TauriCommand()
    : new TauriCommand();
}
