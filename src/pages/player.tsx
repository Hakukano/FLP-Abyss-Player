import { ApiServices } from "../services/api";
import { Col, Container, Row } from "react-bootstrap";
import Group from "../components/group";
import { useSearchParams } from "react-router-dom";

interface Props {
  apiServices: ApiServices;
}

export default function Player(props: Props) {
  const [searchParams, _] = useSearchParams();

  const playlistId = searchParams.get("playlist_id");

  return (
    <Container fluid className="vh-100 d-flex p-3">
      <Row className="w-100">
        <Col md={6}>
          {playlistId && <Group apiServices={props.apiServices} />}
        </Col>
        <Col md={6}></Col>
      </Row>
    </Container>
  );
}
