import { sendRequest } from "../../api";
import { Session, SessionService, basePath } from "../session";

export default class Remote implements SessionService {
  async write(session: Session): Promise<void> {
    await sendRequest("POST", basePath.concat(["write"]), {
      body: session,
    });
  }

  async read(session: Session): Promise<void> {
    await sendRequest("POST", basePath.concat(["read"]), {
      body: session,
    });
  }
}
