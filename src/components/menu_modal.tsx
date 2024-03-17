import { Dispatch, SetStateAction, useState } from "react";
import { Button, Modal, Stack } from "react-bootstrap";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

import { ApiServices } from "../services/api";
import { ErrorModal, useError } from "./error_modal";
import AppConfig from "./app_config";

export interface MenuState {
  show: boolean;
  setShow: Dispatch<SetStateAction<boolean>>;
  popup: () => void;
}

export function useMenu(): MenuState {
  const [show, setShow] = useState(false);

  const popup = () => {
    setShow(true);
  };

  return {
    show,
    setShow,
    popup,
  };
}

interface Props {
  state: MenuState;
  apiServices: ApiServices;
}

export function MenuModal(props: Props) {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const errorState = useError();

  const handleClose = () => {
    props.state.setShow(false);
  };

  const changePlaylist = () => {
    navigate("/playlists");
    handleClose();
  };

  return (
    <>
      <ErrorModal state={errorState} />
      <Modal show={props.state.show} onHide={handleClose} backdrop="static">
        <Modal.Header closeButton>
          <Modal.Title>{t("menu.title")}</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <Stack gap={3}>
            <AppConfig apiServices={props.apiServices} />
          </Stack>
        </Modal.Body>
        <Modal.Footer>
          <Button variant="primary" onClick={changePlaylist}>
            {t("menu.change_playlist")}
          </Button>
        </Modal.Footer>
      </Modal>
    </>
  );
}
