import { invoke } from "@tauri-apps/api/core";

type Method = "POST" | "GET" | "PUT" | "DELETE";

export interface Response<Body> {
  code: number;
  body: Body;
}

export async function send_tauri_command<Args, Body>(
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
