import { useState } from "react";
import { useTranslation } from "react-i18next";

import Button from "react-bootstrap/Button";
import Card from "react-bootstrap/Card";
import Col from "react-bootstrap/Col";
import Container from "react-bootstrap/Container";
import Row from "react-bootstrap/Row";
import Stack from "react-bootstrap/Stack";
import AppConfig from "../components/app_config.tsx";
import {
  AppConfigBrief,
  AppConfigService,
} from "../services/api/app_config.ts";

interface Props {
  appConfigService: AppConfigService;
}

export default function Welcome(props: Props) {
  const { t } = useTranslation();

  const [appConfig, setAppConfig] = useState<AppConfigBrief | null>(null);

  return (
    <Container
      fluid
      className="vh-100 d-flex justify-content-center align-items-center"
    >
      <Row>
        <Col>
          <Card>
            <Card.Header>{t("app_name")}</Card.Header>
            <Card.Body>
              <Stack gap={3}>
                <AppConfig
                  appConfigService={props.appConfigService}
                  appConfigState={[appConfig, setAppConfig]}
                />
                {appConfig ? (
                  appConfig.root_path ? (
                    <Button variant="info">{t("load_playlist")}</Button>
                  ) : (
                    <Button variant="warning">{t("new_playlist")}</Button>
                  )
                ) : (
                  <></>
                )}
              </Stack>
            </Card.Body>
          </Card>
        </Col>
      </Row>
    </Container>
  );
}
