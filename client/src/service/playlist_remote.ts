import { ListQuery, ListResponse } from "@/service"

export interface Error {
  status: number,
  message: string,
}

export interface Data {
  id: number,
  path: string,
  mime_type: string,
}

export namespace Read {
  export interface Path {
    id: number,
  }
  export type Response = Data
}

export namespace List {
  export type Query = ListQuery
  export type Response = ListResponse<Data>
}

export interface PlaylistRemote{
  read(path: Read.Path): Promise<Read.Response>
  list(query: List.Query): Promise<List.Response>
}
