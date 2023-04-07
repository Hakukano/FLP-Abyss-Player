import { ListQuery, ListResponse } from "@/service"

export interface Error {
  cause: string,
  message: string,
}

export interface Data {
  id: number,
  path: string,
  mime_type: string,
  remote_id: number,
}

export namespace Create {
  export interface Body {
    path: string,
    mime_type: string,
    remote_id: number,
  }
  export type Response = number
}

export namespace Read {
  export interface Query {
    id?: number,
    remote_id?: number,
  }
  export type Response = Data
}

export namespace Delete {
  export interface Query {
    id?: number,
    remote_id?: number,
  }
  export type Response = Data
}

export namespace List {
  export type Query = ListQuery
  export type Response = ListResponse<Data>
}

export interface PlaylistLocal {
  create(body: Create.Body): Promise<Create.Response>
  read(query: Read.Query): Promise<Read.Response>
  delete(query: Delete.Query): Promise<Delete.Response>
  list(query: List.Query): Promise<List.Response>
  count(): Promise<number>
  purge(): Promise<void>
}
