import Remote from "./scanner/remote";

export const basePath = ["scanner"];

export interface IndexArgs {
  root_path: string;
  allowed_mimes: string;
}

export interface ScannerService {
  index(args: IndexArgs): Promise<string[]>;
}

export function instantiateScannerService(): ScannerService {
  return import.meta.env.MODE === "test" ? new Remote() : new Remote();
}
