use clap::{Args, Subcommand};
use colored::*;
use uuid::Uuid;
use validator::Validate;
use chrono::{DateTime, Utc, NaiveDateTime};

use crate::database::Database;
use crate::models::{CreateRetiro, EstadoRetiro};
use crate::repositories::RetiroRepository;
use crate::{AppError, Result};

#[derive(Subcommand)]
pub enum RetiroCommands {
    /// Crear un nuevo retiro
    Crear(CrearRetiroArgs),
    /// Listar retiros
    Listar(ListarRetiroArgs),
    /// Mostrar detalles de un retiro
    Mostrar(MostrarRetiroArgs),
    /// Actualizar un retiro existente
    Actualizar(ActualizarRetiroArgs),
    /// Cambiar el estado de un retiro
    Estado(EstadoRetiroArgs),
    /// Eliminar un retiro
    Eliminar(EliminarRetiroArgs),
    /// Buscar retiros por nombre
    Buscar(BuscarRetiroArgs),
}

#[derive(Args)]
pub struct CrearRetiroArgs {
    /// Nombre del retiro
    #[arg(short, long)]
    pub nombre: String,
    
    /// Descripción del retiro
    #[arg(short, long)]
    pub descripcion: Option<String>,
    
    /// Fecha de inicio (YYYY-MM-DD HH:MM:SS)
    #[arg(long)]
    pub fecha_inicio: String,
    
    /// Fecha de fin (YYYY-MM-DD HH:MM:SS)
    #[arg(long)]
    pub fecha_fin: String,
    
    /// Ubicación del retiro
    #[arg(short, long)]
    pub ubicacion: Option<String>,
    
    /// Número de participantes
    #[arg(short, long)]
    pub participantes: i32,
}

#[derive(Args)]
pub struct ListarRetiroArgs {
    /// Filtrar por estado del retiro
    #[arg(short, long, value_enum)]
    pub estado: Option<CliEstadoRetiro>,
}

#[derive(Args)]
pub struct MostrarRetiroArgs {
    /// ID del retiro
    pub id: String,
}

#[derive(Args)]
pub struct ActualizarRetiroArgs {
    /// ID del retiro a actualizar
    pub id: String,
    
    /// Nuevo nombre del retiro
    #[arg(short, long)]
    pub nombre: Option<String>,
    
    /// Nueva descripción del retiro
    #[arg(short, long)]
    pub descripcion: Option<String>,
    
    /// Nueva fecha de inicio (YYYY-MM-DD HH:MM:SS)
    #[arg(long)]
    pub fecha_inicio: Option<String>,
    
    /// Nueva fecha de fin (YYYY-MM-DD HH:MM:SS)
    #[arg(long)]
    pub fecha_fin: Option<String>,
    
    /// Nueva ubicación del retiro
    #[arg(short, long)]
    pub ubicacion: Option<String>,
    
    /// Nuevo número de participantes
    #[arg(short, long)]
    pub participantes: Option<i32>,
}

#[derive(Args)]
pub struct EstadoRetiroArgs {
    /// ID del retiro
    pub id: String,
    
    /// Nuevo estado del retiro
    #[arg(value_enum)]
    pub estado: CliEstadoRetiro,
}

#[derive(Args)]
pub struct EliminarRetiroArgs {
    /// ID del retiro a eliminar
    pub id: String,
    
    /// Confirmar eliminación sin preguntar
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args)]
pub struct BuscarRetiroArgs {
    /// Término de búsqueda en el nombre
    pub query: String,
}

#[derive(clap::ValueEnum, Clone)]
pub enum CliEstadoRetiro {
    Planificacion,
    Activo,
    Finalizado,
}

impl From<CliEstadoRetiro> for EstadoRetiro {
    fn from(cli_estado: CliEstadoRetiro) -> Self {
        match cli_estado {
            CliEstadoRetiro::Planificacion => EstadoRetiro::Planificacion,
            CliEstadoRetiro::Activo => EstadoRetiro::Activo,
            CliEstadoRetiro::Finalizado => EstadoRetiro::Finalizado,
        }
    }
}

