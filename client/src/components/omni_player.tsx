import { useEffect, useState } from "react";
import {
  ArrowLeftSquare,
  ArrowRightSquare,
  Circle,
  CircleFill,
  PlayCircle,
  PlayCircleFill,
  QuestionCircle,
  QuestionCircleFill,
  RCircle,
  RCircleFill,
} from "react-bootstrap-icons";
import { Col, FormControl, Row, Stack } from "react-bootstrap";
import ReactPlayer from "react-player";

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
  const [auto, setAuto] = useState(localStorage.getItem("auto") === "true");
  const [autoInterval, setAutoInterval] = useState(
    parseInt(localStorage.getItem("auto_interval") || "1"),
  );
  const [repeat, setRepeat] = useState(
    localStorage.getItem("repeat") === "true",
  );
  const [random, setRandom] = useState(
    localStorage.getItem("random") === "true",
  );
  const [loop, setLoop] = useState(localStorage.getItem("loop") === "true");
  const [groupRandom, setGroupRandom] = useState(
    localStorage.getItem("group_random") === "true",
  );
  const [groupLoop, setGroupLoop] = useState(
    localStorage.getItem("group_loop") === "true",
  );

  const [canProceed, setCanProceed] = useState(
    props.entry.mime.startsWith("image"),
  );

  const errorState = useError();
  const navigate = useNavigate();

  const updateAuto = (value: boolean) => {
    localStorage.setItem("auto", value.toString());
    setAuto(value);
  };
  const updateAutoInterval = (value: number) => {
    localStorage.setItem("auto_interval", value.toString());
    setAutoInterval(value);
  };
  const updateRepeat = (value: boolean) => {
    localStorage.setItem("repeat", value.toString());
    setRepeat(value);
  };
  const updateRandom = (value: boolean) => {
    localStorage.setItem("random", value.toString());
    setRandom(value);
  };
  const updateLoop = (value: boolean) => {
    localStorage.setItem("loop", value.toString());
    setLoop(value);
  };
  const updateGroupRandom = (value: boolean) => {
    localStorage.setItem("group_random", value.toString());
    setGroupRandom(value);
  };
  const updateGroupLoop = (value: boolean) => {
    localStorage.setItem("group_loop", value.toString());
    setGroupLoop(value);
  };

  const nextEntry = () => {
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
      props.apiServices.entry
        .index({
          group_id: props.groups[nextGroupIndex].id,
        })
        .then((resp) => {
          navigate(
            `/player?playlist_id=${props.playlist.id}&group_id=${props.groups[nextGroupIndex].id}&entry_id=${resp[0].id}`,
          );
        })
        .catch((err) => errorState.popup(err));
    } else if (nextEntryIndex !== entryIndex) {
      navigate(
        `/player?playlist_id=${props.playlist.id}&group_id=${props.group.id}&entry_id=${props.entries[nextEntryIndex].id}`,
      );
    }
  };

  const previousEntry = () => {
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
      props.apiServices.entry
        .index({
          group_id: props.groups[previousGroupIndex].id,
        })
        .then((resp) => {
          navigate(
            `/player?playlist_id=${props.playlist.id}&group_id=${props.groups[previousGroupIndex].id}&entry_id=${resp[resp.length - 1].id}`,
          );
        })
        .catch((err) => errorState.popup(err));
    } else if (previousEntryIndex !== entryIndex) {
      navigate(
        `/player?playlist_id=${props.playlist.id}&group_id=${props.group.id}&entry_id=${props.entries[previousEntryIndex].id}`,
      );
    }
  };

  useEffect(() => {
    if (canProceed && auto) {
      const interval = setTimeout(nextEntry, autoInterval * 1000);
      return () => clearTimeout(interval);
    }
  }, [canProceed, auto, autoInterval]);

  return (
    <>
      <ErrorModal state={errorState} />
      <Row
        style={{ width: "calc(100% - 32px)", height: "calc(100vh - 64px)" }}
        onClick={(event) => {
          if (event.clientX < window.innerWidth / 2) {
            previousEntry();
          } else {
            nextEntry();
          }
        }}
      >
        <Col
          style={{
            maxWidth: "100%",
            maxHeight: "100%",
          }}
        >
          {props.entry.mime.startsWith("image") ? (
            <img
              src={`/stream/entries/${props.entry.id}`}
              className="w-100 h-100"
              style={{
                objectFit: "contain",
              }}
            />
          ) : (
            <ReactPlayer
              url={`/stream/entries/${props.entry.id}`}
              playing={true}
              loop={loop}
              controls
              className="w-100 h-100"
              onEnded={() => setCanProceed(true)}
            />
          )}
        </Col>
      </Row>
      <Row
        style={{
          width: "calc(100% - 32px)",
          paddingTop: "4px",
          alignItems: "center",
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
        <Col md={1}>
          {groupRandom ? (
            <QuestionCircleFill
              size={24}
              className="text-warning"
              style={{ cursor: "pointer" }}
              onClick={() => updateGroupRandom(false)}
            />
          ) : (
            <QuestionCircle
              size={24}
              className="text-warning"
              style={{ cursor: "pointer" }}
              onClick={() => updateGroupRandom(true)}
            />
          )}
        </Col>
        <Col md={1}>
          {groupLoop ? (
            <CircleFill
              size={24}
              className="text-warning"
              style={{ cursor: "pointer" }}
              onClick={() => updateGroupLoop(false)}
            />
          ) : (
            <Circle
              size={24}
              className="text-warning"
              style={{ cursor: "pointer" }}
              onClick={() => updateGroupLoop(true)}
            />
          )}
        </Col>
        <Col md={3} className="d-flex justify-content-center">
          <Stack direction="horizontal" gap={1}>
            {auto ? (
              <PlayCircleFill
                size={24}
                className="text-info"
                style={{ cursor: "pointer" }}
                onClick={() => updateAuto(false)}
              />
            ) : (
              <PlayCircle
                size={24}
                className="text-info"
                style={{ cursor: "pointer" }}
                onClick={() => updateAuto(true)}
              />
            )}
            <FormControl
              type="number"
              defaultValue={autoInterval}
              min={1}
              onChange={(e) => updateAutoInterval(parseInt(e.target.value))}
            />
          </Stack>
        </Col>
        <Col md={1} className="d-flex justify-content-end">
          {repeat ? (
            <RCircleFill
              size={24}
              className="text-success"
              style={{ cursor: "pointer" }}
              onClick={() => updateRepeat(false)}
            />
          ) : (
            <RCircle
              size={24}
              className="text-success"
              style={{ cursor: "pointer" }}
              onClick={() => updateRepeat(true)}
            />
          )}
        </Col>
        <Col md={1} className="d-flex justify-content-end">
          {random ? (
            <QuestionCircleFill
              size={24}
              className="text-success"
              style={{ cursor: "pointer" }}
              onClick={() => updateRandom(false)}
            />
          ) : (
            <QuestionCircle
              size={24}
              className="text-success"
              style={{ cursor: "pointer" }}
              onClick={() => updateRandom(true)}
            />
          )}
        </Col>
        <Col md={1} className="d-flex justify-content-end">
          {loop ? (
            <CircleFill
              size={24}
              className="text-success"
              style={{ cursor: "pointer" }}
              onClick={() => updateLoop(false)}
            />
          ) : (
            <Circle
              size={24}
              className="text-success"
              style={{ cursor: "pointer" }}
              onClick={() => updateLoop(true)}
            />
          )}
        </Col>
        <Col md={2} className="d-flex justify-content-end">
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
