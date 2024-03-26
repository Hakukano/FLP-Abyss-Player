import { Response, sendRequest } from "../../api";
import {
  PlaylistBrief,
  CreateArgs,
  PlaylistDetails,
  PlaylistService,
  basePath,
} from "../playlist";

export default class TauriCommand implements PlaylistService {
  index(): Promise<Response<PlaylistBrief[]>> {
    return sendRequest(basePath, "GET", {});
  }

  create(playlistCreate: CreateArgs): Promise<Response<PlaylistDetails>> {
    return sendRequest(basePath, "POST", playlistCreate);
  }

  show(id: string): Promise<Response<PlaylistDetails>> {
    return sendRequest(basePath.concat([id]), "GET", {});
  }

  destroy(id: string): Promise<Response<void>> {
    return sendRequest(basePath.concat([id]), "DELETE", {});
  }
}
