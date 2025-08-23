use std::{env, future::IntoFuture, net::ToSocketAddrs, sync::Arc};

use axum::{routing::get, Router};
use tokio::{
    signal::unix::{signal, SignalKind},
    task::JoinSet,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use sl_messages::relay::SimpleMessageRelay;

pub mod flags;
mod web;

pub use flags::MsgRelaySvc;

struct Inner {
    pub(crate) relay: SimpleMessageRelay,
}

async fn health_check() -> &'static str {
    "ok"
}

pub async fn start_service(flags: MsgRelaySvc) -> anyhow::Result<()> {
    let mut servers = JoinSet::new();
    let state = {
        let relay = SimpleMessageRelay::new();

        Arc::new(Inner { relay })
    };

    let app = Router::new()
        .route("/", get(health_check))
        .route("/v1/msg-relay", get(web::handler))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    for addrs in env::var("LISTEN")
        .ok()
        // NOTE: Not using localhost:8080 as default listen address, needs to be set explicitly if
        // needed.
        .unwrap_or_else(|| String::from(""))
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .chain(flags.listen.iter().map(|s| s.as_str()))
    {
        for addr in addrs.to_socket_addrs()? {
            let listener = tokio::net::TcpListener::bind(addr).await?;

            tracing::info!("listening on {}", listener.local_addr()?);

            servers.spawn(
                axum::serve(listener, app.clone().into_make_service())
                    .with_graceful_shutdown(shutdown())
                    .into_future(),
            );
        }
    }

    while servers.join_next().await.is_some() {}

    Ok(())
}

async fn shutdown() {
    let sigint = async {
        signal(SignalKind::interrupt())
            .expect("cant install SIGINT")
            .recv()
            .await;
    };

    let sigterm = async {
        signal(SignalKind::terminate())
            .expect("cant install SIGTERM")
            .recv()
            .await;
    };

    tokio::select! {
        _ = sigint => {},
        _ = sigterm => {},
    };
}
