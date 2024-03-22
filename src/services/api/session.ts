import { Response } from "../api";
import TauriCommand from "./session/tauri_command";

export const basePath = ["session"];

export interface Session {
  path: string;
}

export interface SessionService {
  write(session: Session): Promise<Response<void>>;
  read(session: Session): Promise<Response<void>>;
}

export function instantiateSessionService(): SessionService {
  return import.meta.env.MODE === "test"
    ? new TauriCommand()
    : new TauriCommand();
}
