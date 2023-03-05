use std::{
    collections::VecDeque,
    fmt::Display,
    path::Path,
    process::{Child, Command},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
};

use anyhow::Result;
use eframe::{
    egui::{self, TextStyle},
    epaint::Color32,
};
use passwords::PasswordGenerator;
use reqwest::{Client, Method};
use serde::Serialize;
use tokio::runtime;

use crate::{
    font::gen_rich_text,
    helper::{find_available_port, seconds_to_h_m_s},
};

fn gen_vlc_http_request_url_base(port: u16, extra_path: impl Display) -> String {
    format!("http://{VLC_HTTP_HOST}:{}/requests{}", port, extra_path)
}

mod status {
    use std::collections::VecDeque;

    use serde::{Deserialize, Serialize};

    #[derive(Default, Serialize, Deserialize)]
    #[serde(rename = "info")]
    pub struct Info {
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(rename = "$value", default)]
        pub value: String,
    }

    #[derive(Default, Serialize, Deserialize)]
    #[serde(rename = "category")]
    pub struct Category {
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(default)]
        pub info: Vec<Info>,
    }

    #[derive(Default, Serialize, Deserialize)]
    #[serde(rename = "information")]
    pub struct Information {
        #[serde(default)]
        pub category: Vec<Category>,
    }

    #[derive(Default, Serialize, Deserialize)]
    #[serde(rename = "root")]
    pub struct Root {
        #[serde(skip_deserializing)]
        pub error: VecDeque<String>,

        pub fullscreen: bool,
        pub aspectratio: Option<String>,
        pub audiodelay: f32,
        pub apiversion: u32,
        pub currentplid: i32,
        pub time: u32,
        pub volume: u32,
        pub length: u32,
        pub random: bool,
        pub rate: u32,
        pub state: String,
        #[serde(rename = "loop")]
        pub lop: bool,
        pub version: String,
        pub position: f32,
        pub repeat: bool,
        pub subtitledelay: u32,
        pub information: Information,
    }
}

pub struct VideoPlayer {
    command: Command,
    http_password: String,
    http_port: u16,
    child: Option<Child>,

    runtime: runtime::Runtime,
    /// The video has been played at least once or is still being played
    played: Arc<AtomicBool>,
    status: Arc<RwLock<status::Root>>,
}

impl VideoPlayer {
    pub fn new(
        player_path: impl AsRef<Path>,
        video_path: impl AsRef<Path>,
        ctx: egui::Context,
    ) -> Self {
        let pg = PasswordGenerator {
            length: 16,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: false,
            spaces: false,
            exclude_similar_characters: false,
            strict: true,
        };
        let http_password = pg.generate_one().expect("Cannot generate password");
        let http_port = find_available_port().expect("Cannot find an available port");
        let mut command = Command::new(player_path.as_ref());
        command
            .arg("--extraintf")
            .arg("http")
            .arg("--http-host")
            .arg(VLC_HTTP_HOST)
            .arg("--http-port")
            .arg(http_port.to_string())
            .arg("--http-password")
            .arg(http_password.as_str())
            .arg(video_path.as_ref());
        let runtime = runtime::Builder::new_multi_thread()
            .worker_threads(RUNTIME_THREADS)
            .thread_name(RUNTIME_THREAD_NAME)
            .thread_stack_size(RUNTIME_THREAD_STACK_SIZE)
            .enable_all()
            .build()
            .expect("Cannot build tokio runtime");
        let video_player = Self {
            command,
            http_password,
            http_port,
            child: None,
            runtime,
            played: Arc::new(AtomicBool::new(false)),
            status: Arc::new(RwLock::new(status::Root::default())),
        };

        let request_url = gen_vlc_http_request_url_base(video_player.http_port, VLC_HTTP_STATUS);
        let request_password = video_player.http_password.clone();
        let played = video_player.played.clone();
        let status = video_player.status.clone();
        video_player.runtime.spawn(async move {
            let mut timer =
                tokio::time::interval(std::time::Duration::from_millis(STATUS_SYNC_INTERVAL_MS));
            loop {
                timer.tick().await;
                let request = Client::new()
                    .request(Method::GET, request_url.as_str())
                    .basic_auth("", Some(request_password.as_str()));
                let ctx = ctx.clone();
                let played = played.clone();
                let status = status.clone();
                let _ = tokio::spawn(async move {
                    match request.send().await {
                        Err(_) => {
                            let mut queue = VecDeque::new();
                            queue.push_back("Waiting for the stream...".to_string());
                            status.write().expect("Cannot get status write lock").error = queue;
                        }
                        Ok(response) => match response.text().await {
                            Err(err) => {
                                status
                                    .write()
                                    .expect("Cannot get status write lock")
                                    .error
                                    .push_back(err.to_string());
                            }
                            Ok(text) => {
                                match quick_xml::de::from_str::<status::Root>(text.as_str()) {
                                    Err(err) => {
                                        status
                                            .write()
                                            .expect("Cannot get status write lock")
                                            .error
                                            .push_back(err.to_string());
                                    }
                                    Ok(xml) => {
                                        if xml.state == "playing" || xml.state == "paused" {
                                            played.swap(true, Ordering::AcqRel);
                                        }
                                        *status.write().expect("Cannot get status write lock") = xml
                                    }
                                }
                            }
                        },
                    }
                    ctx.request_repaint();
                })
                .await;
            }
        });

        video_player
    }

