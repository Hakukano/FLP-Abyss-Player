import { Response, sendTauriCommand } from "../../api";
import {
  PlaylistBrief,
  CreateArgs,
  PlaylistDetails,
  PlaylistService,
  basePath,
} from "../playlist";

export default class TauriCommand implements PlaylistService {
  index(): Promise<Response<PlaylistBrief[]>> {
    return sendTauriCommand(basePath, "GET", {});
  }

  create(playlistCreate: CreateArgs): Promise<Response<PlaylistDetails>> {
    return sendTauriCommand(basePath, "POST", playlistCreate);
  }

  show(id: string): Promise<Response<PlaylistDetails>> {
    return sendTauriCommand(basePath.concat([id]), "GET", {});
  }

  destroy(id: string): Promise<Response<void>> {
    return sendTauriCommand(basePath.concat([id]), "DELETE", {});
  }
}
