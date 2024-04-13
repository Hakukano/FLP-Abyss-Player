import qs from "qs";

import { SortArgs, sendRequestVoid, sendRequestJson } from "../../api";
import {
  EntryBrief,
  CreateArgs,
  EntryDetails,
  IndexArgs,
  EntryService,
  basePath,
  ShiftArgs,
} from "../entry";

export default class Remote implements EntryService {
  index(args: IndexArgs): Promise<EntryBrief[]> {
    return sendRequestJson("GET", basePath, {
      query: qs.stringify(args),
    });
  }

  create(args: CreateArgs): Promise<EntryDetails> {
    return sendRequestJson("POST", basePath, { body: args });
  }

  sort(args: SortArgs): Promise<void> {
    return sendRequestVoid("PUT", basePath, { body: args });
  }

  show(id: string): Promise<EntryDetails> {
    return sendRequestJson("GET", basePath.concat([id]));
  }

  destroy(id: string): Promise<void> {
    return sendRequestVoid("DELETE", basePath.concat([id]));
  }

  shift(id: string, args: ShiftArgs): Promise<void> {
    return sendRequestVoid("PUT", basePath.concat([id]), { body: args });
  }
}
