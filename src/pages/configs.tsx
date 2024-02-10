import React, { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import Button from "react-bootstrap/Button";
import Col from "react-bootstrap/Col";
import Form from "react-bootstrap/Form";
import Row from "react-bootstrap/Row";
import Stack from "react-bootstrap/Stack";
import Toast from "react-bootstrap/Toast";
import ToastContainer from "react-bootstrap/ToastContainer";

import { open } from "@tauri-apps/plugin-dialog";

import {
  AppConfigBrief,
  AppConfigService,
} from "../services/api/app_config.ts";
import translations from "../translations.ts";

const FORM_LABEL_WIDTH = 2;
const FORM_INPUT_WIDTH = 4;
const FORM_DESCRIPTION_WIDTH = 6;

interface Props extends React.HTMLAttributes<HTMLElement> {
  appConfigService: AppConfigService;
}

export default function Config(props: Props) {
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
    const resp = await props.appConfigService.index();
    setAppConfig(resp.body);
  };

  const setLocale = async (locale: string) => {
    if (appConfig) {
      const config = appConfig;
      config.locale = locale;
      await props.appConfigService.update(config);
      await fetchAppConfig();
    }
  };

  const setRootPath = async () => {
    if (appConfig) {
      const path = await open({ multiple: false, directory: true });
      const config = appConfig;
      config.root_path = path;
      await props.appConfigService.update(config);
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
          <Form.Label column sm={FORM_LABEL_WIDTH}>
            {t("app_config.locale.name")}
          </Form.Label>
          <Col sm={FORM_INPUT_WIDTH}>
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
          <Form.Label column sm={FORM_DESCRIPTION_WIDTH}>
            {t("app_config.locale.description")}
          </Form.Label>
        </Form.Group>
        <Form.Group as={Row} className="mb-3" controlId="app-config-root-path">
          <Form.Label column sm={FORM_LABEL_WIDTH}>
            {t("app_config.root_path.name")}
          </Form.Label>
          <Col sm={FORM_INPUT_WIDTH}>
            <Button
              variant="light"
              className="w-100 text-start"
              onClick={() => setRootPath().catch(handleError)}
            >
              {appConfig?.root_path || t("app_config.root_path.placeholder")}
            </Button>
          </Col>
          <Form.Label column sm={FORM_DESCRIPTION_WIDTH}>
            {t("app_config.root_path.description")}
          </Form.Label>
        </Form.Group>
      </Form>
    </Stack>
  );
}
