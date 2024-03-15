import { t } from "i18next";
import { Dispatch, SetStateAction, useState } from "react";
import { ToastContainer, Toast } from "react-bootstrap";

interface UseError {
  message: string;
  setMessage: Dispatch<SetStateAction<string>>;
  show: boolean;
  setShow: Dispatch<SetStateAction<boolean>>;
  handleError: (err: any) => void;
}

export function useError(): UseError {
  const [message, setMessage] = useState("");
  const [show, setShow] = useState(false);

  const handleError = (err: any) => {
    if (typeof err === "string") {
      setMessage(err);
    } else {
      setMessage(JSON.stringify(err));
    }
    setShow(true);
  };

  return { message, setMessage, show, setShow, handleError };
}

interface Props {
  state: UseError;
}

export function ErrorModal(props: Props) {
  return (
    <ToastContainer className="p-3" position="top-center" style={{ zIndex: 1 }}>
      <Toast onClose={() => props.state.setShow(false)} show={props.state.show}>
        <Toast.Header>
          <strong className="me-auto">{t("error")}</strong>
        </Toast.Header>
        <Toast.Body>{props.state.message}</Toast.Body>
      </Toast>
    </ToastContainer>
  );
}
