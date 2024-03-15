import { useTranslation } from "react-i18next";
import { ApiServices } from "../services/api";
import { Col, Container, Row, Stack } from "react-bootstrap";
import Playlist from "../components/playlist";

interface Props {
  apiServices: ApiServices;
}

export default function Player(props: Props) {
  const { t } = useTranslation();

  return (
    <Container fluid className="vh-100 d-flex m-3">
      <Row className="w-100">
        <Col md={6}>
          <Stack gap={3}>
            <h2>{t("playlist.title")}</h2>
            <Playlist apiServices={props.apiServices} />
          </Stack>
        </Col>
      </Row>
    </Container>
  );
}
