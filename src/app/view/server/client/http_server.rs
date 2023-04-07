use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{Extension, Router};
use tower_http::services::{ServeDir, ServeFile};

use crate::app::view::server::service::playlist::Playlist;

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
                ServeDir::new("assets/static")
                    .not_found_service(ServeFile::new("assets/static/index.html")),
            )
            .nest("/playlists", playlists::route())
            .layer(Extension(self.clone()));
        let addr = SocketAddr::new(bind_host.parse().expect("Invalid bind host"), bind_port);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("Cannot build axum server");
        Ok(())
    }
}
