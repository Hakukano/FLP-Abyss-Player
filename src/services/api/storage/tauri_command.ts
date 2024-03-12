import { Response, sendTauriCommand } from "../../api";
import { Storage, StorageService, basepath } from "../storage";

export default class TauriCommand implements StorageService {
  async write(storage: Storage): Promise<Response<void>> {
    return await sendTauriCommand(basepath.concat(["write"]), "POST", storage);
  }

  async read(storage: Storage): Promise<Response<void>> {
    return await sendTauriCommand(basepath.concat(["read"]), "POST", storage);
  }
}
