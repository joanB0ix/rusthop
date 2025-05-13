use std::sync::Arc;

use clap::{Parser, ValueEnum, arg, command};
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt};

use rusthop::{
    adapters::inbound::http::HttpServer, adapters::outbound::in_memory::InMemoryRepo,
    application::ShortenerService, ports::UrlRepository, shared::id::NanoIdGenerator,
};

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
                start_server(repo, args.listen).await;
            }
            #[cfg(not(feature = "memory"))]
            panic!("compiled without `memory` feature");
        }
    }
    Ok(())
}

async fn start_server<R>(repo: R, listen: std::net::SocketAddr)
where
    R: UrlRepository + Clone + Send + Sync + 'static,
{
    let generator = Arc::new(NanoIdGenerator);
    let svc = ShortenerService::new(repo, generator);
    let router = HttpServer::new(svc).router();

    tracing::info!(%listen, "Listening");

    let listener = TcpListener::bind(listen).await.expect("Failed to bind");
    tracing::info!(addr = %listener.local_addr().unwrap(), "Listening");

    axum::serve(listener, router).await.expect("Server failed");
}
