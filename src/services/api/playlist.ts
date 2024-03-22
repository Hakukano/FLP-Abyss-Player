import { Response } from "../api";
import TauriCommand from "./playlist/tauri_command";

export const basePath = ["playlists"];

export interface PlaylistImmutable {
  id: string;
  name: string;
}

export interface PlaylistMutable {}

export interface PlaylistBrief extends PlaylistImmutable, PlaylistMutable {}

export interface PlaylistDetails extends PlaylistImmutable, PlaylistMutable {}

export interface CreateArgs {
  name: string;
}

export interface PlaylistService {
  index(): Promise<Response<PlaylistBrief[]>>;

  create(playlistCreate: CreateArgs): Promise<Response<PlaylistDetails>>;

  show(id: string): Promise<Response<PlaylistDetails>>;

  destroy(id: string): Promise<Response<void>>;
}

export function instantiatePlaylistService(): PlaylistService {
  return import.meta.env.MODE === "test"
    ? new TauriCommand()
    : new TauriCommand();
}
