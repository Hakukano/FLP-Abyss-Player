import Dexie, { Table } from 'dexie'

import * as PlaylistLocal from '../playlist_local'

export interface PlaylistLocalTable {
  id?: number,
  data: PlaylistLocal.Create.Body[],
}

export class Db extends Dexie {
  playlist_locals!: Table<PlaylistLocalTable>

  constructor() {
    super('flp-abyss-player-client')
    this.version(1).stores({
      playlist_locals: '++id, data',
    })
  }
}

export const db = new Db()
