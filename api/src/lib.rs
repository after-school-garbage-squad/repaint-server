use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::time::Duration;

use axum::extract::FromRef;
use repaint_server_core::SeaOrm;
use teloc::ServiceProvider;
use tokio::signal;
use tokio::sync::oneshot;

mod middleware;
mod routes;

#[cfg(debug_assertions)]
const LOG_LEVEL: &str = "debug,sqlx=warn";
#[cfg(not(debug_assertions))]
const LOG_LEVEL: &str = "info,sqlx=warn";

#[derive(Debug, Clone, FromRef)]
struct AppState {}

pub async fn run() {
    let addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(
        envvar("LISTEN_IP_ADDR", Some([127, 0, 0, 1].into())),
        envvar("PORT", 4000),
    ));
    let db_url = envvar_str("DATABASE_URL", None);
    let base_url = envvar_str("AUTHORITY", None);
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

fn envvar<T, E>(var_name: &str, default: impl Into<Option<T>>) -> T
where
    T: std::fmt::Debug + FromStr<Err = E>,
    E: std::fmt::Debug,
{
    if let Ok(v) = std::env::var(var_name) {
        if v.is_empty() {
            panic!("{var_name} is set, but it's empty");
        }

        return v
            .parse()
            .unwrap_or_else(|e| panic!("environment variable {var_name} is not valid: {e:?}"));
    }

    if !cfg!(debug_assertions) {
        panic!("{var_name} is not set; Default environment variables are not allowed in the release build");
    }
    let Some(d) = default.into() else {
        panic!("{var_name} is not set");
    };

    tracing::warn!("environment variable {var_name} is not set. defaulting to {d:?}");
    d
}

fn envvar_str<'a>(var_name: &str, default: impl Into<Option<&'a str>>) -> String {
    if let Ok(v) = std::env::var(var_name) {
        if v.is_empty() {
            panic!("{var_name} is set, but it's empty");
        }

        return v;
    }
    if !cfg!(debug_assertions) {
        panic!("{var_name} is not set; Default environment variables are not allowed in the release build");
    }
    let Some(d) = default.into() else {
        panic!("{var_name} is not set");
    };
    tracing::warn!("environment variable {var_name} is not set. defaulting to {d:?}");

    d.to_owned()
}
