import qs from "qs";

import { sendRequestJson } from "../../api";
import { IndexArgs, ScannerService, basePath } from "../scanner";

export default class Remote implements ScannerService {
  index(args: IndexArgs): Promise<string[]> {
    return sendRequestJson("GET", basePath, {
      query: qs.stringify(args),
    });
  }
}
