import React from "react";
import { useTranslation } from "react-i18next";

import Col from "react-bootstrap/Col";
import Form from "react-bootstrap/Form";
import Row from "react-bootstrap/Row";
import Stack from "react-bootstrap/Stack";

import { AppConfigService } from "../services/api/app_config.ts";
import i18next from "i18next";

interface Props extends React.HTMLAttributes<HTMLElement> {
  appConfigService: AppConfigService;
}

export default function Config(props: Props) {
  const { t } = useTranslation();

  return (
    <Stack gap={2}>
      <h2>{t("app_config.title")}</h2>
      <Form>
        <Form.Group as={Row} className="mb-3" controlId="app-config-locale">
          <Form.Label column sm={2}>
            {t("app_config.locale.name")}
          </Form.Label>
          <Col sm={10}>
            <Form.Select>
              <option>-</option>
              {i18next.languages.map((language) => (
                <option key={language} value={language}>
                  {language}
                </option>
              ))}
            </Form.Select>
          </Col>
        </Form.Group>
      </Form>
    </Stack>
  );
}
