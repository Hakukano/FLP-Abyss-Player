import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import Col from "react-bootstrap/Col";
import Form from "react-bootstrap/Form";
import Row from "react-bootstrap/Row";
import Stack from "react-bootstrap/Stack";

import { AppConfigBrief } from "../services/api/app_config.ts";
import { ApiServices } from "../services/api.ts";
import translations from "../translations.ts";
import { ErrorModal, useError } from "./error_modal.tsx";

const FORM_LABEL_WIDTH = 3;
const FORM_INPUT_WIDTH = 9;

interface Props {
  apiServices: ApiServices;
}

export default function AppConfig(props: Props) {
  const { t } = useTranslation();

  const [appConfig, setAppConfig] = useState<AppConfigBrief | null>(null);

  const errorState = useError();

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

  useEffect(() => {
    fetchAppConfig().catch(errorState.popup);
  }, []);

  return (
    <Stack gap={3}>
      <ErrorModal state={errorState} />
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
                setLocale(event.target.value).catch(errorState.popup);
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
      </Form>
    </Stack>
  );
}
