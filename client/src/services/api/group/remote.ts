import qs from "qs";

import { SortArgs, sendRequestJson, sendRequestVoid } from "../../api";
import {
  GroupBrief,
  CreateArgs,
  GroupDetails,
  IndexArgs,
  GroupService,
  basePath,
  ShiftArgs,
} from "../group";

export default class Remote implements GroupService {
  index(args: IndexArgs): Promise<GroupBrief[]> {
    return sendRequestJson("GET", basePath, {
      query: qs.stringify(args),
    });
  }

  create(args: CreateArgs): Promise<void> {
    return sendRequestVoid("POST", basePath, { body: args });
  }

  sort(args: SortArgs): Promise<void> {
    return sendRequestVoid("PUT", basePath, { body: args });
  }

  show(id: string): Promise<GroupDetails> {
    return sendRequestJson("GET", basePath.concat([id]));
  }

  destroy(id: string): Promise<void> {
    return sendRequestVoid("DELETE", basePath.concat([id]));
  }

  shift(id: string, args: ShiftArgs): Promise<void> {
    return sendRequestVoid("PUT", basePath.concat([id]), { body: args });
  }
}