    fn send_status_get_request(&self, query: Vec<(String, String)>) {
        let request_url = gen_vlc_http_request_url_base(self.http_port, VLC_HTTP_STATUS);
        let request_password = self.http_password.clone();
        self.runtime.spawn(async move {
            let _ = Client::new()
                .request(Method::GET, request_url.as_str())
                .query(query.as_slice())
                .basic_auth("", Some(request_password.as_str()))
                .send()
                .await;
        });
    }
}

impl Drop for VideoPlayer {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
        }
    }
}

impl super::VideoPlayer for VideoPlayer {
    fn is_end(&self) -> bool {
        self.played.load(Ordering::Acquire)
            && self.status.read().expect("Cannot get status lock").state == "stopped"
    }

    fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let status = self.status.read().expect("Cannot get status read lock");

        ui.spacing_mut().item_spacing = egui::vec2(0.0, 2.0);

        if !status.error.is_empty() {
            status.error.iter().for_each(|err| {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label(gen_rich_text(
                        ctx,
                        err,
                        TextStyle::Body,
                        Some(Color32::LIGHT_RED),
                    ));
                });
            });
            return;
        }

        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(gen_rich_text(
                ctx,
                format!(
                    "Now playing: {}",
                    status
                        .information
                        .category
                        .iter()
                        .find(|c| c.name == "meta")
                        .map(|c| c
                            .info
                            .iter()
                            .find(|i| i.name == "filename")
                            .map(|i| i.value.clone())
                            .unwrap_or_else(|| "No filename found".to_string()))
                        .unwrap_or_else(|| "No meta found".to_string())
                ),
                TextStyle::Body,
                None,
            ));
        });

        let (time_h, time_m, time_s) = seconds_to_h_m_s(status.time);
        let (length_h, length_m, length_s) = seconds_to_h_m_s(status.length);
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(gen_rich_text(
                ctx,
                format!(
                    "Time: {:02}:{:02}:{:02} / {:02}:{:02}:{:02}",
                    time_h, time_m, time_s, length_h, length_m, length_s
                ),
                TextStyle::Body,
                None,
            ));
        });

        let mut xml_serializer = quick_xml::se::Serializer::new(String::new());
        xml_serializer.indent(' ', 2);
        let information = status
            .serialize(xml_serializer)
            .expect("Cannot serialize information");
        ui.with_layout(
            egui::Layout::left_to_right(egui::Align::TOP).with_cross_justify(true),
            |ui| {
                ui.add_space(10.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(information.as_str());
                });
            },
        );
    }

    fn start(&mut self) -> Result<()> {
        if self.child.is_none() {
            self.child.replace(self.command.spawn()?);
        }
        Ok(())
    }

    fn pause(&mut self) -> Result<()> {
        self.send_status_get_request(vec![("command".to_string(), "pl_pause".to_string())]);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            child.kill()?;
        }
        Ok(())
    }

    fn fast_forward(&mut self, seconds: u32) -> Result<()> {
        self.send_status_get_request(vec![
            ("command".to_string(), "seek".to_string()),
            ("val".to_string(), format!("+{seconds}")),
        ]);
        Ok(())
    }

    fn rewind(&mut self, seconds: u32) -> Result<()> {
        self.send_status_get_request(vec![
            ("command".to_string(), "seek".to_string()),
            ("val".to_string(), format!("-{seconds}")),
        ]);
        Ok(())
    }
}
