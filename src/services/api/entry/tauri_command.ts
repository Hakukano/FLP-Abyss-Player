import { Response, SortArgs, sendTauriCommand } from "../../api";
import {
  EntryBrief,
  CreateArgs,
  EntryDetails,
  IndexArgs,
  EntryService,
  basePath,
  ShiftArgs,
} from "../entry";

export default class TauriCommand implements EntryService {
  index(args: IndexArgs): Promise<Response<EntryBrief[]>> {
    return sendTauriCommand(basePath, "GET", args);
  }

  create(args: CreateArgs): Promise<Response<EntryDetails>> {
    return sendTauriCommand(basePath, "POST", args);
  }

  sort(args: SortArgs): Promise<Response<void>> {
    return sendTauriCommand(basePath, "PUT", args);
  }

  show(id: string): Promise<Response<EntryDetails>> {
    return sendTauriCommand(basePath.concat([id]), "GET", {});
  }

  destroy(id: string): Promise<Response<void>> {
    return sendTauriCommand(basePath.concat([id]), "DELETE", {});
  }

  shift(id: string, args: ShiftArgs): Promise<Response<void>> {
    return sendTauriCommand(basePath.concat([id]), "PUT", args);
  }
}
