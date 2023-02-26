use std::net::TcpListener;

pub fn find_available_port() -> Option<u16> {
    (40000..40100).find(|port| port_is_available(*port))
}

pub fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
