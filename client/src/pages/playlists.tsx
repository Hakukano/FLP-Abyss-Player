import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { confirm } from "@tauri-apps/plugin-dialog";
import { Container, Stack } from "react-bootstrap";
import { useNavigate } from "react-router-dom";

import { ApiServices } from "../services/api";
import { PlaylistBrief } from "../services/api/playlist";
import { ErrorModal, useError } from "../components/error_modal";
import { FormModal, useForm } from "../components/form_modal";
import List from "../components/list";
import { MenuButton } from "react-bootstrap-icons";
import { MenuModal, useMenu } from "../components/menu_modal";

interface Props {
  apiServices: ApiServices;
}

export default function Playlists(props: Props) {
  const [playlists, setPlaylists] = useState<PlaylistBrief[] | null>(null);

  const { t } = useTranslation();
  const navigate = useNavigate();

  const errorState = useError();
  const menuState = useMenu();
  const formState = useForm();

  const popupMenu = () => {
    menuState.popup();
  };

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
    }
  };

  const selectPlaylist = (id: string) => {
    navigate(`/player?playlist_id=${id}`);
  };

  useEffect(() => {
    fetchPlaylists().catch(errorState.popup);
  }, []);

  return (
    <Container fluid className="vh-100 d-flex p-3">
      <Stack gap={3}>
        <ErrorModal state={errorState} />
        <MenuModal state={menuState} apiServices={props.apiServices} />
        <FormModal state={formState} handleSubmit={createPlaylist} />
        <Stack direction="horizontal" gap={2}>
          <MenuButton
            className="text-info"
            size={24}
            style={{ cursor: "pointer" }}
            onClick={popupMenu}
          />
          <h2 className="m-0">{t("playlist.title")}</h2>
        </Stack>
        <List
          headers={{ id: null, name: t("playlist.name.label") }}
          data={playlists || []}
          handleNew={newPlaylist}
          handleDelete={deletePlaylist}
          handleSelect={selectPlaylist}
        />
      </Stack>
    </Container>
  );
}
