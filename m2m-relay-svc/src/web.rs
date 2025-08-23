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

use sl_messages::relay::Relay;

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
                        // instance_id, setup msg
                        Some(Ok(Message::Binary(msg))) => {
                            let _ = conn.send(msg).await;
                        }
                        
                        // p1 send vk to p2
                        // Some(Ok(Message::Text(msg))) => {
                        //     println!("recv text msg: {msg}");
                        //     let _ = conn.send(msg.into()).await;
                        // }

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