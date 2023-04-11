import ArrowLeftIcon from '@heroicons/react/24/outline/ArrowLeftIcon'
import ArrowRightIcon from '@heroicons/react/24/outline/ArrowRightIcon'
import CloudIcon from '@heroicons/react/24/outline/CloudIcon'
import ListBulletIcon from '@heroicons/react/24/outline/ListBulletIcon'
import TrashIcon from '@heroicons/react/24/outline/TrashIcon'
import { Dispatch, SetStateAction } from 'react'

import { PlaylistLocal } from '@/service/playlist_local'
import { PlaylistData } from '@/pages'

export default function Component(props: {
  playlistLocal: PlaylistLocal
  playlistData: PlaylistData | null,
  setPlaylistData: Dispatch<SetStateAction<PlaylistData | null>>,
  showPlaylistRemote: boolean,
  setShowPlaylistRemote: Dispatch<SetStateAction<boolean>>,
  showPlaylistLocal: boolean,
  setShowPlaylistLocal: Dispatch<SetStateAction<boolean>>,
}) {
  return (
    <nav
      className='w-full bg-white border-gray-200 dark:bg-gray-900'
      style={{ height: '5vh' }}
    >
      <div className='h-full flex items-center justify-between p-1'>
        <ArrowLeftIcon className='h-full' onClick={async () => {
          const data = props.playlistData
          if (data) {
            const next = await props.playlistLocal.step({ current: data.id, step: -1 })
            props.setPlaylistData({
              id: next.remote_id,
              path: next.path,
              mime_type: next.mime_type,
            })
          }
        }} />
        <CloudIcon className='h-full' onClick={() => props.setShowPlaylistRemote(!props.showPlaylistRemote)} />
        <ListBulletIcon className='h-full' onClick={() => props.setShowPlaylistLocal(!props.showPlaylistLocal)} />
        <TrashIcon className='h-full' onClick={() => {
          if (confirm("Purge local playlist?") === true) {
            props.playlistLocal.purge()
          }
        }} />
        <ArrowRightIcon className='h-full' onClick={async () => {
          const data = props.playlistData
          if (data) {
            const next = await props.playlistLocal.step({ current: data.id, step: 1 })
            props.setPlaylistData({
              id: next.remote_id,
              path: next.path,
              mime_type: next.mime_type,
            })
          }
        }} />
      </div>
    </nav>
  )
}
