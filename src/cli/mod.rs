pub mod categoria_commands;
pub mod commands;
pub mod retiro_commands;
pub mod transaccion_commands;

use crate::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "retiros")]
#[command(about = "Sistema de Gestión Financiera para Retiros")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Gestión de categorías de ingresos y gastos
    #[command(subcommand)]
    Categoria(categoria_commands::CategoriaCommands),
    /// Gestión de retiros y eventos
    #[command(subcommand)]
    Retiro(retiro_commands::RetiroCommands),
    /// Gestión de transacciones financieras
    #[command(subcommand)]
    Transaccion(transaccion_commands::TransaccionCommands),
}

pub async fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Categoria(categoria_cmd) => {
            categoria_commands::handle_categoria_command(categoria_cmd).await
        }
        Commands::Retiro(retiro_cmd) => retiro_commands::handle_retiro_command(retiro_cmd).await,
        Commands::Transaccion(transaccion_cmd) => {
            transaccion_commands::handle_transaccion_command(transaccion_cmd).await
        }
    }
}
