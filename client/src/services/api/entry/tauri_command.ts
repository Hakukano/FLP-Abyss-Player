import { Response, SortArgs, sendRequest } from "../../api";
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
    return sendRequest(basePath, "GET", args);
  }

  create(args: CreateArgs): Promise<Response<EntryDetails>> {
    return sendRequest(basePath, "POST", args);
  }

  sort(args: SortArgs): Promise<Response<void>> {
    return sendRequest(basePath, "PUT", args);
  }

  show(id: string): Promise<Response<EntryDetails>> {
    return sendRequest(basePath.concat([id]), "GET", {});
  }

  destroy(id: string): Promise<Response<void>> {
    return sendRequest(basePath.concat([id]), "DELETE", {});
  }

  shift(id: string, args: ShiftArgs): Promise<Response<void>> {
    return sendRequest(basePath.concat([id]), "PUT", args);
  }
}
