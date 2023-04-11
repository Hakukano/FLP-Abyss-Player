import { useState } from 'react'
import ReactPlayer from 'react-player'
import XMarkIcon from '@heroicons/react/24/outline/XMarkIcon'

import PlaylistRemote from '@/components/playlist_remote'
import PlaylistLocal from '@/components/playlist_local'
import Toolbar from '@/components/toolbar'
import ServicePlaylistRemote from '@/service/playlist_remote/fetch'
import ServicePlaylistLocal from '@/service/playlist_local/dexie'

export interface PlaylistData {
  id: number,
  path: string,
  mime_type: string,
}

export default function Page() {
  const playlistRemote = new ServicePlaylistRemote
  const playlistLocal = new ServicePlaylistLocal

  const [playlistData, setPlaylistData] = useState(null as PlaylistData | null)

  const [showPlaylistRemote, setShowPlaylistRemote] = useState(false)
  const [showPlaylistLocal, setShowPlaylistLocal] = useState(false)

  return (
    <main className='flex flex-col w-full h-full items-center justify-between p-2 space-y-2'>
      <Toolbar
        playlistLocal={playlistLocal}
        playlistData={playlistData}
        setPlaylistData={setPlaylistData}
        showPlaylistRemote={showPlaylistRemote}
        setShowPlaylistRemote={setShowPlaylistRemote}
        showPlaylistLocal={showPlaylistLocal}
        setShowPlaylistLocal={setShowPlaylistLocal}
      />
      <div className='w-full h-full'>
        {
          playlistData
            ? playlistData.mime_type.startsWith('image/')
              ? <div
                className='w-full text-center'
                style={{ height: '88vh' }}
              >
                <img
                  className='w-full h-full align-middle'
                  style={{
                    objectFit: 'contain',
                    overflow: 'hidden',
                  }}
                  src={`/playlists/${playlistData.id}/stream`}
                  alt={playlistData.path}
                />
              </div>
              : <ReactPlayer
                width='100%'
                height='88vh'
                url={`/playlists/${playlistData.id}/stream`}
                controls={true}
              />
            : <></>
        }
      </div>
      {
        showPlaylistRemote
          ?
          <>
            <div
              className="absolute justify-center items-center fixed inset-0 z-50 outline-none focus:outline-none"
              style={{
                width: '90%',
                height: '95vh',
                left: '5%',
              }}
            >
              <div className="relative w-full h-full">
                {/*content*/}
                <div className="h-full border-0 rounded-lg shadow-lg relative flex flex-col w-full bg-white outline-none focus:outline-none">
                  {/*header*/}
                  <div className="flex items-start justify-between p-1 border-b border-solid border-slate-200 rounded-t">
                    <h3 className="text-2xl text-black font-semibold">
                      Remote Playlist
                    </h3>
                    <XMarkIcon
                      className="h-8 p-1 ml-auto bg-transparent border-0 text-black float-right text-3xl leading-none font-semibold outline-none focus:outline-none"
                      onClick={() => setShowPlaylistRemote(false)}
                    />
                  </div>
                  {/*body*/}
                  <div className="relative p-2 w-full flex min-h-0">
                    <PlaylistRemote
                      playlistRemote={playlistRemote}
                      setPlaylistData={setPlaylistData}
                      playlistLocal={playlistLocal}
                    />
                  </div>
                </div>
              </div>
            </div>
            <div className="opacity-25 fixed inset-0 z-40 bg-black"></div>
          </>
          : <></>
      }
      {
        showPlaylistLocal
          ?
          <>
            <div
              className="absolute justify-center items-center fixed inset-0 z-50 outline-none focus:outline-none"
              style={{
                width: '90%',
                height: '95vh',
                left: '5%',
              }}
            >
              <div className="relative w-full h-full">
                {/*content*/}
                <div className="h-full border-0 rounded-lg shadow-lg relative flex flex-col w-full bg-white outline-none focus:outline-none">
                  {/*header*/}
                  <div className="flex items-start justify-between p-1 border-b border-solid border-slate-200 rounded-t">
                    <h3 className="text-2xl text-black font-semibold">
                      Local Playlist
                    </h3>
                    <XMarkIcon
                      className="h-8 p-1 ml-auto bg-transparent border-0 text-black float-right text-3xl leading-none font-semibold outline-none focus:outline-none"
                      onClick={() => setShowPlaylistLocal(false)}
                    />
                  </div>
                  {/*body*/}
                  <div className="relative p-2 w-full flex min-h-0">
                    <PlaylistLocal
                      playlistLocal={playlistLocal}
                      setPlaylistData={setPlaylistData}
                    />
                  </div>
                </div>
              </div>
            </div>
            <div className="opacity-25 fixed inset-0 z-40 bg-black"></div>
          </>
          : <></>
      }
    </main>
  )
}
