import { useTranslation } from "react-i18next";

import Container from "react-bootstrap/Container";
import Nav from "react-bootstrap/Nav";
import Navbar from "react-bootstrap/Navbar";
import { Link, Outlet } from "react-router-dom";
import Stack from "react-bootstrap/Stack";

export default function Layout() {
  const { t } = useTranslation();

  return (
    <Container fluid className="vh-100">
      <Stack gap={4}>
        <Navbar expand="lg" className="bg-body-tertiary">
          <Container>
            <Navbar.Brand>{t("app_name")}</Navbar.Brand>
            <Navbar.Toggle aria-controls="layout-navbar" />
            <Navbar.Collapse id="layout-navbar">
              <Nav className="me-auto">
                <Nav.Link as={Link} to="/config">
                  {t("config")}
                </Nav.Link>
              </Nav>
            </Navbar.Collapse>
          </Container>
        </Navbar>
        <Outlet />
      </Stack>
    </Container>
  );
}
