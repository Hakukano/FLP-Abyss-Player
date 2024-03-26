import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { Stack } from "react-bootstrap";
import { useNavigate } from "react-router-dom";

import List from "./list";
import { ApiServices } from "../services/api";
import { ErrorModal, useError } from "./error_modal";
import { ScanModal, useScan } from "./scan_modal";
import { PlaylistDetails } from "../services/api/playlist";
import { GroupBrief, GroupDetails } from "../services/api/group";

interface Props {
  apiServices: ApiServices;
  playlist: PlaylistDetails;
  groups: GroupBrief[];
  group: GroupDetails | null;

  fetchGroups: (playlistId: string) => void;
}

export default function Group(props: Props) {
  const [listData, setListData] = useState<{ id: string; path: string }[]>([]);

  const { t } = useTranslation();
  const navigate = useNavigate();

  const errorState = useError();
  const scanState = useScan();

  const refreshListData = () => {
    setListData(
      props.groups.map((group) => {
        return { id: group.id, path: group.meta.path };
      }),
    );
  };

  const newGroup = () => {
    scanState.popup();
  };

  const closeScan = () => {
    scanState.setShow(false);
    props.fetchGroups(props.playlist.id);
  };

  const deleteGroup = async (id: string) => {
    if (confirm(t("group.delete.confirm"))) {
      await props.apiServices.group.destroy(id);
      if (id === props.group?.id) {
        navigate(`/player?playlist_id=${props.playlist.id}`);
      } else {
        props.fetchGroups(props.playlist.id);
      }
    }
  };

  const selectGroup = (id: string) => {
    navigate(`/player?playlist_id=${props.playlist.id}&group_id=${id}`);
  };

  const shiftGroup = async (id: string, offset: number) => {
    await props.apiServices.group.shift(id, { offset });
    props.fetchGroups(props.playlist.id);
  };

  const sortGroups = async (values: { [key: string]: any }) => {
    await props.apiServices.group.sort({
      by: values["by"].value,
      ascend: values["ascend"],
    });
    props.fetchGroups(props.playlist.id);
  };

  useEffect(() => {
    refreshListData();
  }, [props.groups]);

  return (
    <Stack gap={3}>
      <ErrorModal state={errorState} />
      <ScanModal
        state={scanState}
        apiServices={props.apiServices}
        playlistId={props.playlist.id}
        handleClose={closeScan}
      />
      <h2>{t("group.title")}</h2>
      <List
        headers={{ id: null, path: t("group.path.label") }}
        data={listData}
        highlightedIds={props.group ? new Set([props.group.id]) : new Set()}
        handleNew={newGroup}
        handleDelete={deleteGroup}
        handleSelect={selectGroup}
        handleShift={shiftGroup}
        handleSort={sortGroups}
      />
    </Stack>
  );
}
