import { useEffect, useState } from "react";
import { Col, Container, Row, Stack } from "react-bootstrap";
import { useSearchParams } from "react-router-dom";
import { MenuButton } from "react-bootstrap-icons";
import { useTranslation } from "react-i18next";

import Group from "../components/group";
import { ApiServices } from "../services/api";
import { MenuModal, useMenu } from "../components/menu_modal";
import { EntryBrief, EntryDetails } from "../services/api/entry";
import { ErrorModal, useError } from "../components/error_modal";
import Entry from "../components/entry";
import { PlaylistDetails } from "../services/api/playlist";
import { GroupBrief, GroupDetails } from "../services/api/group";
import { OmniPlayer } from "../components/omni_player";

interface Props {
  apiServices: ApiServices;
}

export default function Player(props: Props) {
  const [playlist, setPlaylist] = useState<PlaylistDetails | null>(null);
  const [groups, setGroups] = useState<GroupBrief[]>([]);
  const [group, setGroup] = useState<GroupDetails | null>(null);
  const [entries, setEntries] = useState<EntryBrief[]>([]);
  const [entry, setEntry] = useState<EntryDetails | null>(null);

  const { t } = useTranslation();
  const [searchParams] = useSearchParams();

  const errorState = useError();
  const menuState = useMenu();

  const clear = () => {
    setPlaylist(null);
    setGroups([]);
    setGroup(null);
    setEntries([]);
    setEntry(null);
  };

  const fetchGroups = (playlistId: string) => {
    props.apiServices.group
      .index({ playlist_id: playlistId })
      .then((resp) => setGroups(resp.body))
      .catch((err) => errorState.popup(err));
  };

  const fetchEntries = (groupId: string) => {
    props.apiServices.entry
      .index({ group_id: groupId })
      .then((resp) => setEntries(resp.body))
      .catch((err) => errorState.popup(err));
  };

  useEffect(() => {
    clear();
    const playlistId = searchParams.get("playlist_id");
    const groupId = searchParams.get("group_id");
    const entryId = searchParams.get("entry_id");
    if (playlistId) {
      props.apiServices.playlist
        .show(playlistId)
        .then((resp) => {
          setPlaylist(resp.body);
          fetchGroups(resp.body.id);
        })
        .catch((err) => errorState.popup(err));
    }
    if (groupId) {
      props.apiServices.group
        .show(groupId)
        .then((resp) => {
          setGroup(resp.body);
          fetchEntries(resp.body.id);
        })
        .catch((err) => errorState.popup(err));
    }
    if (entryId) {
      props.apiServices.entry
        .show(entryId)
        .then((resp) => setEntry(resp.body))
        .catch((err) => errorState.popup(err));
    }
  }, [searchParams]);

  const popupMenu = () => {
    menuState.popup();
  };

  return (
    <Container fluid>
      <ErrorModal state={errorState} />
      <MenuModal state={menuState} apiServices={props.apiServices} />
      {playlist && group && entry && (
        <>
          <OmniPlayer
            apiServices={props.apiServices}
            playlist={playlist}
            groups={groups}
            group={group}
            entries={entries}
            entry={entry}
          />
          <hr />
        </>
      )}
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
            {playlist && (
              <Group
                apiServices={props.apiServices}
                playlist={playlist}
                groups={groups}
                group={group}
                fetchGroups={fetchGroups}
              />
            )}
          </Col>
          <Col md={6}>
            {playlist && group && (
              <Entry
                apiServices={props.apiServices}
                playlist={playlist}
                group={group}
                entries={entries}
                entry={entry}
                fetchEntries={fetchEntries}
              />
            )}
          </Col>
        </Row>
      </Stack>
    </Container>
  );
}
