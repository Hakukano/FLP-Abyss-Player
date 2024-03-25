use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{Extension, Router};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

use crate::{utils::cli::CLI, views::player::server::service::playlist::Playlist};

mod playlists;

#[derive(Clone)]
pub struct HttpServer {
    pub playlist: Arc<dyn Playlist>,
}

impl HttpServer {
    pub async fn run(&self, bind_host: &str, bind_port: u16) -> Result<()> {
        let app = Router::new()
            .nest_service(
                "/",
                ServeDir::new(format!("{}/static", CLI.assets_path)).not_found_service(
                    ServeFile::new(format!("{}/static/index.html", CLI.assets_path)),
                ),
            )
            .nest("/playlists", playlists::route())
            .layer(Extension(self.clone()));
        let addr = SocketAddr::new(bind_host.parse().expect("Invalid bind host"), bind_port);
        axum::serve(
            TcpListener::bind(&addr).await.expect("Cannot bind address"),
            app.into_make_service(),
        )
        .await
        .expect("Cannot build axum server");
        Ok(())
    }
}
