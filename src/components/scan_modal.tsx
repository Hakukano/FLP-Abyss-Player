import { Dispatch, SetStateAction, useState } from "react";
import { ErrorModal, useError } from "./error_modal";
import { useTranslation } from "react-i18next";
import { Button, Modal, Stack, Table } from "react-bootstrap";
import { ApiServices } from "../services/api";
import { open } from "@tauri-apps/plugin-dialog";
import { PlusCircle, XCircle } from "react-bootstrap-icons";

interface UseScan {
  show: boolean;
  setShow: Dispatch<SetStateAction<boolean>>;
}

export function useScan(): UseScan {
  const [show, setShow] = useState(false);

  return { show, setShow };
}

interface Props {
  state: UseScan;
  apiServices: ApiServices;
}

export function ScanModal(props: Props) {
  const [rootPath, setRootPath] = useState<string | null>(null);
  const [allowedMime, setAllowedMime] = useState<string>("");
  const [allowedMimes, setAllowedMimes] = useState<string[]>([]);
  const [fullPaths, setFullPaths] = useState<string[]>([]);
  const [oneLevelDeeper, setOneLevelDeeper] = useState(false);

  const { t } = useTranslation();

  const errorState = useError();

  const handleClose = () => {
    props.state.setShow(false);
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
    if (allowedMime.trim().length === 0) {
      return;
    }
    allowedMimes.push(allowedMime);
    setAllowedMimes(allowedMimes);
  };

  const removeAllowedMime = (mime: string) => {
    allowedMimes.splice(allowedMimes.indexOf(mime), 1);
    setAllowedMimes(allowedMimes);
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
        onHide={handleClose}
        backdrop="static"
        size="xl"
      >
        <Modal.Header closeButton>
          <Modal.Title>{t("scan.title")}</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          {fullPaths.length === 0 ? (
            <Stack gap={3}>
              <Stack direction="horizontal" gap={2}>
                <span>{t("scan.root_path.label")}</span>
                <Button
                  className="float-end"
                  variant={rootPath ? "info" : "danger"}
                  onClick={popupRootPath}
                >
                  {rootPath ? rootPath : t("scan.errors.root_not_set")}
                </Button>
              </Stack>
              <Stack gap={2}>
                <span>{t("scan.allowed_mimes.label")}</span>
                <Table striped bordered hover>
                  <thead>
                    <tr>
                      <td>
                        <input
                          onChange={(e) => {
                            setAllowedMime(e.target.value);
                          }}
                          onKeyUp={(e) => {
                            if (e.key === "Enter") {
                              addAllowedMime();
                            }
                          }}
                        />
                      </td>
                      <td
                        className="text-end"
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
            <></>
          )}
        </Modal.Body>
        <Modal.Footer>
          <Button variant="primary" onClick={handleScan}>
            {t("scan.submit")}
          </Button>
        </Modal.Footer>
      </Modal>
    </>
  );
}
