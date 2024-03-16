import { Response } from "../api";
import TauriCommand from "./scanner/tauri_command";

export const basePath = ["scanner"];

export interface IndexArgs {
  root_path: string;
  allowed_mimes: string[];
}

export interface ScannerService {
  index(args: IndexArgs): Promise<Response<string[]>>;
}

export function instantiateScannerService(): ScannerService {
  return import.meta.env.MODE === "test"
    ? new TauriCommand()
    : new TauriCommand();
}
