import { useEffect, useState } from "react";
import { ApiServices } from "../services/api";
import { GroupBrief } from "../services/api/group";
import { ErrorModal, useError } from "./error_modal";
import { useTranslation } from "react-i18next";
import { confirm } from "@tauri-apps/plugin-dialog";
import List from "./list";
import { Stack } from "react-bootstrap";
import { useNavigate, useSearchParams } from "react-router-dom";
import { ScanModal, useScan } from "./scan_modal";

interface Props {
  apiServices: ApiServices;
}

export default function Group(props: Props) {
  const [groups, setGroups] = useState<GroupBrief[] | null>(null);

  const { t } = useTranslation();
  const [searchParams, _] = useSearchParams();
  const navigate = useNavigate();

  const errorState = useError();
  const scanState = useScan();

  const playlist_id = searchParams.get("playlist_id");

  const fetchGroups = async () => {
    setGroups(
      (
        await props.apiServices.group.index({
          playlist_id: playlist_id,
        })
      ).body,
    );
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
    navigate(`/player?playlist_id=${playlist_id}&group_id=${id}`);
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
        handleClose={closeScan}
      />
      <h2>{t("group.title")}</h2>
      <List
        headers={{ path: t("group.path.label") }}
        data={groups || []}
        handleNew={newGroup}
        handleDelete={deleteGroup}
        handleSelect={selectGroup}
        handleShift={shiftGroup}
        handleSort={sortGroups}
      />
    </Stack>
  );
}
