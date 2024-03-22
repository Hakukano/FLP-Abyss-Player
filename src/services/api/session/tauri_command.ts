import { Response, sendTauriCommand } from "../../api";
import { Session, SessionService, basePath } from "../session";

export default class TauriCommand implements SessionService {
  async write(session: Session): Promise<Response<void>> {
    return await sendTauriCommand(basePath.concat(["write"]), "POST", session);
  }

  async read(session: Session): Promise<Response<void>> {
    return await sendTauriCommand(basePath.concat(["read"]), "POST", session);
  }
}
