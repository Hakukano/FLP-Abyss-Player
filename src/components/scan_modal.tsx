import { Dispatch, SetStateAction, useState } from "react";
import { ErrorModal, useError } from "./error_modal";
import { useTranslation } from "react-i18next";
import { Button, Col, Modal, Row, Stack, Table } from "react-bootstrap";
import { ApiServices } from "../services/api";
import { open } from "@tauri-apps/plugin-dialog";
import { PlusCircle, XCircle } from "react-bootstrap-icons";

interface UseScan {
  show: boolean;
  setShow: Dispatch<SetStateAction<boolean>>;
  popup: () => void;
}

export function useScan(): UseScan {
  const [show, setShow] = useState(false);

  const popup = () => {
    setShow(true);
  };

  return { show, setShow, popup };
}

interface Props {
  state: UseScan;
  apiServices: ApiServices;

  handleClose: () => void;
}

export function ScanModal(props: Props) {
  const [rootPath, setRootPath] = useState<string | null>(null);
  const [inputMime, setInputMime] = useState<string>("");
  const [allowedMimes, setAllowedMimes] = useState<string[]>([]);
  const [fullPaths, setFullPaths] = useState<string[]>([]);
  const [oneLevelDeeper, setOneLevelDeeper] = useState(false);

  const { t } = useTranslation();

  const errorState = useError();

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

  const handleScan = () => {
    if (!rootPath) {
      return errorState.popup(t("scan.errors.root_not_set"));
    }
    if (allowedMimes.length === 0) {
      return errorState.popup(t("scan.errors.allowed_mimes_not_set"));
    }
    props.apiServices.scanner
      .index({ root_path: rootPath, allowed_mimes: allowedMimes })
      .then((resp) => setFullPaths(resp.body))
      .catch((err) => errorState.popup(err));
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
          {fullPaths.length === 0 ? (
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
                        <input
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
            <Stack gap={3}></Stack>
          )}
        </Modal.Body>
        <Modal.Footer>
          {fullPaths.length === 0 ? (
            <Button variant="primary" onClick={handleScan}>
              {t("scan.scan")}
            </Button>
          ) : (
            <Button variant="primary" onClick={handleScan}>
              {t("scan.submit")}
            </Button>
          )}
        </Modal.Footer>
      </Modal>
    </>
  );
}
