use std::net::TcpListener;

use eframe::egui::Vec2;

pub fn find_available_port() -> Option<u16> {
    (40000..40100).find(|port| port_is_available(*port))
}

pub fn port_is_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

pub fn seconds_to_h_m_s(seconds: u32) -> (u32, u32, u32) {
    let h = seconds / 3600;
    let h_remainder = seconds % 3600;
    let m = h_remainder / 60;
    let s = h_remainder % 60;
    (h, m, s)
}

pub fn scale_fit_all(max_size: Vec2, origin_size: Vec2) -> Vec2 {
    let scale_x = max_size.x / origin_size.x;
    let scale_y = max_size.y / origin_size.y;
    let scale = scale_x.min(scale_y);
    Vec2::new(origin_size.x * scale, origin_size.y * scale)
}

#[allow(dead_code)]
pub fn scale_fit_height(max_height: f32, origin_size: Vec2) -> Vec2 {
    let scale = max_height / origin_size.y;
    Vec2::new(origin_size.x * scale, origin_size.y * scale)
}
