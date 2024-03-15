import { useEffect, useState } from "react";
import { ApiServices } from "../services/api";
import { PlaylistBrief } from "../services/api/playlist";
import { ErrorModal, useError } from "./error_modal";
import { Stack, Table } from "react-bootstrap";
import { PlusCircle, XCircle } from "react-bootstrap-icons";
import { FormModal, useForm } from "./form_modal";
import { useTranslation } from "react-i18next";
import { confirm } from "@tauri-apps/plugin-dialog";

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

  const addPlaylist = async () => {
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
    <Stack gap={2}>
      <ErrorModal state={errorState} />
      <FormModal
        state={formState}
        handleClose={() => {
          formState.setShow(false);
        }}
        handleSubmit={createPlaylist}
      />
      <Table striped bordered hover>
        <thead>
          <tr>
            <td>{t("playlist.name.label")}</td>
            <td
              className="text-end"
              style={{ whiteSpace: "nowrap", width: "1px" }}
            >
              <PlusCircle
                className="text-info"
                size={24}
                onClick={addPlaylist}
              />
            </td>
          </tr>
        </thead>
        <tbody>
          {playlists &&
            playlists.map((playlist) => {
              return (
                <tr key={playlist.id}>
                  <td>{playlist.name}</td>
                  <td
                    className="text-end"
                    style={{ whiteSpace: "nowrap", width: "1px" }}
                  >
                    <XCircle
                      className="text-danger"
                      size={24}
                      onClick={() => deletePlaylist(playlist.id)}
                    />
                  </td>
                </tr>
              );
            })}
        </tbody>
      </Table>
    </Stack>
  );
}
