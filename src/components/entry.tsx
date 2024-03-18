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
  groupId: string;
  entryId: string | null;
}

export default function Entry(props: Props) {
  const [listData, setListData] = useState<{ id: string; path: string }[]>([]);

  const { t } = useTranslation();
  const navigate = useNavigate();

  const errorState = useError();
  const scanState = useScan();

  const fetchEntries = async () => {
    try {
      setListData(
        (
          await props.apiServices.entry.index({
            group_id: props.groupId,
          })
        ).body.map((entry) => {
          return { id: entry.id, path: entry.meta.path };
        }),
      );
    } catch (_) {
      setListData([]);
    }
  };

  const newEntry = () => {
    scanState.popup();
  };

  const closeScan = () => {
    fetchEntries()
      .then(() => scanState.setShow(false))
      .catch((err) => errorState.popup(err));
  };

  const deleteEntry = async (id: string) => {
    if (await confirm(t("entry.delete.confirm"))) {
      await props.apiServices.entry.destroy(id);
      await fetchEntries();
    }
  };

  const selectEntry = (id: string) => {
    navigate(
      `/player?playlist_id=${props.playlistId}&group_id=${props.groupId}&entry_id=${id}`,
    );
  };

  const shiftEntry = async (id: string, offset: number) => {
    await props.apiServices.entry.shift(id, { offset });
    await fetchEntries();
  };

  const sortEntries = async (values: { [key: string]: any }) => {
    await props.apiServices.entry.sort({
      by: values["by"].value,
      ascend: values["ascend"],
    });
    await fetchEntries();
  };

  useEffect(() => {
    fetchEntries().catch(errorState.popup);
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
      <h2>{t("entry.title")}</h2>
      <List
        headers={{ id: null, path: t("entry.path.label") }}
        data={listData}
        highlightedIds={props.entryId ? new Set([props.entryId]) : new Set()}
        handleNew={newEntry}
        handleDelete={deleteEntry}
        handleSelect={selectEntry}
        handleShift={shiftEntry}
        handleSort={sortEntries}
      />
    </Stack>
  );
}
