import { Data, PlaylistRemote } from '@/service/playlist_remote'
import { Dispatch, SetStateAction, useEffect, useState } from 'react'
import DataTable, { TableColumn } from 'react-data-table-component'

export default function Component(props: {
  playlistRemote: PlaylistRemote,
  setPlaylistData: Dispatch<SetStateAction<Data | null>>,
}) {
  const [search, setSearch] = useState('')
  const [length, setLength] = useState(5)
  const [data, setData] = useState([] as Data[])
  const [count, setCount] = useState(0)
  const [loading, setLoading] = useState(false)

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
      width: '100%',
      cell: row => <button
        onClick={async () => {
          setLoading(true)
          try {
            const res = await props.playlistRemote.read({ id: row.id })
            props.setPlaylistData(res)
          } finally {
            setLoading(false)
          }
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
  ]

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
    } finally {
      setLoading(false)
    }
  }

  const handlePageChange = (page: number) => {
    fetchData(page, length)
  }

  const handlePerRowsChange = (newPerPage: number, page: number) => {
    setLength(newPerPage)
    fetchData(page, newPerPage)
  }

  useEffect(() => {
    fetchData(1, length)
  }, []) // eslint-disable-line react-hooks/exhaustive-deps

  return (
    <div className='flex flex-col space-y-1'>
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
            fetchData(1, length)
          }
        }}
      />
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
  )
}
