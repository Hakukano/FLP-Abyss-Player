import ReactPlayer from 'react-player'

export default function Page() {
  return (
    <main className='flex min-h-screen flex-col items-center justify-between p-24'>
      <ReactPlayer
        url='/playlists/0/stream'
        controls={true}
      />
    </main>
  )
}
