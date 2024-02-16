import { invoke } from "@tauri-apps/api/core";

type Method = "POST" | "GET" | "PUT" | "DELETE";

export interface Response<Body> {
  status: number;
  body: Body;
}

export async function sendTauriCommand<Args, Body>(
  path: string[],
  method: Method,
  args: Args,
): Promise<Response<Body>> {
  return await invoke("api", {
    request: {
      path,
      method,
      args,
    },
  });
}
