import Fuse from 'fuse.js'
import { arrayMoveMutable } from 'array-move'

import { db } from '../db/dexie'
import * as Super from '../playlist_local'

export default class PlaylistLocal implements Super.PlaylistLocal {
  constructor() {
    db.playlist_locals.get(1)
      .then(table => {
        if (!table) {
          db.playlist_locals.put({
            id: 1,
            data: [],
          }, 1)
        }
      })
  }

  async create(body: Super.Create.Body): Promise<Super.Create.Response> {
    const table = await db.playlist_locals.get(1)
    if (!table) {
      throw {
        cause: 'db',
        message: 'not found',
      } as Super.Error
    }
    const id = table.data.length
    table.data.push(body)
    await db.playlist_locals.put(table, 1)
    return id
  }

  async read(query: Super.Read.Query): Promise<Super.Read.Response> {
    const table = await db.playlist_locals.get(1)
    if (!table) {
      throw {
        cause: 'db',
        message: 'not found',
      } as Super.Error
    }
    if (typeof query.id !== 'undefined') {
      const data = table.data[query.id]
      return {
        id: query.id,
        path: data.path,
        mime_type: data.mime_type,
        remote_id: data.remote_id,
      }
    } else if (typeof query.remote_id !== 'undefined') {
      const data = table.data
        .map((d, i) => {
          return {
            id: i,
            path: d.path,
            mime_type: d.mime_type,
            remote_id: d.remote_id,
          }
        })
        .find(d => d.remote_id === query.remote_id)
      if (!data) {
        throw {
          cause: 'query',
          message: 'not found',
        } as Super.Error
      }
      return data
    } else {
      throw {
        cause: 'query',
        message: 'bad request',
      } as Super.Error
    }
  }

  async delete(query: Super.Delete.Query): Promise<Super.Delete.Response> {
    const table = await db.playlist_locals.get(1)
    if (!table) {
      throw {
        cause: 'db',
        message: 'not found',
      } as Super.Error
    }
    if (typeof query.id !== 'undefined') {
      table.data.splice(query.id, 1)
    } else if (typeof query.remote_id !== 'undefined') {
      const index = table.data.findIndex(d => d.remote_id === query.remote_id)
      if (index < 0) {
        throw {
          cause: 'query',
          message: 'not found',
        } as Super.Error
      }
      table.data.splice(index, 1)
    } else {
      throw {
        cause: 'query',
        message: 'bad request',
      } as Super.Error
    }
    await db.playlist_locals.put(table, 1)
  }

  async list(query: Super.List.Query): Promise<Super.List.Response> {
    const table = await db.playlist_locals.get(1)
    if (!table) {
      throw {
        cause: 'db',
        message: 'not found',
      } as Super.Error
    }
    const list = table.data.map((d, i) => {
      return {
        id: i,
        path: d.path,
        mime_type: d.mime_type,
        remote_id: d.remote_id,
      } as Super.Data
    })
    if (query.search) {
      const searchResult = new Fuse(list, { keys: ['path'] }).search(query.search)
      const count = searchResult.length
      const data = searchResult
        .slice(query.offset, query.offset + query.length)
        .map(s => s.item)
      return { count, data }
    } else {
      const count = list.length
      const data = list.slice(query.offset, query.offset + query.length)
      return { count, data }
    }
  }

  async count(): Promise<number> {
    const table = await db.playlist_locals.get(1)
    if (!table) {
      throw {
        cause: 'db',
        message: 'not found',
      } as Super.Error
    }
    return table.data.length
  }

  async purge(): Promise<void> {
    await db.playlist_locals.put({ id: 1, data: [] }, 1)
  }

  async step(query: Super.Step.Query): Promise<Super.Step.Response> {
    const table = await db.playlist_locals.get(1)
    if (!table) {
      throw {
        cause: 'db',
        message: 'not found',
      } as Super.Error
    }
    const current = table.data.findIndex(d => d.remote_id === query.current) || 0
    const after = current + query.step
    const index = Math.min(Math.max(after, 0), table.data.length - 1)
    const data = table.data[index]
    return {
      id: index,
      path: data.path,
      mime_type: data.mime_type,
      remote_id: data.remote_id,
    }
  }

  async move(query: Super.Move.Query): Promise<Super.Move.Response> {
    const table = await db.playlist_locals.get(1)
    if (!table) {
      throw {
        cause: 'db',
        message: 'not found',
      } as Super.Error
    }
    arrayMoveMutable(table.data, query.id, Math.min(Math.max(query.id + query.step, 0), table.data.length - 1))
    await db.playlist_locals.put(table, 1)
  }
}
