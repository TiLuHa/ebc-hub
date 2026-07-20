use std::sync::Arc;

use color_eyre::eyre::{Context, Result};
use tokio::{net::TcpListener, signal};
use tracing_subscriber::EnvFilter;

use ebc_hub::{
    db_access::Storage,
    web::{self, AppState},
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing();
    dotenvy::dotenv()?;

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://ebc-hub.db".to_owned());

    let listen_address =
        std::env::var("EBC_HUB_LISTEN").unwrap_or_else(|_| "0.0.0.0:3000".to_owned());

    tracing::info!(database_url, "opening database");

    let storage = Storage::connect(&database_url)
        .await
        .wrap_err("failed to open EBC Hub storage")?;

    let state = AppState {
        storage: Arc::new(storage),
    };

    let app = web::router(state);

    let listener = TcpListener::bind(&listen_address)
        .await
        .wrap_err_with(|| format!("failed to bind HTTP server to {listen_address}"))?;

    tracing::info!(
        address = %listen_address,
        "EBC Hub web server started"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .wrap_err("EBC Hub HTTP server failed")?;

    tracing::info!("EBC Hub stopped");

    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("ebc_hub=debug,tower_http=info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};

        let mut signal =
            signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");

        signal.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            tracing::info!("received Ctrl+C");
        }

        () = terminate => {
            tracing::info!("received termination signal");
        }
    }
}
