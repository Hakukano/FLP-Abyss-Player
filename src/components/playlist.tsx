import { useEffect, useState } from "react";
import { ApiServices } from "../services/api";
import { PlaylistBrief } from "../services/api/playlist";
import { useError } from "./error_modal";
import { useForm } from "./form_modal";
import { useTranslation } from "react-i18next";
import { confirm } from "@tauri-apps/plugin-dialog";
import List from "./list";

interface Props {
  apiServices: ApiServices;
}

export default function Playlist(props: Props) {
  const { t } = useTranslation();

  const [playlists, setPlaylists] = useState<PlaylistBrief[] | null>(null);

  const errorState = useError();
  const formState = useForm();

  const fetchPlaylist = async () => {
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
    await fetchPlaylist();
  };

  const deletePlaylist = async (id: string) => {
    if (await confirm(t("playlist.delete.confirm"))) {
      await props.apiServices.playlist.destroy(id);
      await fetchPlaylist();
    }
  };

  useEffect(() => {
    fetchPlaylist().catch(errorState.handleError);
  }, []);

  return (
    <List
      createFormState={formState}
      headers={[t("playlist.name.label")]}
      data={playlists || []}
      handleNew={newPlaylist}
      handleCreate={createPlaylist}
      handleDelete={deletePlaylist}
    />
  );
}
