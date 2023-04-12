import { Dispatch, SetStateAction, useEffect, useState } from 'react'
import InfiniteScroll from 'react-infinite-scroll-component'
import ChevronUpIcon from '@heroicons/react/24/outline/ChevronUpIcon'
import ChevronDownIcon from '@heroicons/react/24/outline/ChevronDownIcon'
import MinusIcon from '@heroicons/react/24/outline/MinusIcon'

import { Data, PlaylistLocal } from '@/service/playlist_local'
import { PlaylistData } from '@/pages'

const LOAD_MORE = 20

export default function Component(props: {
  playlistLocal: PlaylistLocal,
  setPlaylistData: Dispatch<SetStateAction<PlaylistData | null>>,
}) {
  const [search, setSearch] = useState('')
  const [data, setData] = useState([] as Data[])
  const [hasMore, setHasMore] = useState(true)

  const loadMeida = async (id: number) => {
    try {
      const res = await props.playlistLocal.read({ id })
      props.setPlaylistData({
        id: res.remote_id,
        path: res.path,
        mime_type: res.mime_type,
      })
    } finally {
    }
  }

  const fetchData = async (length: number) => {
    const res = await props.playlistLocal.list({
      search,
      filter: [],
      offset: 0,
      length,
    })
    setData(res.data)
    setHasMore(length < res.count)
  }

  useEffect(() => {
    fetchData(data.length + LOAD_MORE)
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
            fetchData(data.length + LOAD_MORE)
          }
        }}
      />
      <div
        id='scrollable-pl-local'
        className='w-full h-full flex overflow-auto flex-col'
      >
        <InfiniteScroll
          className='w-full flex flex-col'
          next={() => {
            fetchData(data.length + LOAD_MORE)
          }}
          hasMore={hasMore}
          dataLength={data.length}
          loader={<h4>Loading...</h4>}
          scrollableTarget='scrollable-pl-local'
        >
          {data.map((d, i) => (
            <div
              key={i}
              className='w-full grid grid-rows-1 grid-cols-12 gap-1'
            >
              <button
                className='m-0.5 border border-solid border-black text-left col-span-11'
                style={{ color: '#222' }}
                onClick={async () => {
                  await loadMeida(d.id)
                }}
              >
                {d.path}
              </button>
              <div className='col-span-1 grid grid-rows-3 grid-cols-1'>
                <ChevronUpIcon
                  className='row-span-1 text-center max-h-8'
                  style={{ color: '#222' }}
                  onClick={async () => {
                    try {
                      await props.playlistLocal.move({ id: d.id, step: -1 })
                      await fetchData(data.length)
                    } catch (_) { }
                  }}
                />
                <MinusIcon
                  className='row-span-1 text-center max-h-8'
                  style={{ color: '#222' }}
                  onClick={async () => {
                    try {
                      await props.playlistLocal.delete({ id: d.id })
                      await fetchData(data.length)
                    } catch (_) { }
                  }}
                />
                <ChevronDownIcon
                  className='row-span-1 text-center max-h-8'
                  style={{ color: '#222' }}
                  onClick={async () => {
                    try {
                      await props.playlistLocal.move({ id: d.id, step: 1 })
                      await fetchData(data.length)
                    } catch (_) { }
                  }}
                />
              </div>

            </div>
          ))}
        </InfiniteScroll>
      </div>
    </div>
  )
}
