import { db } from '../db/dexie'
import * as Super from '../playlist_local'

export default class PlaylistLocal implements Super.PlaylistLocal {
  constructor() {
    db.playlist_locals.add({
      id: 0,
      data: [],
    })
  }

  async create(body: Super.Create.Body): Promise<Super.Create.Response> {
    const table = await db.playlist_locals.get(0)
    if (!table) {
      throw {
        cause: 'db',
        message: 'not found',
      } as Super.Error
    }
    const id = table.data.length
    table.data.push(body)
    await db.playlist_locals.update(0, table)
    return id
  }

  async read(query: Super.Read.Query): Promise<Super.Read.Response> {

  }

  async delete(query: Super.Delete.Query): Promise<Super.Delete.Response> {

  }

  async list(query: Super.List.Query): Promise<Super.List.Response> {

  }

  async count(): Promise<number> {

  }

  async purge(): Promise<void> {

  }
}
