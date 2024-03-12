import { Response, sendTauriCommand } from "../../api";
import { Scanner, ScannerService, basePath } from "../scanner";

export default class TauriCommand implements ScannerService {
  async index(scanner: Scanner): Promise<Response<string[]>> {
    return await sendTauriCommand(basePath, "GET", scanner);
  }
}