pub async fn handle_retiro_command(command: RetiroCommands) -> Result<()> {
    // Conectar a la base de datos
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./retiros.db".to_string());
    
    let db = Database::new(&database_url).await?;
    let repo = RetiroRepository::new(db.pool().clone());

    match command {
        RetiroCommands::Crear(args) => crear_retiro(repo, args).await,
        RetiroCommands::Listar(args) => listar_retiros(repo, args).await,
        RetiroCommands::Mostrar(args) => mostrar_retiro(repo, args).await,
        RetiroCommands::Actualizar(args) => actualizar_retiro(repo, args).await,
        RetiroCommands::Estado(args) => cambiar_estado_retiro(repo, args).await,
        RetiroCommands::Eliminar(args) => eliminar_retiro(repo, args).await,
        RetiroCommands::Buscar(args) => buscar_retiros(repo, args).await,
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
    
    Err(AppError::Validation(format!("Formato de fecha inválido: {}. Use YYYY-MM-DD o YYYY-MM-DD HH:MM:SS", date_str)))
}

async fn crear_retiro(repo: RetiroRepository, args: CrearRetiroArgs) -> Result<()> {
    println!("{}", "🆕 Creando nuevo retiro...".cyan().bold());

    let fecha_inicio = parse_datetime(&args.fecha_inicio)?;
    let fecha_fin = parse_datetime(&args.fecha_fin)?;

    let create_data = CreateRetiro {
        nombre: args.nombre.clone(),
        descripcion: args.descripcion.clone(),
        fecha_inicio,
        fecha_fin,
        ubicacion: args.ubicacion.clone(),
        numero_participantes: args.participantes,
    };

    // Validar datos antes de crear
    if let Err(e) = create_data.validate() {
        println!("{} {}", "❌ Error de validación:".red().bold(), e);
        return Err(AppError::Validation(e.to_string()));
    }

    match repo.create(create_data).await {
        Ok(retiro) => {
            println!("{}", "✅ Retiro creado exitosamente!".green().bold());
            println!();
            println!("📋 {}", "Detalles:".bold());
            println!("   ID: {}", retiro.id.to_string().bright_blue());
            println!("   Nombre: {}", retiro.nombre.bright_white());
            println!("   Estado: {}", format!("{}", retiro.estado).bright_yellow());
            println!("   Participantes: {}", retiro.numero_participantes.to_string().bright_green());
            println!("   Fecha inicio: {}", retiro.fecha_inicio.format("%Y-%m-%d %H:%M").to_string().bright_cyan());
            println!("   Fecha fin: {}", retiro.fecha_fin.format("%Y-%m-%d %H:%M").to_string().bright_cyan());
            if let Some(ubicacion) = &retiro.ubicacion {
                println!("   Ubicación: {}", ubicacion.bright_magenta());
            }
            if let Some(descripcion) = &retiro.descripcion {
                println!("   Descripción: {}", descripcion.bright_black());
            }
        }
        Err(e) => {
            println!("{} {}", "❌ Error creando retiro:".red().bold(), e);
            return Err(e);
        }
    }

    Ok(())
}

async fn listar_retiros(repo: RetiroRepository, args: ListarRetiroArgs) -> Result<()> {
    println!("{}", "📋 Listando retiros...".cyan().bold());
    println!();

    let retiros = match args.estado {
        Some(estado) => repo.get_by_estado(estado.into()).await?,
        None => repo.get_all().await?,
    };

    if retiros.is_empty() {
        println!("{}", "📭 No se encontraron retiros.".yellow());
        return Ok(());
    }

    println!("{:<38} {:<25} {:<15} {:<12} {:<12} {:<20}", 
             "ID".bold(), "NOMBRE".bold(), "ESTADO".bold(), "PARTICIPANTES".bold(), "INICIO".bold(), "UBICACIÓN".bold());
    println!("{}", "─".repeat(120).bright_black());

    let total = retiros.len();
    
    for retiro in &retiros {
        let estado_color = match retiro.estado {
            EstadoRetiro::Planificacion => retiro.estado.to_string().yellow(),
            EstadoRetiro::Activo => retiro.estado.to_string().green(),
            EstadoRetiro::Finalizado => retiro.estado.to_string().bright_black(),
        };
        
        let ubicacion_display = retiro.ubicacion.as_deref().unwrap_or("N/A");
        
        println!(
            "{:<38} {:<25} {:<15} {:<12} {:<12} {:<20}",
            retiro.id.to_string().bright_blue(),
            retiro.nombre.bright_white(),
            estado_color,
            retiro.numero_participantes.to_string().bright_green(),
            retiro.fecha_inicio.format("%Y-%m-%d").to_string().bright_cyan(),
            ubicacion_display.bright_magenta()
        );
    }

    println!();
    println!("{} {}", "📊 Total:".bold(), total.to_string().bright_green());

    Ok(())
}

async fn mostrar_retiro(repo: RetiroRepository, args: MostrarRetiroArgs) -> Result<()> {
    println!("{}", "🔍 Buscando retiro...".cyan().bold());

    let id = Uuid::parse_str(&args.id)
        .map_err(|_| AppError::Validation("ID inválido".to_string()))?;

    match repo.get_by_id(id).await? {
        Some(retiro) => {
            println!("{}", "✅ Retiro encontrado!".green().bold());
            println!();
            println!("📋 {}", "Detalles completos:".bold());
            println!("   ID: {}", retiro.id.to_string().bright_blue());
            println!("   Nombre: {}", retiro.nombre.bright_white());
            println!("   Estado: {}", format!("{}", retiro.estado).bright_yellow());
            println!("   Participantes: {}", retiro.numero_participantes.to_string().bright_green());
            println!("   Fecha inicio: {}", retiro.fecha_inicio.format("%Y-%m-%d %H:%M:%S UTC").to_string().bright_cyan());
            println!("   Fecha fin: {}", retiro.fecha_fin.format("%Y-%m-%d %H:%M:%S UTC").to_string().bright_cyan());
            
            if let Some(ubicacion) = &retiro.ubicacion {
                println!("   Ubicación: {}", ubicacion.bright_magenta());
            } else {
                println!("   Ubicación: {}", "No especificada".bright_black());
            }
            
            if let Some(descripcion) = &retiro.descripcion {
                println!("   Descripción: {}", descripcion.bright_black());
            } else {
                println!("   Descripción: {}", "No especificada".bright_black());
            }
            
            println!("   Creado: {}", retiro.created_at.format("%Y-%m-%d %H:%M:%S UTC").to_string().bright_black());
            println!("   Actualizado: {}", retiro.updated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string().bright_black());
        }
        None => {
            println!("{}", "❌ Retiro no encontrado.".red().bold());
            return Err(AppError::NotFound("Retiro".to_string()));
        }
    }

    Ok(())
}

async fn actualizar_retiro(repo: RetiroRepository, args: ActualizarRetiroArgs) -> Result<()> {
    println!("{}", "✏️  Actualizando retiro...".cyan().bold());

    let id = Uuid::parse_str(&args.id)
        .map_err(|_| AppError::Validation("ID inválido".to_string()))?;

    // Obtener retiro actual
    let retiro_actual = match repo.get_by_id(id).await? {
        Some(ret) => ret,
        None => {
            println!("{}", "❌ Retiro no encontrado.".red().bold());
            return Err(AppError::NotFound("Retiro".to_string()));
        }
    };

    // Parsear fechas si se proporcionan
    let fecha_inicio = if let Some(fecha_str) = &args.fecha_inicio {
        parse_datetime(fecha_str)?
    } else {
        retiro_actual.fecha_inicio
    };

    let fecha_fin = if let Some(fecha_str) = &args.fecha_fin {
        parse_datetime(fecha_str)?
    } else {
        retiro_actual.fecha_fin
    };

    // Crear datos de actualización usando valores actuales como default
    let update_data = CreateRetiro {
        nombre: args.nombre.unwrap_or(retiro_actual.nombre),
        descripcion: args.descripcion.or(retiro_actual.descripcion),
        fecha_inicio,
        fecha_fin,
        ubicacion: args.ubicacion.or(retiro_actual.ubicacion),
        numero_participantes: args.participantes.unwrap_or(retiro_actual.numero_participantes),
    };

    // Validar datos
    if let Err(e) = update_data.validate() {
        println!("{} {}", "❌ Error de validación:".red().bold(), e);
        return Err(AppError::Validation(e.to_string()));
    }

    match repo.update(id, update_data).await? {
        Some(retiro) => {
            println!("{}", "✅ Retiro actualizado exitosamente!".green().bold());
            println!();
            println!("📋 {}", "Nuevos detalles:".bold());
            println!("   ID: {}", retiro.id.to_string().bright_blue());
            println!("   Nombre: {}", retiro.nombre.bright_white());
            println!("   Estado: {}", format!("{}", retiro.estado).bright_yellow());
            println!("   Participantes: {}", retiro.numero_participantes.to_string().bright_green());
            println!("   Fecha inicio: {}", retiro.fecha_inicio.format("%Y-%m-%d %H:%M").to_string().bright_cyan());
            println!("   Fecha fin: {}", retiro.fecha_fin.format("%Y-%m-%d %H:%M").to_string().bright_cyan());
            if let Some(ubicacion) = &retiro.ubicacion {
                println!("   Ubicación: {}", ubicacion.bright_magenta());
            }
        }
        None => {
            println!("{}", "❌ Error: Retiro no encontrado durante la actualización.".red().bold());
            return Err(AppError::NotFound("Retiro".to_string()));
        }
    }

    Ok(())
}

async fn cambiar_estado_retiro(repo: RetiroRepository, args: EstadoRetiroArgs) -> Result<()> {
    println!("{}", "🔄 Cambiando estado del retiro...".cyan().bold());

    let id = Uuid::parse_str(&args.id)
        .map_err(|_| AppError::Validation("ID inválido".to_string()))?;

    let nuevo_estado: EstadoRetiro = args.estado.into();

    match repo.update_estado(id, nuevo_estado).await? {
        Some(retiro) => {
            println!("{}", "✅ Estado actualizado exitosamente!".green().bold());
            println!();
            println!("📋 {}", "Detalles:".bold());
            println!("   Retiro: {}", retiro.nombre.bright_white());
            println!("   Nuevo estado: {}", format!("{}", retiro.estado).bright_yellow());
        }
        None => {
            println!("{}", "❌ Retiro no encontrado.".red().bold());
            return Err(AppError::NotFound("Retiro".to_string()));
        }
    }

    Ok(())
}

async fn eliminar_retiro(repo: RetiroRepository, args: EliminarRetiroArgs) -> Result<()> {
    let id = Uuid::parse_str(&args.id)
        .map_err(|_| AppError::Validation("ID inválido".to_string()))?;

    // Verificar que el retiro existe
    let retiro = match repo.get_by_id(id).await? {
        Some(ret) => ret,
        None => {
            println!("{}", "❌ Retiro no encontrado.".red().bold());
            return Err(AppError::NotFound("Retiro".to_string()));
        }
    };

    if !args.force {
        println!("{}", "⚠️  ¿Estás seguro de que quieres eliminar este retiro?".yellow().bold());
        println!("   Nombre: {}", retiro.nombre.bright_white());
        println!("   Estado: {}", format!("{}", retiro.estado).bright_yellow());
        println!("   Participantes: {}", retiro.numero_participantes.to_string().bright_green());
        println!();
        println!("{}", "⚠️  ADVERTENCIA: Esto también eliminará todas las transacciones asociadas.".red());
        println!("{}", "Usa --force para confirmar la eliminación.".bright_black());
        return Ok(());
    }

    println!("{}", "🗑️  Eliminando retiro...".cyan().bold());

    match repo.delete(id).await? {
        true => {
            println!("{}", "✅ Retiro eliminado exitosamente!".green().bold());
        }
        false => {
            println!("{}", "❌ Error: No se pudo eliminar el retiro.".red().bold());
            return Err(AppError::Internal("Error eliminando retiro".to_string()));
        }
    }

    Ok(())
}

async fn buscar_retiros(repo: RetiroRepository, args: BuscarRetiroArgs) -> Result<()> {
    println!("{} '{}'", "🔍 Buscando retiros con:".cyan().bold(), args.query.bright_white());
    println!();

    let retiros = repo.search_by_name(&args.query).await?;

    if retiros.is_empty() {
        println!("{}", "📭 No se encontraron retiros que coincidan con la búsqueda.".yellow());
        return Ok(());
    }

    println!("{:<38} {:<25} {:<15} {:<12} {:<12}", 
             "ID".bold(), "NOMBRE".bold(), "ESTADO".bold(), "PARTICIPANTES".bold(), "INICIO".bold());
    println!("{}", "─".repeat(100).bright_black());

    let total = retiros.len();
    
    for retiro in &retiros {
        let estado_color = match retiro.estado {
            EstadoRetiro::Planificacion => retiro.estado.to_string().yellow(),
            EstadoRetiro::Activo => retiro.estado.to_string().green(),
            EstadoRetiro::Finalizado => retiro.estado.to_string().bright_black(),
        };
        
        println!(
            "{:<38} {:<25} {:<15} {:<12} {:<12}",
            retiro.id.to_string().bright_blue(),
            retiro.nombre.bright_white(),
            estado_color,
            retiro.numero_participantes.to_string().bright_green(),
            retiro.fecha_inicio.format("%Y-%m-%d").to_string().bright_cyan(),
        );
    }

    println!();
    println!("{} {}", "📊 Encontrados:".bold(), total.to_string().bright_green());

    Ok(())
}
