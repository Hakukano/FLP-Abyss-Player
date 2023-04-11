import { Dispatch, SetStateAction, useEffect, useState } from 'react'
import DataTable, { TableColumn } from 'react-data-table-component'
import PlusIcon from '@heroicons/react/24/outline/PlusIcon'
import MinusIcon from '@heroicons/react/24/outline/MinusIcon'

import { Data, PlaylistRemote } from '@/service/playlist_remote'
import { PlaylistLocal } from '@/service/playlist_local'
import { PlaylistData } from '@/pages'

export default function Component(props: {
  playlistRemote: PlaylistRemote,
  setPlaylistData: Dispatch<SetStateAction<PlaylistData | null>>,
  playlistLocal: PlaylistLocal,
}) {
  const [search, setSearch] = useState('')
  const [page, setPage] = useState(1)
  const [length, setLength] = useState(5)
  const [data, setData] = useState([] as Data[])
  const [count, setCount] = useState(0)
  const [loading, setLoading] = useState(false)

  const [inLocalMap, setInLocalMap] = useState(new Map<number, number>)

  const loadMeida = async (id: number) => {
    setLoading(true)
    try {
      const res = await props.playlistRemote.read({ id })
      props.setPlaylistData(res)
    } finally {
      setLoading(false)
    }
  }

  const fetchData = async (page: number, length: number) => {
    setLoading(true)
    try {
      const res = await props.playlistRemote.list({
        search,
        filter: [],
        offset: (page - 1) * length,
        length,
      })
      setData(res.data)
      setCount(res.count)
      const map = new Map<number, number>
      for (const d of res.data) {
        try {
          const local = await props.playlistLocal.read({ remote_id: d.id })
          map.set(d.id, local.id)
        } catch (_) {
        }
      }
      setInLocalMap(map)
    } finally {
      setLoading(false)
    }
  }

  const columns: TableColumn<Data>[] = [
    {
      omit: true,
      name: 'ID',
      selector: row => row.id,
    },
    {
      name: 'Full Path',
      selector: row => row.path,
      button: true,
      width: '95%',
      cell: row => <button
        onClick={async () => {
          await loadMeida(row.id)
        }}
      >
        {row.path}
      </button>
    },
    {
      omit: true,
      name: 'MIME Type',
      selector: row => row.mime_type,
    },
    {
      name: '',
      button: true,
      width: '5%',
      cell: row => inLocalMap.has(row.id)
        ? <MinusIcon
          className='w-full h-full'
          onClick={async () => {
            try {
              await props.playlistLocal.delete({ remote_id: row.id })
              await fetchData(page, length)
            } catch (_) { }
          }}
        />
        : <PlusIcon
          className='w-full h-full'
          onClick={async () => {
            try {
              await props.playlistLocal.create({
                path: row.path,
                mime_type: row.mime_type,
                remote_id: row.id,
              })
              await fetchData(page, length)
            } catch (_) { }
          }}
        />
    },
  ]

  const handlePageChange = (page: number) => {
    setPage(page)
    fetchData(page, length)
  }

  const handlePerRowsChange = (newPerPage: number, page: number) => {
    setLength(newPerPage)
    setPage(page)
    fetchData(page, newPerPage)
  }

  useEffect(() => {
    fetchData(page, length)
  }, []) // eslint-disable-line react-hooks/exhaustive-deps

  return (
    <div className='w-full h-full flex flex-col space-y-1'>
      <input
        className='w-full'
        type='text'
        placeholder='Press ENTER to search...'
        value={search}
        style={{
          color: '#222',
        }}
        onChange={event => setSearch(event.target.value)}
        onKeyDown={event => {
          if (event.key === 'Enter') {
            fetchData(page, length)
          }
        }}
      />
      <div
        id='scrollable-pl-remote'
        className='h-full flex overflow-auto flex-col'
      >
        <div
          className='w-full h-auto'
        >
          <DataTable
            columns={columns}
            data={data}
            progressPending={loading}
            pagination
            paginationServer
            paginationPerPage={length}
            paginationTotalRows={count}
            paginationRowsPerPageOptions={[5, 10, 15, 20]}
            onChangeRowsPerPage={handlePerRowsChange}
            onChangePage={handlePageChange}
          />
        </div>
      </div>
    </div>
  )
}
