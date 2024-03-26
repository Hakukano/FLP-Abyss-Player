import { Response, SortArgs, sendRequest } from "../../api";
import {
  GroupBrief,
  CreateArgs,
  GroupDetails,
  IndexArgs,
  GroupService,
  basePath,
  ShiftArgs,
} from "../group";

export default class TauriCommand implements GroupService {
  index(args: IndexArgs): Promise<Response<GroupBrief[]>> {
    return sendRequest(basePath, "GET", args);
  }

  create(args: CreateArgs): Promise<Response<GroupDetails>> {
    return sendRequest(basePath, "POST", args);
  }

  sort(args: SortArgs): Promise<Response<void>> {
    return sendRequest(basePath, "PUT", args);
  }

  show(id: string): Promise<Response<GroupDetails>> {
    return sendRequest(basePath.concat([id]), "GET", {});
  }

  destroy(id: string): Promise<Response<void>> {
    return sendRequest(basePath.concat([id]), "DELETE", {});
  }

  shift(id: string, args: ShiftArgs): Promise<Response<void>> {
    return sendRequest(basePath.concat([id]), "PUT", args);
  }
}
