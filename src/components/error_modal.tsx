import { Dispatch, SetStateAction, useState } from "react";
import { ToastContainer, Toast } from "react-bootstrap";
import { useTranslation } from "react-i18next";

interface UseError {
  message: string;
  setMessage: Dispatch<SetStateAction<string>>;
  show: boolean;
  setShow: Dispatch<SetStateAction<boolean>>;
  popup: (err: any) => void;
}

export function useError(): UseError {
  const [message, setMessage] = useState("");
  const [show, setShow] = useState(false);

  const popup = (err: any) => {
    if (typeof err === "string") {
      setMessage(err);
    } else {
      setMessage(JSON.stringify(err));
    }
    setShow(true);
  };

  return { message, setMessage, show, setShow, popup: popup };
}

interface Props {
  state: UseError;
}

export function ErrorModal(props: Props) {
  const { t } = useTranslation();

  return (
    <ToastContainer
      className="p-3"
      position="bottom-end"
      style={{ zIndex: 2147483647 }}
    >
      <Toast onClose={() => props.state.setShow(false)} show={props.state.show}>
        <Toast.Header>
          <strong className="me-auto">{t("error")}</strong>
        </Toast.Header>
        <Toast.Body>{props.state.message}</Toast.Body>
      </Toast>
    </ToastContainer>
  );
}
