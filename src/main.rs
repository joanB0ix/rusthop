use std::sync::Arc;

use clap::{Parser, ValueEnum, arg, command};
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt};

use rusthop::{
    application::ShortenerService, http::HttpServer, id::NanoIdGenerator, ports::UrlRepository,
};

#[cfg(feature = "memory")]
use rusthop::infra_memory::InMemoryRepo;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(long, default_value = "0.0.0.0:8080")]
    listen: std::net::SocketAddr,

    #[arg(value_enum, default_value_t = Backend::Memory)]
    backend: Backend,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Backend {
    Memory,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    match args.backend {
        Backend::Memory => {
            #[cfg(feature = "memory")]
            {
                let repo = InMemoryRepo::default();
                start(repo, args.listen).await;
            }
            #[cfg(not(feature = "memory"))]
            panic!("compiled without `memory` feature");
        }
    }
    Ok(())
}

async fn start<R: UrlRepository + Clone + Send + Sync + 'static>(
    repo: R,
    listen: std::net::SocketAddr,
) {
    let generator = Arc::new(NanoIdGenerator);
    let svc = ShortenerService::new(repo, generator);
    let server = HttpServer::new(svc);
    let router = server.router();

    tracing::info!(%listen, "Listening");

    let listener = TcpListener::bind(listen).await.expect("Failed to bind");
    tracing::info!(addr = %listener.local_addr().unwrap(), "Listening");

    axum::serve(listener, router).await.expect("Server failed");
}
