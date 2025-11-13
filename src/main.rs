use std::env;

#[cfg(not(feature = "desktop"))]
use backend_rust::cli::run_cli;

#[cfg(feature = "desktop")]
use backend_rust::desktop::run_desktop;

use tracing::Level;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configurar logging
    tracing_subscriber::fmt()
        .with_max_level(Level::ERROR)
        .with_target(false)
        .without_time()
        .init();

    // Cargar variables de entorno
    dotenvy::dotenv().ok();

    // Detectar si ejecutar como CLI o Desktop
    let args: Vec<String> = env::args().collect();
    
    // Si no hay argumentos o el primer argumento es --desktop, ejecutar app desktop
    if args.len() == 1 || (args.len() > 1 && args[1] == "--desktop") {
        #[cfg(feature = "desktop")]
        {
            run_desktop().await?;
        }
        #[cfg(not(feature = "desktop"))]
        {
            eprintln!("Desktop app no está habilitada. Usa 'cargo run --features desktop' o ejecuta con argumentos CLI.");
            std::process::exit(1);
        }
    } else {
    // Ejecutar CLI
        #[cfg(not(feature = "desktop"))]
        {
    if let Err(e) = run_cli().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
            }
        }
        #[cfg(feature = "desktop")]
        {
            eprintln!("En modo desktop, usa sin argumentos para la app gráfica.");
            std::process::exit(1);
        }
    }

    Ok(())
}
