import { Response, sendRequest } from "../../api";
import { IndexArgs, ScannerService, basePath } from "../scanner";

export default class TauriCommand implements ScannerService {
  async index(args: IndexArgs): Promise<Response<string[]>> {
    return await sendRequest(basePath, "GET", args);
  }
}
