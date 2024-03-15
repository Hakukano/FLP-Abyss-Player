import { useEffect, useState } from "react";
import { ApiServices } from "../services/api";
import { GroupBrief } from "../services/api/group";
import { ErrorModal, useError } from "./error_modal";
import { FormModal, useForm } from "./form_modal";
import { useTranslation } from "react-i18next";
import { confirm } from "@tauri-apps/plugin-dialog";
import List from "./list";
import { Stack } from "react-bootstrap";
import { useSearchParams } from "react-router-dom";

interface Props {
  apiServices: ApiServices;
}

export default function Group(props: Props) {
  const { t } = useTranslation();
  const [searchParams, _] = useSearchParams();

  const [groups, setGroups] = useState<GroupBrief[] | null>(null);

  const errorState = useError();
  const formState = useForm();

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
    formState.popup({
      header: t("group.title"),
      rows: [
        {
          name: "path",
          type: "text",
          initial: "",
          label: t("group.path.label"),
          placeholder: t("group.path.placeholder"),
        },
      ],
    });
  };

  const createGroup = async (values: { [key: string]: any }) => {
    if (!playlist_id) {
      throw "";
    }
    await props.apiServices.group.create({ playlist_id, path: values["path"] });
    await fetchGroups();
  };

  const sortGroups = async (values: { [key: string]: any }) => {};

  const deleteGroup = async (id: string) => {
    if (await confirm(t("group.delete.confirm"))) {
      await props.apiServices.group.destroy(id);
      await fetchGroups();
    }
  };

  const shiftGroup = async (id: string, offset: number) => {
    await props.apiServices.group.shift(id, { offset });
    await fetchGroups();
  };

  useEffect(() => {
    fetchGroups().catch(errorState.handleError);
  }, []);

  return (
    <Stack gap={3}>
      <ErrorModal state={errorState} />
      <FormModal
        state={formState}
        handleClose={() => {
          formState.setShow(false);
        }}
        handleSubmit={createGroup}
      />
      <h2>{t("group.title")}</h2>
      <List
        headers={[t("group.path.label")]}
        data={groups || []}
        handleNew={newGroup}
        handleDelete={deleteGroup}
        handleShift={shiftGroup}
        handleSort={sortGroups}
      />
    </Stack>
  );
}
