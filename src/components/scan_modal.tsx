import { Dispatch, SetStateAction, useState } from "react";
import { useTranslation } from "react-i18next";
import {
  Button,
  Col,
  FormCheck,
  FormControl,
  Modal,
  Row,
  Stack,
  Table,
} from "react-bootstrap";
import { open } from "@tauri-apps/plugin-dialog";
import { PlusCircle, XCircle } from "react-bootstrap-icons";

import { ApiServices } from "../services/api";
import { ErrorModal, useError } from "./error_modal";

interface ScanState {
  show: boolean;
  setShow: Dispatch<SetStateAction<boolean>>;
  popup: () => void;
}

export function useScan(): ScanState {
  const [show, setShow] = useState(false);

  const popup = () => {
    setShow(true);
  };

  return { show, setShow, popup };
}

interface Props {
  state: ScanState;
  apiServices: ApiServices;

  handleClose: () => void;
}

export function ScanModal(props: Props) {
  const [rootPath, setRootPath] = useState<string | null>(null);
  const [inputMime, setInputMime] = useState<string>("");
  const [allowedMimes, setAllowedMimes] = useState<string[]>([]);
  const [ungroupedPaths, setUngroupedPaths] = useState<string[]>([]);
  const [oneLevelDeeper, setOneLevelDeeper] = useState(false);
  const [groupPath, setGroupPath] = useState("");
  const [groupedPaths, setGroupedPaths] = useState<{
    [key: string]: string[];
  }>({});

  const { t } = useTranslation();

  const errorState = useError();

  const clear = () => {
    setRootPath(null);
    setInputMime("");
    setAllowedMimes([]);
    setUngroupedPaths([]);
    setOneLevelDeeper(false);
    setGroupPath("");
    setGroupedPaths({});
  };

  const popupRootPath = () => {
    open({
      multiple: false,
      directory: true,
      recursive: true,
    })
      .then((path) => {
        setRootPath(path);
      })
      .catch((err) => {
        errorState.popup(err);
      });
  };

  const addAllowedMime = () => {
    if (inputMime.trim().length === 0) {
      return;
    }
    const newAllowedMimes = [...new Set(allowedMimes.concat([inputMime]))];
    setAllowedMimes(newAllowedMimes);
  };

  const removeAllowedMime = (mime: string) => {
    const newAllowedMimes = allowedMimes.filter(
      (allowedMime) => allowedMime !== mime,
    );
    setAllowedMimes(newAllowedMimes);
  };

  const scan = () => {
    if (!rootPath) {
      return errorState.popup(t("scan.errors.root_not_set"));
    }
    if (allowedMimes.length === 0) {
      return errorState.popup(t("scan.errors.allowed_mimes_not_set"));
    }
    props.apiServices.scanner
      .index({ root_path: rootPath, allowed_mimes: allowedMimes })
      .then((resp) => setUngroupedPaths(resp.body))
      .catch((err) => errorState.popup(err));
  };

  const grouping = () => {
    const groupPathNoEnding = groupPath.replace(/\/$/, "");
    const groupsToMatch = new Set<string>();
    if (oneLevelDeeper) {
      for (const ungroupedPath of ungroupedPaths) {
        const matched = ungroupedPath.match(
          new RegExp(`^${groupPathNoEnding}(/[^/]*)/`),
        );
        if (!matched || matched.length < 2) {
          continue;
        }
        groupsToMatch.add(`${groupPathNoEnding}${matched[1]}`);
      }
    } else {
      groupsToMatch.add(groupPath);
    }
    const toMove: { [key: string]: number[] } = {};
    const groupsToMatchArr = [...groupsToMatch];
    ungroupedPaths.forEach((ungroupedPath, index) => {
      const groupToMatch = groupsToMatchArr.find((groupToMatch) => {
        return ungroupedPath.startsWith(groupToMatch);
      });
      if (!!groupToMatch) {
        if (!toMove[groupToMatch]) {
          toMove[groupToMatch] = [];
        }
        toMove[groupToMatch].push(index);
      }
    });
    const clonedUngroupedPaths: string[] = JSON.parse(
      JSON.stringify(ungroupedPaths),
    );
    const clonedGroupedPaths: { [key: string]: string[] } = JSON.parse(
      JSON.stringify(groupedPaths),
    );
    Object.entries(toMove).forEach(([group, indexes]) => {
      if (indexes.length > 0) {
        if (!clonedGroupedPaths[group]) {
          clonedGroupedPaths[group] = [];
        }
        indexes.forEach((index) => {
          const removed = clonedUngroupedPaths.splice(index, 1)[0];
          if (!!removed) {
            clonedGroupedPaths[group].push(removed);
          }
        });
      }
    });
    setUngroupedPaths(clonedUngroupedPaths);
    setGroupedPaths(clonedGroupedPaths);
  };

  return (
    <>
      <ErrorModal state={errorState} />
      <Modal
        show={props.state.show}
        onHide={props.handleClose}
        backdrop="static"
        size="xl"
      >
        <Modal.Header closeButton>
          <Modal.Title>{t("scan.title")}</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          {ungroupedPaths.length === 0 ? (
            <Stack gap={3}>
              <Row>
                <Col md={3}>
                  <span>{t("scan.root_path.label")}</span>
                </Col>
                <Col md={9}>
                  <Button
                    className="w-100"
                    variant={rootPath ? "info" : "danger"}
                    onClick={popupRootPath}
                  >
                    {rootPath ? rootPath : t("scan.errors.root_not_set")}
                  </Button>
                </Col>
              </Row>
              <Stack gap={2}>
                <span>{t("scan.allowed_mimes.label")}</span>
                <Table striped bordered hover>
                  <thead>
                    <tr>
                      <td>
                        <FormControl
                          className="w-100"
                          onChange={(e) => {
                            setInputMime(e.target.value);
                          }}
                          onKeyUp={(e) => {
                            if (e.key === "Enter") {
                              addAllowedMime();
                            }
                          }}
                        />
                      </td>
                      <td
                        className="text-end align-middle"
                        style={{ whiteSpace: "nowrap", width: "1px" }}
                      >
                        <PlusCircle
                          className="text-info"
                          size={24}
                          style={{ cursor: "pointer" }}
                          onClick={addAllowedMime}
                        />
                      </td>
                    </tr>
                  </thead>
                  <tbody>
                    {allowedMimes.map((mime) => {
                      return (
                        <tr key={mime}>
                          <td>{mime}</td>
                          <td
                            className="text-end"
                            style={{ whiteSpace: "nowrap", width: "1px" }}
                          >
                            <XCircle
                              className="text-danger"
                              size={24}
                              style={{ cursor: "pointer" }}
                              onClick={() => removeAllowedMime(mime)}
                            />
                          </td>
                        </tr>
                      );
                    })}
                  </tbody>
                </Table>
              </Stack>
            </Stack>
          ) : (
            <Stack gap={3}>
              <Stack direction="horizontal" gap={2}>
                <FormControl
                  type="text"
                  onChange={(e) => {
                    setGroupPath(e.target.value);
                  }}
                />
                <span style={{ whiteSpace: "nowrap" }}>
                  {t("scan.ungrouped.one_level_deeper")}
                </span>
                <FormCheck
                  type="checkbox"
                  checked={oneLevelDeeper}
                  onChange={(e) => setOneLevelDeeper(e.target.checked)}
                  className="ms-auto"
                />
                <PlusCircle
                  className="text-info"
                  size={24}
                  style={{ cursor: "pointer" }}
                  onClick={grouping}
                />
              </Stack>
              <Table striped bordered hover>
                <thead>
                  <tr>
                    <td>{t("scan.ungrouped.title")}</td>
                  </tr>
                </thead>
                <tbody>
                  {ungroupedPaths.map((ungroupedPath, index) => {
                    return (
                      <tr key={index}>
                        <td
                          onClick={() =>
                            navigator.clipboard.writeText(ungroupedPath)
                          }
                          style={{ cursor: "pointer" }}
                        >
                          {ungroupedPath}
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </Table>
              <Table striped bordered hover>
                <thead>
                  <tr>
                    <td>{t("scan.grouped.title")}</td>
                    <td>{t("scan.grouped.entry_count")}</td>
                  </tr>
                </thead>
                <tbody>
                  {Object.entries(groupedPaths).map(
                    ([groupedPath, entries], index) => {
                      return (
                        <tr key={index}>
                          <td>{groupedPath}</td>
                          <td>{entries.length}</td>
                        </tr>
                      );
                    },
                  )}
                </tbody>
              </Table>
            </Stack>
          )}
        </Modal.Body>
        <Modal.Footer>
          {ungroupedPaths.length === 0 ? (
            <Button variant="primary" onClick={scan}>
              {t("scan.scan")}
            </Button>
          ) : (
            <>
              <Button variant="secondary" onClick={clear}>
                {t("scan.back")}
              </Button>
              <Button variant="primary" onClick={scan}>
                {t("scan.submit")}
              </Button>
            </>
          )}
        </Modal.Footer>
      </Modal>
    </>
  );
}
