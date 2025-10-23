use backend_rust::cli::run_cli;
use tracing::{Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configurar logging (solo errores para el CLI)
    tracing_subscriber::fmt()
        .with_max_level(Level::ERROR)
        .with_target(false)
        .without_time()
        .init();

    // Cargar variables de entorno
    dotenvy::dotenv().ok();

    // Ejecutar CLI
    if let Err(e) = run_cli().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
