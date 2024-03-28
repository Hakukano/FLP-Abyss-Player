import Remote from "./session/remote";

export const basePath = ["session"];

export interface Session {
  path: string;
}

export interface SessionService {
  write(session: Session): Promise<void>;
  read(session: Session): Promise<void>;
}

export function instantiateSessionService(): SessionService {
  return import.meta.env.MODE === "test" ? new Remote() : new Remote();
}
