import { useEffect, useState } from "react";
import { ApiServices } from "../services/api";
import { PlaylistBrief } from "../services/api/playlist";
import { ErrorModal, useError } from "./error_modal";
import { FormModal, useForm } from "./form_modal";
import { useTranslation } from "react-i18next";
import { confirm } from "@tauri-apps/plugin-dialog";
import List from "./list";
import { Stack } from "react-bootstrap";
import { useNavigate, useSearchParams } from "react-router-dom";

interface Props {
  apiServices: ApiServices;
}

export default function Playlist(props: Props) {
  const [playlists, setPlaylists] = useState<PlaylistBrief[] | null>(null);

  const { t } = useTranslation();
  const [searchParams, _] = useSearchParams();
  const navigate = useNavigate();

  const errorState = useError();
  const formState = useForm();

  const playlistId = searchParams.get("playlist_id");

  const fetchPlaylists = async () => {
    setPlaylists((await props.apiServices.playlist.index()).body);
  };

  const newPlaylist = () => {
    formState.popup({
      header: t("playlist.title"),
      rows: [
        {
          name: "name",
          type: "text",
          initial: "",
          validator: (value) =>
            value.length === 0 ? t("playlist.errors.name_too_short") : null,
          label: t("playlist.name.label"),
          placeholder: t("playlist.name.placeholder"),
        },
      ],
    });
  };

  const createPlaylist = async (values: { [key: string]: any }) => {
    await props.apiServices.playlist.create({ name: values["name"] });
    await fetchPlaylists();
  };

  const deletePlaylist = async (id: string) => {
    if (await confirm(t("playlist.delete.confirm"))) {
      await props.apiServices.playlist.destroy(id);
      await fetchPlaylists();
      if (playlistId === id) {
        navigate(`/player`);
      }
    }
  };

  const selectPlaylist = (id: string) => {
    navigate(`/player?playlist_id=${id}`);
  };

  useEffect(() => {
    fetchPlaylists().catch(errorState.popup);
  }, []);

  return (
    <Stack gap={3}>
      <ErrorModal state={errorState} />
      <FormModal state={formState} handleSubmit={createPlaylist} />
      <h2>{t("playlist.title")}</h2>
      <List
        headers={{ id: null, name: t("playlist.name.label") }}
        data={playlists || []}
        handleNew={newPlaylist}
        handleDelete={deletePlaylist}
        handleSelect={selectPlaylist}
      />
    </Stack>
  );
}
