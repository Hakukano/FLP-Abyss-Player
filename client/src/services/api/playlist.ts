import Remote from "./playlist/remote";

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
  index(): Promise<PlaylistBrief[]>;

  create(playlistCreate: CreateArgs): Promise<void>;

  show(id: string): Promise<PlaylistDetails>;

  destroy(id: string): Promise<void>;
}

export function instantiatePlaylistService(): PlaylistService {
  return import.meta.env.MODE === "test" ? new Remote() : new Remote();
}
