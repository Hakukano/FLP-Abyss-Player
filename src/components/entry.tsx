import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { confirm } from "@tauri-apps/plugin-dialog";
import { Stack } from "react-bootstrap";
import { useNavigate } from "react-router-dom";
import { ScanModal, useScan } from "./scan_modal";

import { ApiServices } from "../services/api";
import { GroupDetails } from "../services/api/group";
import List from "./list";
import { ErrorModal, useError } from "./error_modal";
import { PlaylistDetails } from "../services/api/playlist";
import { EntryBrief, EntryDetails } from "../services/api/entry";

interface Props {
  apiServices: ApiServices;
  playlist: PlaylistDetails;
  group: GroupDetails;
  entries: EntryBrief[];
  entry: EntryDetails | null;

  fetchEntries: (groupId: string) => void;
}

export default function Entry(props: Props) {
  const [listData, setListData] = useState<{ id: string; path: string }[]>([]);

  const { t } = useTranslation();
  const navigate = useNavigate();

  const errorState = useError();
  const scanState = useScan();

  const refreshListData = () => {
    setListData(
      props.entries.map((entry) => {
        return {
          id: entry.id,
          path: entry.meta.path.replace(props.group.meta.path, ""),
        };
      }),
    );
  };

  const newEntry = () => {
    scanState.popup();
  };

  const closeScan = () => {
    scanState.setShow(false);
    props.fetchEntries(props.group.id);
  };

  const deleteEntry = async (id: string) => {
    if (await confirm(t("entry.delete.confirm"))) {
      await props.apiServices.entry.destroy(id);
      if (id === props.entry?.id) {
        navigate(
          `/player?playlist_id=${props.playlist.id}&group_id=${props.group.id}`,
        );
      } else {
        props.fetchEntries(props.group.id);
      }
    }
  };

  const selectEntry = (id: string) => {
    navigate(
      `/player?playlist_id=${props.playlist.id}&group_id=${props.group.id}&entry_id=${id}`,
    );
  };

  const shiftEntry = async (id: string, offset: number) => {
    await props.apiServices.entry.shift(id, { offset });
    props.fetchEntries(props.group.id);
  };

  const sortEntries = async (values: { [key: string]: any }) => {
    await props.apiServices.entry.sort({
      by: values["by"].value,
      ascend: values["ascend"],
    });
    props.fetchEntries(props.group.id);
  };

  useEffect(() => {
    refreshListData();
  }, [props.entries]);

  return (
    <Stack gap={3}>
      <ErrorModal state={errorState} />
      <ScanModal
        state={scanState}
        apiServices={props.apiServices}
        playlistId={props.playlist.id}
        handleClose={closeScan}
      />
      <h2>{t("entry.title")}</h2>
      <List
        headers={{ id: null, path: t("entry.path.label") }}
        data={listData}
        highlightedIds={props.entry ? new Set([props.entry.id]) : new Set()}
        handleNew={newEntry}
        handleDelete={deleteEntry}
        handleSelect={selectEntry}
        handleShift={shiftEntry}
        handleSort={sortEntries}
      />
    </Stack>
  );
}
