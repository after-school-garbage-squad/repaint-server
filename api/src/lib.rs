use std::net::{SocketAddr, SocketAddrV4};
use std::time::Duration;

use repaint_server_core::SeaOrm;
use teloc::ServiceProvider;
use tokio::signal;
use tokio::sync::oneshot;

use crate::utils::{envvar, envvar_str};

mod middleware;
mod routes;
mod utils;

#[cfg(debug_assertions)]
const LOG_LEVEL: &str = "debug,sqlx=warn";
#[cfg(not(debug_assertions))]
const LOG_LEVEL: &str = "info,sqlx=warn";

pub async fn run() {
    let addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(
        envvar("LISTEN_IP_ADDR", Some([127, 0, 0, 1].into())),
        envvar("PORT", 4000),
    ));
    let db_url = envvar_str("DATABASE_URL", None);
    let db = SeaOrm::new(db_url).await.expect("couldn't connect to db");

    let container = ServiceProvider::new()
        .add_singleton::<SeaOrm>()
        .add_instance(db);

    tracing::info!("staring server at {addr}");

    let (send, recv) = oneshot::channel();

    tokio::select! {
        _ = axum::Server::bind(&addr).serve().with_graceful_shutdown(wait_for_shutdown_signal(send)) => {}

        _ = recv => {
            tracing::info!("graceful shutdown has timed out. forcing shutdown.");
        }
    }
}

async fn wait_for_shutdown_signal(force_shutdown_tx: oneshot::Sender<()>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install terminate signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("exit signal received, trying graceful shutdown");

    // workaround for https://github.com/hyperium/hyper/issues/2787
    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(3)).await;
        force_shutdown_tx.send(()).unwrap();
    });
}
