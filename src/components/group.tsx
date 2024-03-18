import { useEffect, useState } from "react";
import { ApiServices } from "../services/api";
import { ErrorModal, useError } from "./error_modal";
import { useTranslation } from "react-i18next";
import { confirm } from "@tauri-apps/plugin-dialog";
import List from "./list";
import { Stack } from "react-bootstrap";
import { useNavigate } from "react-router-dom";
import { ScanModal, useScan } from "./scan_modal";

interface Props {
  apiServices: ApiServices;
  playlistId: string;
  groupId: string | null;
}

export default function Group(props: Props) {
  const [listData, setListData] = useState<{ id: string; path: string }[]>([]);

  const { t } = useTranslation();
  const navigate = useNavigate();

  const errorState = useError();
  const scanState = useScan();

  const fetchGroups = async () => {
    try {
      setListData(
        (
          await props.apiServices.group.index({
            playlist_id: props.playlistId,
          })
        ).body.map((group) => {
          return { id: group.id, path: group.meta.path };
        }),
      );
    } catch (_) {
      setListData([]);
    }
  };

  const newGroup = () => {
    scanState.popup();
  };

  const closeScan = () => {
    fetchGroups()
      .then(() => scanState.setShow(false))
      .catch((err) => errorState.popup(err));
  };

  const deleteGroup = async (id: string) => {
    if (await confirm(t("group.delete.confirm"))) {
      await props.apiServices.group.destroy(id);
      await fetchGroups();
    }
  };

  const selectGroup = (id: string) => {
    navigate(`/player?playlist_id=${props.playlistId}&group_id=${id}`);
  };

  const shiftGroup = async (id: string, offset: number) => {
    await props.apiServices.group.shift(id, { offset });
    await fetchGroups();
  };

  const sortGroups = async (values: { [key: string]: any }) => {
    await props.apiServices.group.sort({
      by: values["by"].value,
      ascend: values["ascend"],
    });
    await fetchGroups();
  };

  useEffect(() => {
    fetchGroups().catch(errorState.popup);
  }, []);

  return (
    <Stack gap={3}>
      <ErrorModal state={errorState} />
      <ScanModal
        state={scanState}
        apiServices={props.apiServices}
        playlistId={props.playlistId}
        handleClose={closeScan}
      />
      <h2>{t("group.title")}</h2>
      <List
        headers={{ id: null, path: t("group.path.label") }}
        data={listData}
        highlightedIds={props.groupId ? new Set([props.groupId]) : new Set()}
        handleNew={newGroup}
        handleDelete={deleteGroup}
        handleSelect={selectGroup}
        handleShift={shiftGroup}
        handleSort={sortGroups}
      />
    </Stack>
  );
}
