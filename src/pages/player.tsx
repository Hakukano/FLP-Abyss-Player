import { ApiServices } from "../services/api";
import { Col, Container, Row } from "react-bootstrap";
import Playlist from "../components/playlist";

interface Props {
  apiServices: ApiServices;
}

export default function Player(props: Props) {
  return (
    <Container fluid className="vh-100 d-flex m-3">
      <Row className="w-100">
        <Col md={6}>
          <Playlist apiServices={props.apiServices} />
        </Col>
      </Row>
    </Container>
  );
}
