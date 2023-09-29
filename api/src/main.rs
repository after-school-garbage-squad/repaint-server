use std::net::{SocketAddr, SocketAddrV4};
use std::time::Duration;

use axum::body::Body;
use axum::http::Request;
use axum::Router;
use cfg_if::cfg_if;
use repaint_server_core::SeaOrm;
use repaint_server_fcm::Fcm;
use repaint_server_firestore::Firestore;
use repaint_server_gcs::Gcs;
use repaint_server_otp::Otp;
use repaint_server_pubsub::PubSub;
use repaint_server_usecase::usecase::admin::AdminUsecaseImpl;
use repaint_server_usecase::usecase::event::EventUsecaseImpl;
use repaint_server_usecase::usecase::image::ImageUsecaseImpl;
use repaint_server_usecase::usecase::palette::PaletteUsecaseImpl;
use repaint_server_usecase::usecase::spot::SpotUsecaseImpl;
use repaint_server_usecase::usecase::traffic::TrafficUsecaseImpl;
use repaint_server_usecase::usecase::visitor::VisitorUsecaseImpl;
use sentry::integrations::tower as sentry_tower;
use teloc::{Resolver, ServiceProvider};
use tokio::signal;
use tokio::sync::oneshot;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::routes::admin::admin;
use crate::routes::healthz::healthz;
use crate::routes::license::license;
use crate::routes::version::version;
use crate::routes::visitor::visitor;
use crate::utils::{envvar, envvar_str};

mod middleware;
mod routes;
mod utils;

#[cfg(debug_assertions)]
const VERSION: &str = "DEVELOPMENT BUILD";

#[cfg(not(debug_assertions))]
const VERSION: &str = env!("GIT_HASH");
#[cfg(not(debug_assertions))]
static_assertions::const_assert!(VERSION.len() == 7);

fn main() {
    dotenvy::dotenv().ok();
    let _ = sentry::init((
        envvar_str("SENTRY_DSN", None),
        sentry::ClientOptions {
            release: Some(VERSION.into()),
            traces_sample_rate: 1.0,
            environment: Some(envvar_str("SENTRY_ENVIRONMENT", "development").into()),
            ..Default::default()
        },
    ));
    start();
}

#[tokio::main]
async fn start() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(sentry_tracing::layer())
        .init();

    let addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(
        envvar("LISTEN_IP_ADDR", Some([127, 0, 0, 1].into())),
        envvar("PORT", 4000),
    ));
    let db_url = envvar_str("DATABASE_URL", None);
    let db = SeaOrm::new(db_url).await.expect("couldn't connect to db");

    let container = ServiceProvider::new()
        .add_singleton::<SeaOrm>()
        .add_singleton::<FcmProvider>()
        .add_singleton::<FirestoreProvider>()
        .add_singleton::<GcsProvider>()
        .add_singleton::<OtpProvider>()
        .add_singleton::<PubSubProvider>()
        .add_singleton::<EmailProvider>()
        .add_transient::<AdminUsecaseImpl<SeaOrm, FirestoreProvider, EmailProvider>>()
        .add_transient::<EventUsecaseImpl<SeaOrm, FirestoreProvider>>()
        .add_transient::<ImageUsecaseImpl<SeaOrm, GcsProvider, OtpProvider, PubSubProvider>>()
        .add_transient::<PaletteUsecaseImpl<SeaOrm, FirestoreProvider, PubSubProvider>>()
        .add_transient::<SpotUsecaseImpl<SeaOrm, FirestoreProvider>>()
        .add_transient::<TrafficUsecaseImpl<SeaOrm, FirestoreProvider, FcmProvider>>()
        .add_transient::<VisitorUsecaseImpl<SeaOrm, FirestoreProvider>>()
        .add_instance(db)
        .add_instance(fcm_provider().await)
        .add_instance(firestore_provider().await)
        .add_instance(gcs_provider().await)
        .add_instance(otp_provider())
        .add_instance(pubsub_provider().await)
        .add_instance(email_provider());

    let admin_usecase: AdminUsecaseImpl<_, _, _> = container.resolve();
    let event_usecase: EventUsecaseImpl<_, _> = container.resolve();
    let image_usecase: ImageUsecaseImpl<_, _, _, _> = container.resolve();
    let palette_usecase: PaletteUsecaseImpl<_, _, _> = container.resolve();
    let spot_usecase: SpotUsecaseImpl<_, _> = container.resolve();
    let traffic_usecase: TrafficUsecaseImpl<_, _, _> = container.resolve();
    let visitor_usecase: VisitorUsecaseImpl<_, _> = container.resolve();

    let app = Router::new()
        .nest(
            "/admin",
            admin(
                admin_usecase,
                event_usecase,
                traffic_usecase,
                spot_usecase,
                image_usecase.clone(),
            ),
        )
        .nest(
            "/visitor",
            visitor(visitor_usecase, palette_usecase, image_usecase),
        )
        .nest("/healthz", healthz())
        .nest("/version", version())
        .nest("/license", license())
        .layer(sentry_tower::NewSentryLayer::<Request<Body>>::new_from_top())
        .layer(sentry_tower::SentryHttpLayer::with_transaction());

    tracing::info!("staring server at {addr}");

    let (send, recv) = oneshot::channel();

    tokio::select! {
        _ = axum::Server::bind(&addr).serve(app.into_make_service()).with_graceful_shutdown(wait_for_shutdown_signal(send)) => {}

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

type FcmProvider = Fcm;
type FirestoreProvider = Firestore;
type GcsProvider = Gcs;
type OtpProvider = Otp;
type PubSubProvider = PubSub;

async fn fcm_provider() -> FcmProvider {
    let project_id = envvar_str("PROJECT_ID", None);

    Fcm::new(project_id).await
}

async fn firestore_provider() -> FirestoreProvider {
    let project_id = envvar_str("PROJECT_ID", None);

    Firestore::new(project_id).await
}

async fn gcs_provider() -> GcsProvider {
    let bucket = envvar_str("BUCKET", None);

    Gcs::new(bucket).await
}

fn otp_provider() -> OtpProvider {
    let bucket = envvar_str("BUCKET", None);
    let origin = envvar_str("ORIGIN", None);

    Otp::new(bucket, origin)
}

async fn pubsub_provider() -> PubSubProvider {
    let cluster = envvar("CLUSTER", None);
    let clustering_topic = envvar_str("CLUSTERING_TOPIC", None);
    let merge_topic = envvar_str("MERGE_TOPIC", None);

    PubSub::new(cluster, clustering_topic, merge_topic).await
}

cfg_if! {
    if #[cfg(feature = "email_sg")] {
        use repaint_server_sg::SendGrid;

        type EmailProvider = SendGrid;

        fn email_provider() -> EmailProvider {
            let api_key = envvar_str("SENDGRID_API_KEY", None);
            let send_from = envvar_str("SENDGRID_SEND_FROM", None);
            let url = envvar_str("SENDGRID_URL", None);

            SendGrid::new(api_key, send_from, url)
        }
    } else if #[cfg(feature = "email_gmail")] {
        use repaint_server_gmail::Gmail;

        type EmailProvider = Gmail;

        fn email_provider() -> EmailProvider {
            let send_from = envvar_str("GMAIL_SEND_FROM", None);
            let url = envvar_str("GMAIL_URL", None);
            let username = envvar_str("SMTP_USERNAME", None);
            let password = envvar_str("SMTP_PASSWORD", None);

            Gmail::new(send_from, url, username, password)
        }
    } else {
        compile_error!("you must set one email provider");
    }
}
