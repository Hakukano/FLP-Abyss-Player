import { Response } from "../api";
import TauriCommand from "./storage/tauri_command";

export const basepath = ["storage"];

export interface Storage {
  path: string;
}

export interface StorageService {
  write(storage: Storage): Promise<Response<void>>;
  read(storage: Storage): Promise<Response<void>>;
}

export function instantiateStorageService(): StorageService {
  return import.meta.env.MODE === "test"
    ? new TauriCommand()
    : new TauriCommand();
}
