use chrono::{DateTime, NaiveDateTime, Utc};
use clap::{Args, Subcommand};
use colored::*;
use uuid::Uuid;
use validator::Validate;

use crate::database::Database;
use crate::models::{CreateTransaccion, TipoTransaccion};
use crate::repositories::TransaccionRepository;
use crate::{AppError, Result};

#[derive(Subcommand)]
pub enum TransaccionCommands {
    /// Crear una nueva transacción
    Crear(CrearTransaccionArgs),
    /// Listar transacciones
    Listar(ListarTransaccionArgs),
    /// Mostrar detalles de una transacción
    Mostrar(MostrarTransaccionArgs),
    /// Eliminar una transacción
    Eliminar(EliminarTransaccionArgs),
    /// Calcular balance de un retiro
    Balance(BalanceArgs),
}

#[derive(Args)]
pub struct CrearTransaccionArgs {
    /// ID del retiro
    #[arg(long)]
    pub retiro_id: String,

    /// ID de la categoría
    #[arg(long)]
    pub categoria_id: String,

    /// Tipo de transacción (ingreso/gasto)
    #[arg(short, long, value_enum)]
    pub tipo: CliTipoTransaccion,

    /// Monto de la transacción
    #[arg(short, long)]
    pub monto: f64,

    /// Descripción de la transacción
    #[arg(short, long)]
    pub descripcion: String,

    /// Fecha de la transacción (YYYY-MM-DD HH:MM:SS, opcional - usa ahora por defecto)
    #[arg(short, long)]
    pub fecha: Option<String>,
}

#[derive(Args)]
pub struct ListarTransaccionArgs {
    /// Filtrar por retiro
    #[arg(long)]
    pub retiro_id: Option<String>,

    /// Filtrar por tipo de transacción
    #[arg(short, long, value_enum)]
    pub tipo: Option<CliTipoTransaccion>,

    /// Limitar número de resultados
    #[arg(short, long, default_value = "20")]
    pub limit: usize,
}

#[derive(Args)]
pub struct MostrarTransaccionArgs {
    /// ID de la transacción
    pub id: String,
}

#[derive(Args)]
pub struct EliminarTransaccionArgs {
    /// ID de la transacción a eliminar
    pub id: String,

    /// Confirmar eliminación sin preguntar
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args)]
pub struct BalanceArgs {
    /// ID del retiro para calcular balance
    pub retiro_id: String,
}

#[derive(clap::ValueEnum, Clone)]
pub enum CliTipoTransaccion {
    Ingreso,
    Gasto,
}

impl From<CliTipoTransaccion> for TipoTransaccion {
    fn from(cli_tipo: CliTipoTransaccion) -> Self {
        match cli_tipo {
            CliTipoTransaccion::Ingreso => TipoTransaccion::Ingreso,
            CliTipoTransaccion::Gasto => TipoTransaccion::Gasto,
        }
    }
}

pub async fn handle_transaccion_command(command: TransaccionCommands) -> Result<()> {
    // Conectar a la base de datos
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./retiros.db".to_string());

    let db = Database::new(&database_url).await?;
    let repo = TransaccionRepository::new(db.pool().clone());

    match command {
        TransaccionCommands::Crear(args) => crear_transaccion(repo, args).await,
        TransaccionCommands::Listar(args) => listar_transacciones(repo, args).await,
        TransaccionCommands::Mostrar(args) => mostrar_transaccion(repo, args).await,
        TransaccionCommands::Eliminar(args) => eliminar_transaccion(repo, args).await,
        TransaccionCommands::Balance(args) => calcular_balance(repo, args).await,
    }
}

fn parse_datetime(date_str: &str) -> Result<DateTime<Utc>> {
    // Intentar parsear con formato "YYYY-MM-DD HH:MM:SS"
    if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        return Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
    }

    // Intentar parsear solo fecha "YYYY-MM-DD" (asumiendo 00:00:00)
    if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let naive_dt = naive_date.and_hms_opt(0, 0, 0).unwrap();
        return Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
    }

    Err(AppError::Validation(format!(
        "Formato de fecha inválido: {}. Use YYYY-MM-DD o YYYY-MM-DD HH:MM:SS",
        date_str
    )))
}

