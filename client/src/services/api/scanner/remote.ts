import qs from "qs";

import { sendRequest } from "../../api";
import { IndexArgs, ScannerService, basePath } from "../scanner";

export default class Remote implements ScannerService {
  async index(args: IndexArgs): Promise<string[]> {
    const resp = await sendRequest("GET", basePath, {
      searchParams: qs.stringify(args),
    });
    const body = await resp.json();
    return body as string[];
  }
}
