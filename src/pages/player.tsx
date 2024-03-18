import { useEffect, useState } from "react";
import { Col, Container, Row, Stack } from "react-bootstrap";
import { useSearchParams } from "react-router-dom";
import { MenuButton } from "react-bootstrap-icons";
import { useTranslation } from "react-i18next";

import Group from "../components/group";
import { ApiServices } from "../services/api";
import { MenuModal, useMenu } from "../components/menu_modal";
import { EntryDetails } from "../services/api/entry";
import { ErrorModal, useError } from "../components/error_modal";
import Entry from "../components/entry";

interface Props {
  apiServices: ApiServices;
}

export default function Player(props: Props) {
  const [entry, setEntry] = useState<EntryDetails | null>(null);

  const { t } = useTranslation();
  const [searchParams, _] = useSearchParams();

  const errorState = useError();
  const menuState = useMenu();

  const playlistId = searchParams.get("playlist_id");
  const groupId = searchParams.get("group_id");
  const entryId = searchParams.get("entry_id");

  const fetchEntry = async () => {
    if (entryId) {
      setEntry((await props.apiServices.entry.show(entryId)).body);
    }
  };

  useEffect(() => {
    fetchEntry().catch(errorState.popup);
  }, []);

  const popupMenu = () => {
    menuState.popup();
  };

  return (
    <Container fluid className="vh-100 d-flex p-3">
      <ErrorModal state={errorState} />
      <MenuModal state={menuState} apiServices={props.apiServices} />
      <Stack gap={3}>
        <Stack direction="horizontal" gap={2}>
          <MenuButton
            className="text-info"
            size={24}
            style={{ cursor: "pointer" }}
            onClick={popupMenu}
          />
          <h2 className="m-0">
            {t("player.title")} {entry ? entry.meta.path : "???"}
          </h2>
        </Stack>
        <Row className="w-100">
          <Col md={6}>
            {playlistId && (
              <Group
                apiServices={props.apiServices}
                playlistId={playlistId}
                groupId={groupId}
              />
            )}
          </Col>
          <Col md={6}>
            {playlistId && groupId && (
              <Entry
                apiServices={props.apiServices}
                playlistId={playlistId}
                groupId={groupId}
                entryId={entryId}
              />
            )}
          </Col>
        </Row>
      </Stack>
    </Container>
  );
}