async fn crear_transaccion(repo: TransaccionRepository, args: CrearTransaccionArgs) -> Result<()> {
    println!("{}", "💰 Creando nueva transacción...".cyan().bold());

    let retiro_id = Uuid::parse_str(&args.retiro_id)
        .map_err(|_| AppError::Validation("ID de retiro inválido".to_string()))?;

    let categoria_id = Uuid::parse_str(&args.categoria_id)
        .map_err(|_| AppError::Validation("ID de categoría inválido".to_string()))?;

    let fecha = if let Some(fecha_str) = &args.fecha {
        parse_datetime(fecha_str)?
    } else {
        Utc::now()
    };

    let create_data = CreateTransaccion {
        retiro_id,
        categoria_id,
        tipo: args.tipo.into(),
        monto: args.monto,
        descripcion: args.descripcion.clone(),
        fecha,
    };

    // Validar datos antes de crear
    if let Err(e) = create_data.validate() {
        println!("{} {}", "❌ Error de validación:".red().bold(), e);
        return Err(AppError::Validation(e.to_string()));
    }

    match repo.create(create_data).await {
        Ok(transaccion) => {
            println!("{}", "✅ Transacción creada exitosamente!".green().bold());
            println!();
            println!("📋 {}", "Detalles:".bold());
            println!("   ID: {}", transaccion.id.to_string().bright_blue());
            println!(
                "   Tipo: {}",
                format!("{}", transaccion.tipo).bright_yellow()
            );
            println!(
                "   Monto: {}",
                format!("€{:.2}", transaccion.monto).bright_green()
            );
            println!("   Descripción: {}", transaccion.descripcion.bright_white());
            println!(
                "   Fecha: {}",
                transaccion
                    .fecha
                    .format("%Y-%m-%d %H:%M")
                    .to_string()
                    .bright_cyan()
            );
            println!(
                "   Retiro ID: {}",
                transaccion.retiro_id.to_string().bright_magenta()
            );
            println!(
                "   Categoría ID: {}",
                transaccion.categoria_id.to_string().bright_magenta()
            );
        }
        Err(e) => {
            println!("{} {}", "❌ Error creando transacción:".red().bold(), e);
            return Err(e);
        }
    }

    Ok(())
}

async fn listar_transacciones(
    repo: TransaccionRepository,
    args: ListarTransaccionArgs,
) -> Result<()> {
    println!("{}", "📋 Listando transacciones...".cyan().bold());
    println!();

    let transacciones = if let Some(retiro_id_str) = &args.retiro_id {
        let retiro_id = Uuid::parse_str(retiro_id_str)
            .map_err(|_| AppError::Validation("ID de retiro inválido".to_string()))?;
        repo.get_by_retiro(retiro_id).await?
    } else {
        // Para este ejemplo simplificado, no implementamos get_all
        println!(
            "{}",
            "⚠️  Por favor especifica un retiro con --retiro-id".yellow()
        );
        return Ok(());
    };

    if transacciones.is_empty() {
        println!("{}", "📭 No se encontraron transacciones.".yellow());
        return Ok(());
    }

    println!(
        "{:<38} {:<10} {:<12} {:<30} {:<12}",
        "ID".bold(),
        "TIPO".bold(),
        "MONTO".bold(),
        "DESCRIPCIÓN".bold(),
        "FECHA".bold()
    );
    println!("{}", "─".repeat(110).bright_black());

    let mut total_ingresos = 0.0;
    let mut total_gastos = 0.0;
    let mut count = 0;

    for transaccion in &transacciones {
        if count >= args.limit {
            break;
        }

        let tipo_color = match transaccion.tipo {
            TipoTransaccion::Ingreso => {
                total_ingresos += transaccion.monto;
                transaccion.tipo.to_string().green()
            }
            TipoTransaccion::Gasto => {
                total_gastos += transaccion.monto;
                transaccion.tipo.to_string().red()
            }
        };

        let descripcion_truncated = if transaccion.descripcion.len() > 28 {
            format!("{}...", &transaccion.descripcion[..25])
        } else {
            transaccion.descripcion.clone()
        };

        println!(
            "{:<38} {:<10} {:<12} {:<30} {:<12}",
            transaccion.id.to_string().bright_blue(),
            tipo_color,
            format!("€{:.2}", transaccion.monto).bright_green(),
            descripcion_truncated.bright_white(),
            transaccion
                .fecha
                .format("%Y-%m-%d")
                .to_string()
                .bright_cyan(),
        );

        count += 1;
    }

    println!();
    println!("{}", "📊 Resumen:".bold());
    println!(
        "   Total ingresos: {}",
        format!("€{:.2}", total_ingresos).green()
    );
    println!("   Total gastos: {}", format!("€{:.2}", total_gastos).red());
    println!(
        "   Balance: {}",
        format!("€{:.2}", total_ingresos - total_gastos).bright_yellow()
    );
    println!(
        "   Transacciones mostradas: {}/{}",
        count,
        transacciones.len()
    );

    Ok(())
}

