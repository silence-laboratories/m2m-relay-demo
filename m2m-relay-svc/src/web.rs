// Copyright (c) Silence Laboratories Pte. Ltd. All Rights Reserved.
// This software is licensed under the Silence Laboratories License Agreement.

use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocketUpgrade},
        State,
    },
    response::Response,
};

use futures_util::{SinkExt, StreamExt};

use crate::Inner;

pub async fn handler(
    State(state): State<Arc<Inner>>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(|mut socket| async move {
        let mut conn = state.relay.connect();

        loop {
            tokio::select! {
                Some(msg) = conn.next() => {
                    if socket.send(Message::binary(msg)).await.is_err() {
                        break;
                    }
                }

                msg = socket.recv() => {
                    match msg {
                        None => break,
                        Some(Ok(Message::Binary(msg))) => {
                            println!("relay recv binary msg: {:?}", msg.len());
                            let _ = conn.send(msg.to_vec()).await;
                        }

                        Some(Ok(Message::Close(_))) => {
                            tracing::debug!("recv close from the client");
                            break;
                        }

                        Some(Err(err)) => {
                            tracing::error!("recv error {err}");
                            break;
                        }

                        _ => {}
                    }
                }
            }
        }

        tracing::info!("close ws connection");
    })
}