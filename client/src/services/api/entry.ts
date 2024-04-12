import { Meta } from "../../utils/meta";
import { SortArgs } from "../api";
import Remote from "./entry/remote";

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
  index(args: IndexArgs): Promise<EntryBrief[]>;

  create(args: CreateArgs): Promise<void>;

  sort(args: SortArgs): Promise<void>;

  show(id: string): Promise<EntryDetails>;

  destroy(id: string): Promise<void>;

  shift(id: string, args: ShiftArgs): Promise<void>;
}

export function instantiateEntryService(): EntryService {
  return import.meta.env.MODE === "test" ? new Remote() : new Remote();
}