async fn mostrar_transaccion(
    repo: TransaccionRepository,
    args: MostrarTransaccionArgs,
) -> Result<()> {
    println!("{}", "🔍 Buscando transacción...".cyan().bold());

    let id =
        Uuid::parse_str(&args.id).map_err(|_| AppError::Validation("ID inválido".to_string()))?;

    match repo.get_by_id(id).await? {
        Some(transaccion) => {
            println!("{}", "✅ Transacción encontrada!".green().bold());
            println!();
            println!("📋 {}", "Detalles completos:".bold());
            println!("   ID: {}", transaccion.id.to_string().bright_blue());
            println!(
                "   Tipo: {}",
                format!("{}", transaccion.tipo).bright_yellow()
            );
            println!(
                "   Monto: {}",
                format!("€{:.2}", transaccion.monto).bright_green()
            );
            println!("   Descripción: {}", transaccion.descripcion.bright_white());
            println!(
                "   Fecha: {}",
                transaccion
                    .fecha
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string()
                    .bright_cyan()
            );
            println!(
                "   Retiro ID: {}",
                transaccion.retiro_id.to_string().bright_magenta()
            );
            println!(
                "   Categoría ID: {}",
                transaccion.categoria_id.to_string().bright_magenta()
            );
            println!(
                "   Creado: {}",
                transaccion
                    .created_at
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string()
                    .bright_black()
            );
            println!(
                "   Actualizado: {}",
                transaccion
                    .updated_at
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string()
                    .bright_black()
            );
        }
        None => {
            println!("{}", "❌ Transacción no encontrada.".red().bold());
            return Err(AppError::NotFound("Transacción".to_string()));
        }
    }

    Ok(())
}

async fn eliminar_transaccion(
    repo: TransaccionRepository,
    args: EliminarTransaccionArgs,
) -> Result<()> {
    let id =
        Uuid::parse_str(&args.id).map_err(|_| AppError::Validation("ID inválido".to_string()))?;

    // Verificar que la transacción existe
    let transaccion = match repo.get_by_id(id).await? {
        Some(t) => t,
        None => {
            println!("{}", "❌ Transacción no encontrada.".red().bold());
            return Err(AppError::NotFound("Transacción".to_string()));
        }
    };

    if !args.force {
        println!(
            "{}",
            "⚠️  ¿Estás seguro de que quieres eliminar esta transacción?"
                .yellow()
                .bold()
        );
        println!(
            "   Tipo: {}",
            format!("{}", transaccion.tipo).bright_yellow()
        );
        println!(
            "   Monto: {}",
            format!("€{:.2}", transaccion.monto).bright_green()
        );
        println!("   Descripción: {}", transaccion.descripcion.bright_white());
        println!();
        println!(
            "{}",
            "Usa --force para confirmar la eliminación.".bright_black()
        );
        return Ok(());
    }

    println!("{}", "🗑️  Eliminando transacción...".cyan().bold());

    match repo.delete(id).await? {
        true => {
            println!(
                "{}",
                "✅ Transacción eliminada exitosamente!".green().bold()
            );
        }
        false => {
            println!(
                "{}",
                "❌ Error: No se pudo eliminar la transacción.".red().bold()
            );
            return Err(AppError::Internal(
                "Error eliminando transacción".to_string(),
            ));
        }
    }

    Ok(())
}

async fn calcular_balance(repo: TransaccionRepository, args: BalanceArgs) -> Result<()> {
    println!("{}", "💰 Calculando balance del retiro...".cyan().bold());

    let retiro_id = Uuid::parse_str(&args.retiro_id)
        .map_err(|_| AppError::Validation("ID de retiro inválido".to_string()))?;

    match repo.calculate_balance(retiro_id).await {
        Ok(balance) => {
            println!("{}", "✅ Balance calculado!".green().bold());
            println!();

            // También obtener las transacciones para mostrar más detalles
            let transacciones = repo.get_by_retiro(retiro_id).await?;
            let mut total_ingresos = 0.0;
            let mut total_gastos = 0.0;
            let mut count_ingresos = 0;
            let mut count_gastos = 0;

            for transaccion in &transacciones {
                match transaccion.tipo {
                    TipoTransaccion::Ingreso => {
                        total_ingresos += transaccion.monto;
                        count_ingresos += 1;
                    }
                    TipoTransaccion::Gasto => {
                        total_gastos += transaccion.monto;
                        count_gastos += 1;
                    }
                }
            }

            println!("📊 {}", "Resumen financiero:".bold());
            println!("   Retiro ID: {}", retiro_id.to_string().bright_blue());
            println!(
                "   Total ingresos: {} ({} transacciones)",
                format!("€{:.2}", total_ingresos).green(),
                count_ingresos
            );
            println!(
                "   Total gastos: {} ({} transacciones)",
                format!("€{:.2}", total_gastos).red(),
                count_gastos
            );
            println!(
                "   {}: {}",
                "Balance final".bold(),
                format!("€{:.2}", balance).bright_yellow()
            );

            if balance > 0.0 {
                println!("   Estado: {}", "Superávit ✅".green());
            } else if balance < 0.0 {
                println!("   Estado: {}", "Déficit ⚠️".red());
            } else {
                println!("   Estado: {}", "Equilibrado 🟰".yellow());
            }
        }
        Err(e) => {
            println!("{} {}", "❌ Error calculando balance:".red().bold(), e);
            return Err(e);
        }
    }

    Ok(())
}
