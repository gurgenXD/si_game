use axum::{http::StatusCode, routing::get, Json, Router};
use std::net::SocketAddr;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

use crate::presentation::web_api::controllers::packages::{
    create_package, get_package, get_packages,
};

pub async fn run() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    // .route(
    //     "/ws/:game_uuid",
    //     get(ws_handler).with_state(Arc::new(WsState::new())),
    // )

    let app = Router::new()
        .route("/packages", get(get_packages).post(create_package))
        .route("/packages/:uuid", get(get_package));

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    debug!("Listening for requests on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
