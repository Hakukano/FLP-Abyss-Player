import ArrowLeftIcon from '@heroicons/react/24/outline/ArrowLeftIcon'
import ArrowRightIcon from '@heroicons/react/24/outline/ArrowRightIcon'
import CloudIcon from '@heroicons/react/24/outline/CloudIcon'
import ListBulletIcon from '@heroicons/react/24/outline/ListBulletIcon'
import { Dispatch, SetStateAction } from 'react'

export default function Component(props: {
  showPlaylistRemote: boolean,
  setShowPlaylistRemote: Dispatch<SetStateAction<boolean>>,
}) {
  return (
    <nav
      className='w-full bg-white border-gray-200 dark:bg-gray-900'
      style={{ height: '5vh' }}
    >
      <div className='h-full flex items-center justify-between p-1'>
        <ArrowLeftIcon className='h-full' onClick={() => alert('aa')} />
        <CloudIcon className='h-full' onClick={() => props.setShowPlaylistRemote(!props.showPlaylistRemote)} />
        <ListBulletIcon className='h-full' onClick={() => alert('aa')} />
        <ArrowRightIcon className='h-full' onClick={() => alert('aa')} />
      </div>
    </nav>
  )
}
