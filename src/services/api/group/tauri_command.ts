import { Response, SortArgs, sendTauriCommand } from "../../api";
import {
  GroupBrief,
  CreateArgs,
  GroupDetails,
  IndexArgs,
  GroupService,
  basePath,
} from "../group";

export default class TauriCommand implements GroupService {
  index(args: IndexArgs): Promise<Response<GroupBrief[]>> {
    return sendTauriCommand(basePath, "GET", args);
  }

  create(args: CreateArgs): Promise<Response<GroupDetails>> {
    return sendTauriCommand(basePath, "POST", args);
  }

  sort(args: SortArgs): Promise<Response<void>> {
    return sendTauriCommand(basePath, "PUT", args);
  }

  show(id: string): Promise<Response<GroupDetails>> {
    return sendTauriCommand(basePath.concat([id]), "GET", {});
  }

  destroy(id: string): Promise<Response<void>> {
    return sendTauriCommand(basePath.concat([id]), "DELETE", {});
  }
}
