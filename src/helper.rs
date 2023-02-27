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

pub fn seconds_to_h_m_s(seconds: u32) -> (u32, u32, u32) {
    let h = seconds / 3600;
    let h_remainder = seconds % 3600;
    let m = h_remainder / 60;
    let s = h_remainder % 60;
    (h, m, s)
}
