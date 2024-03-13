import { useEffect, useState } from "react";
import { ApiServices } from "../services/api";
import { PlaylistBrief } from "../services/api/playlist";
import { useError } from "./error";

interface Props {
  apiServices: ApiServices;
}

export default function Playlist(props: Props) {
  const [playlists, setPlaylists] = useState<PlaylistBrief[] | null>(null);

  const errorState = useError();

  const fetchPlaylist = async () => {
    const resp = (await props.apiServices.playlist.index()).body;
  };

  const addPlaylist = async () => {
    let name = "";
    await props.apiServices.playlist.create({ name });
    await fetchPlaylist();
  };

  useEffect(() => {
    fetchPlaylist().catch(errorState.handleError);
  }, []);

  return <></>;
}
