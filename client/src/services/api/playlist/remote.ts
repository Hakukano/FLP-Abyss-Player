import { sendRequestJson, sendRequestVoid } from "../../api";
import {
  PlaylistBrief,
  CreateArgs,
  PlaylistDetails,
  PlaylistService,
  basePath,
} from "../playlist";

export default class Remote implements PlaylistService {
  index(): Promise<PlaylistBrief[]> {
    return sendRequestJson("GET", basePath);
  }

  create(playlistCreate: CreateArgs): Promise<void> {
    return sendRequestJson("POST", basePath, { body: playlistCreate });
  }

  show(id: string): Promise<PlaylistDetails> {
    return sendRequestJson("GET", basePath.concat([id]));
  }

  destroy(id: string): Promise<void> {
    return sendRequestVoid("DELETE", basePath.concat([id]));
  }
}
