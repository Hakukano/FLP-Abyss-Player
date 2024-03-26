import { Response, sendRequest } from "../../api";
import { Session, SessionService, basePath } from "../session";

export default class TauriCommand implements SessionService {
  async write(session: Session): Promise<Response<void>> {
    return await sendRequest(basePath.concat(["write"]), "POST", session);
  }

  async read(session: Session): Promise<Response<void>> {
    return await sendRequest(basePath.concat(["read"]), "POST", session);
  }
}
