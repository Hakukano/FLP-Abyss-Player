import { sendRequestVoid } from "../../api";
import { Session, SessionService, basePath } from "../session";

export default class Remote implements SessionService {
  write(session: Session): Promise<void> {
    return sendRequestVoid("POST", basePath.concat(["write"]), {
      body: session,
    });
  }

  read(session: Session): Promise<void> {
    return sendRequestVoid("POST", basePath.concat(["read"]), {
      body: session,
    });
  }
}
