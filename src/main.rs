use axum::{
    routing::get,
    Router,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    info!("initializing router...");

    let app = Router::new().route("/", get(index));
    let port = 3000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    info!("router initialized, now listening on port {}", port);
    
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> String {
    info!("Request");
    return "Hello world!".into();    
}
