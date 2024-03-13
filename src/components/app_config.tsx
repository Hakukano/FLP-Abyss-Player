import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import Button from "react-bootstrap/Button";
import Col from "react-bootstrap/Col";
import Form from "react-bootstrap/Form";
import Row from "react-bootstrap/Row";
import Stack from "react-bootstrap/Stack";
import Toast from "react-bootstrap/Toast";
import ToastContainer from "react-bootstrap/ToastContainer";

import { open } from "@tauri-apps/plugin-dialog";

import { AppConfigBrief } from "../services/api/app_config.ts";
import translations from "../translations.ts";
import { PLAYLIST_EXTENSION } from "../utils/consts.ts";
import { ApiServices } from "../services/api.ts";

const FORM_LABEL_WIDTH = 3;
const FORM_INPUT_WIDTH = 9;

interface Props {
  apiServices: ApiServices;
}

export default function AppConfig(props: Props) {
  const { t } = useTranslation();

  const [error, setError] = useState("");
  const [errorShow, setErrorShow] = useState(false);
  const [appConfig, setAppConfig] = useState<AppConfigBrief | null>(null);

  const handleError = (err: any) => {
    if (typeof err === "string") {
      setError(err);
    } else {
      setError(JSON.stringify(err));
    }
    setErrorShow(true);
  };

  const fetchAppConfig = async () => {
    const resp = await props.apiServices.appConfig.index();
    setAppConfig(resp.body);
  };

  const setLocale = async (locale: string) => {
    if (appConfig) {
      const config = appConfig;
      config.locale = locale;
      await props.apiServices.appConfig.update(config);
      await fetchAppConfig();
    }
  };

  const setPlaylist = async () => {
    if (appConfig) {
      const path = await open({
        multiple: false,
        directory: false,
        filters: [{ name: "APPL", extensions: [PLAYLIST_EXTENSION] }],
      });
      const config = appConfig;
      config.playlist = path?.path || null;
      await props.apiServices.appConfig.update(config);
      await fetchAppConfig();
    }
  };

  useEffect(() => {
    fetchAppConfig().catch(handleError);
  }, []);

  return (
    <Stack gap={3}>
      <ToastContainer
        className="p-3"
        position="top-center"
        style={{ zIndex: 1 }}
      >
        <Toast onClose={() => setErrorShow(false)} show={errorShow}>
          <Toast.Header>
            <strong className="me-auto">{t("error")}</strong>
          </Toast.Header>
          <Toast.Body>{error}</Toast.Body>
        </Toast>
      </ToastContainer>
      <h2>{t("app_config.title")}</h2>
      <Form>
        <Form.Group as={Row} className="mb-3" controlId="app-config-locale">
          <Form.Label column md={FORM_LABEL_WIDTH}>
            {t("app_config.locale.name")}
          </Form.Label>
          <Col md={FORM_INPUT_WIDTH}>
            <Form.Select
              value={appConfig?.locale}
              onChange={(event) => {
                setLocale(event.target.value).catch(handleError);
              }}
            >
              {Object.entries(translations).map(([language, translation]) => (
                <option key={language} value={language}>
                  {translation.language_name}
                </option>
              ))}
            </Form.Select>
          </Col>
          <Form.Text muted>{t("app_config.locale.description")}</Form.Text>
        </Form.Group>
        <Form.Group
          as={Row}
          className="mb-3"
          controlId="app-config-playlist-path"
        >
          <Form.Label column md={FORM_LABEL_WIDTH}>
            {t("app_config.playlist.name")}
          </Form.Label>
          <Col md={FORM_INPUT_WIDTH}>
            <Button
              variant="light"
              className="w-100 text-start"
              onClick={() => setPlaylist().catch(handleError)}
            >
              {appConfig?.playlist || t("app_config.playlist.placeholder")}
            </Button>
          </Col>
          <Form.Text muted>{t("app_config.playlist.description")}</Form.Text>
        </Form.Group>
      </Form>
    </Stack>
  );
}
