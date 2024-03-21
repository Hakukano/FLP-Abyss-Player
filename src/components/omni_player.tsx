import { useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { ArrowLeftSquare, ArrowRightSquare } from "react-bootstrap-icons";
import { Col, Row } from "react-bootstrap";

import { EntryBrief, EntryDetails } from "../services/api/entry";
import { GroupBrief, GroupDetails } from "../services/api/group";
import { useNavigate } from "react-router-dom";
import { ApiServices } from "../services/api";
import { ErrorModal, useError } from "./error_modal";
import { PlaylistDetails } from "../services/api/playlist";

interface Props {
  apiServices: ApiServices;
  playlist: PlaylistDetails;
  groups: GroupBrief[];
  group: GroupDetails;
  entries: EntryBrief[];
  entry: EntryDetails;
}

export function OmniPlayer(props: Props) {
  const [auto, setAuto] = useState(false);
  const [autoInterval, setAutoInterval] = useState(0);
  const [repeat, setRepeat] = useState(false);
  const [random, setRandom] = useState(false);
  const [loop, setLoop] = useState(false);
  const [groupRandom, setGroupRandom] = useState(false);
  const [groupLoop, setGroupLoop] = useState(false);

  const errorState = useError();
  const navigate = useNavigate();

  const nextEntry = async () => {
    if (repeat) {
      navigate(0);
    }
    const groupIndex = props.groups.findIndex(
      (group) => group.id === props.group.id,
    );
    const entryIndex = props.entries.findIndex(
      (entry) => entry.id === props.entry.id,
    );
    if (groupIndex < 0 || entryIndex < 0) {
      return;
    }
    let nextGroupIndex = groupIndex;
    let nextEntryIndex = entryIndex;
    if (random) {
      nextEntryIndex = Math.floor(Math.random() * props.entries.length);
    } else if (entryIndex < props.entries.length - 1) {
      nextEntryIndex += 1;
    } else if (loop) {
      nextEntryIndex = 0;
    } else if (groupRandom) {
      nextGroupIndex = Math.floor(Math.random() * props.groups.length);
    } else if (groupIndex < props.groups.length - 1) {
      nextGroupIndex += 1;
    } else if (groupLoop) {
      nextGroupIndex = 0;
    }
    if (nextGroupIndex !== groupIndex) {
      try {
        const resp = await props.apiServices.entry.index({
          group_id: nextGroupIndex.toString(),
        });
        navigate(
          `/player?playlist_id=${props.playlist.id}&group_id=${props.groups[nextGroupIndex].id}&entry_id=${resp.body[0].id}`,
        );
      } catch (err) {
        errorState.popup(err);
      }
    } else if (nextEntryIndex !== entryIndex) {
      navigate(
        `/player?playlist_id=${props.playlist.id}&group_id=${props.group.id}&entry_id=${props.entries[nextEntryIndex].id}`,
      );
    }
  };

  const previousEntry = async () => {
    if (repeat) {
      navigate(0);
    }
    const groupIndex = props.groups.findIndex(
      (group) => group.id === props.group.id,
    );
    const entryIndex = props.entries.findIndex(
      (entry) => entry.id === props.entry.id,
    );
    if (groupIndex < 0 || entryIndex < 0) {
      return;
    }
    let previousGroupIndex = groupIndex;
    let previousEntryIndex = entryIndex;
    if (random) {
      previousEntryIndex = Math.floor(Math.random() * props.entries.length);
    } else if (entryIndex > 0) {
      previousEntryIndex -= 1;
    } else if (loop) {
      previousEntryIndex = props.entries.length - 1;
    } else if (groupRandom) {
      previousGroupIndex = Math.floor(Math.random() * props.groups.length);
    } else if (groupIndex > 0) {
      previousGroupIndex -= 1;
    } else if (groupLoop) {
      previousGroupIndex = props.groups.length - 1;
    }
    if (previousGroupIndex !== groupIndex) {
      try {
        const resp = await props.apiServices.entry.index({
          group_id: props.groups[previousGroupIndex].id,
        });
        navigate(
          `/player?playlist_id=${props.playlist.id}&group_id=${props.groups[previousGroupIndex].id}&entry_id=${resp.body[resp.body.length - 1].id}`,
        );
      } catch (err) {
        errorState.popup(err);
      }
    } else if (previousEntryIndex !== entryIndex) {
      navigate(
        `/player?playlist_id=${props.playlist.id}&group_id=${props.group.id}&entry_id=${props.entries[previousEntryIndex].id}`,
      );
    }
  };

  return (
    <>
      <ErrorModal state={errorState} />
      <Row className="vw-100" style={{ height: "calc(100vh - 36px)" }}>
        <Col
          style={{
            maxWidth: "100%",
            maxHeight: "100%",
          }}
        >
          {props.entry.mime.startsWith("image") ? (
            <img
              src={convertFileSrc(props.entry.meta.path)}
              style={{
                width: "100%",
                height: "100%",
                objectFit: "contain",
              }}
            />
          ) : (
            <></>
          )}
        </Col>
      </Row>
      <Row
        className="vw-100"
        style={{ height: "32px", alignItems: "center" }}
        onKeyUp={(e) => {
          if (e.key === "ArrowLeft") {
            previousEntry();
          }
          if (e.key === "ArrowRight") {
            nextEntry();
          }
        }}
      >
        <Col md={2}>
          <ArrowLeftSquare
            size={24}
            className="text-info"
            style={{ cursor: "pointer" }}
            onClick={previousEntry}
          />
        </Col>
        <Col md={2}>
          <ArrowRightSquare
            size={24}
            className="text-info"
            style={{ cursor: "pointer" }}
            onClick={nextEntry}
          />
        </Col>
      </Row>
    </>
  );
}
