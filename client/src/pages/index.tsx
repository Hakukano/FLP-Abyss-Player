import ReactPlayer from 'react-player'

import Playlists from '@/components/playlists'
import Playlist from '@/service/playlist/fetch'
import { useState } from 'react'
import { Data } from '@/service/playlist'

export default function Page() {
  const playlist = new Playlist

  const [playlistData, setPlaylistData] = useState(null as Data | null)

  return (
    <main className='flex flex-col w-full h-full items-center justify-between p-8'>
      <div className='grid grid-rows-1 grid-cols-3 grid-flow-col gap-4'>
        <div className='col-span-2'>
          <Playlists
            playlist={playlist}
            setPlaylistData={setPlaylistData}
          />
        </div>
        <div className='col-span-1'>
          {
            playlistData
              ? playlistData.mime_type.startsWith('image/')
                ? <></>
                : <ReactPlayer
                  width='100%'
                  height='100%'
                  url={`/playlists/${playlistData.id}/stream`}
                  controls={true}
                />
              : <></>
          }
        </div>
      </div>
    </main>
  )
}
