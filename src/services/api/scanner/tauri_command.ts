import { Response, sendTauriCommand } from "../../api";
import { IndexArgs, ScannerService, basePath } from "../scanner";

export default class TauriCommand implements ScannerService {
  async index(args: IndexArgs): Promise<Response<string[]>> {
    return await sendTauriCommand(basePath, "GET", args);
  }
}
