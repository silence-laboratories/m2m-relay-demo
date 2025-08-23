// Copyright (c) Silence Laboratories Pte. Ltd. All Rights Reserved.
// This software is licensed under the Silence Laboratories License Agreement.

use dotenvy::dotenv;

use m2m_relay_svc::{start_service, MsgRelaySvc};
#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    // try to load .env and do not complain if one is not found.
    let _ = dotenv();

    let flags = MsgRelaySvc::from_env_or_exit();

    tracing_subscriber::fmt::init();

    start_service(flags).await?;

    Ok(())
}
