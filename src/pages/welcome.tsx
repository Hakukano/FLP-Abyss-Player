import { useTranslation } from "react-i18next";

import Button from "react-bootstrap/Button";
import Card from "react-bootstrap/Card";
import Col from "react-bootstrap/Col";
import Container from "react-bootstrap/Container";
import Row from "react-bootstrap/Row";
import Stack from "react-bootstrap/Stack";
import AppConfig from "../components/app_config.tsx";
import { ApiServices } from "../services/api.ts";
import { Link, useNavigate } from "react-router-dom";

interface Props {
  apiServices: ApiServices;
}

export default function Welcome(props: Props) {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const gotoPlayer = () => {
    navigate("/player");
  };

  return (
    <Container
      fluid
      className="vh-100 d-flex justify-content-center align-items-center"
    >
      <Row className="w-50">
        <Col>
          <Card>
            <Card.Header>{t("app_name")}</Card.Header>
            <Card.Body>
              <Stack gap={3}>
                <AppConfig apiServices={props.apiServices} />
                <Row>
                  <Col md={6}>
                    <Button
                      className="w-100"
                      variant="warning"
                      onClick={gotoPlayer}
                    >
                      {t("new_session")}
                    </Button>
                  </Col>
                  <Col md={6}>
                    <Button
                      className="w-100"
                      variant="info"
                      onClick={gotoPlayer}
                    >
                      {t("load_session")}
                    </Button>
                  </Col>
                </Row>
              </Stack>
            </Card.Body>
          </Card>
        </Col>
      </Row>
    </Container>
  );
}
