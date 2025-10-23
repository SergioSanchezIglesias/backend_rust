use clap::{Args, Subcommand};
use colored::*;
use uuid::Uuid;
use validator::Validate;

use crate::database::Database;
use crate::models::{CreateCategoria, TipoCategoria};
use crate::repositories::CategoriaRepository;
use crate::{AppError, Result};

#[derive(Subcommand)]
pub enum CategoriaCommands {
    /// Crear una nueva categoría
    Crear(CrearArgs),
    /// Listar categorías
    Listar(ListarArgs),
    /// Mostrar detalles de una categoría
    Mostrar(MostrarArgs),
    /// Actualizar una categoría existente
    Actualizar(ActualizarArgs),
    /// Eliminar una categoría
    Eliminar(EliminarArgs),
}

#[derive(Args)]
pub struct CrearArgs {
    /// Nombre de la categoría
    #[arg(short, long)]
    pub nombre: String,
    
    /// Tipo de categoría (ingreso/gasto)
    #[arg(short, long, value_enum)]
    pub tipo: CliTipoCategoria,
    
    /// Color en formato hexadecimal (ej: #FF5733)
    #[arg(short, long)]
    pub color: String,
}

#[derive(Args)]
pub struct ListarArgs {
    /// Filtrar por tipo de categoría
    #[arg(short, long, value_enum)]
    pub tipo: Option<CliTipoCategoria>,
}

#[derive(Args)]
pub struct MostrarArgs {
    /// ID de la categoría
    pub id: String,
}

#[derive(Args)]
pub struct ActualizarArgs {
    /// ID de la categoría a actualizar
    pub id: String,
    
    /// Nuevo nombre de la categoría
    #[arg(short, long)]
    pub nombre: Option<String>,
    
    /// Nuevo tipo de categoría
    #[arg(short, long, value_enum)]
    pub tipo: Option<CliTipoCategoria>,
    
    /// Nuevo color en formato hexadecimal
    #[arg(short, long)]
    pub color: Option<String>,
}

#[derive(Args)]
pub struct EliminarArgs {
    /// ID de la categoría a eliminar
    pub id: String,
    
    /// Confirmar eliminación sin preguntar
    #[arg(short, long)]
    pub force: bool,
}

#[derive(clap::ValueEnum, Clone)]
pub enum CliTipoCategoria {
    Ingreso,
    Gasto,
}

impl From<CliTipoCategoria> for TipoCategoria {
    fn from(cli_tipo: CliTipoCategoria) -> Self {
        match cli_tipo {
            CliTipoCategoria::Ingreso => TipoCategoria::Ingreso,
            CliTipoCategoria::Gasto => TipoCategoria::Gasto,
        }
    }
}

pub async fn handle_categoria_command(command: CategoriaCommands) -> Result<()> {
    // Conectar a la base de datos
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./retiros.db".to_string());
    
    let db = Database::new(&database_url).await?;
    let repo = CategoriaRepository::new(db.pool().clone());

    match command {
        CategoriaCommands::Crear(args) => crear_categoria(repo, args).await,
        CategoriaCommands::Listar(args) => listar_categorias(repo, args).await,
        CategoriaCommands::Mostrar(args) => mostrar_categoria(repo, args).await,
        CategoriaCommands::Actualizar(args) => actualizar_categoria(repo, args).await,
        CategoriaCommands::Eliminar(args) => eliminar_categoria(repo, args).await,
    }
}

async fn crear_categoria(repo: CategoriaRepository, args: CrearArgs) -> Result<()> {
    println!("{}", "🆕 Creando nueva categoría...".cyan().bold());

    let create_data = CreateCategoria {
        nombre: args.nombre.clone(),
        tipo: args.tipo.into(),
        color: args.color.clone(),
    };

    // Validar datos antes de crear
    if let Err(e) = create_data.validate() {
        println!("{} {}", "❌ Error de validación:".red().bold(), e);
        return Err(AppError::Validation(e.to_string()));
    }

    match repo.create(create_data).await {
        Ok(categoria) => {
            println!("{}", "✅ Categoría creada exitosamente!".green().bold());
            println!();
            println!("📋 {}", "Detalles:".bold());
            println!("   ID: {}", categoria.id.to_string().bright_blue());
            println!("   Nombre: {}", categoria.nombre.bright_white());
            println!("   Tipo: {}", format!("{}", categoria.tipo).bright_yellow());
            println!("   Color: {} {}", categoria.color.bright_magenta(), "●".color(categoria.color.as_str()));
        }
        Err(e) => {
            println!("{} {}", "❌ Error creando categoría:".red().bold(), e);
            return Err(e);
        }
    }

    Ok(())
}

async fn listar_categorias(repo: CategoriaRepository, args: ListarArgs) -> Result<()> {
    println!("{}", "📋 Listando categorías...".cyan().bold());
    println!();

    let categorias = match args.tipo {
        Some(tipo) => repo.get_by_tipo(tipo.into()).await?,
        None => repo.get_all().await?,
    };

    if categorias.is_empty() {
        println!("{}", "📭 No se encontraron categorías.".yellow());
        return Ok(());
    }

    println!("{:<38} {:<20} {:<10} {:<8}", "ID".bold(), "NOMBRE".bold(), "TIPO".bold(), "COLOR".bold());
    println!("{}", "─".repeat(80).bright_black());

    let total = categorias.len();
    
    for categoria in &categorias {
        let tipo_color = match categoria.tipo {
            TipoCategoria::Ingreso => categoria.tipo.to_string().green(),
            TipoCategoria::Gasto => categoria.tipo.to_string().red(),
        };
        
        println!(
            "{:<38} {:<20} {:<10} {} {}",
            categoria.id.to_string().bright_blue(),
            categoria.nombre.bright_white(),
            tipo_color,
            categoria.color.bright_magenta(),
            "●".color(categoria.color.as_str())
        );
    }

    println!();
    println!("{} {}", "📊 Total:".bold(), total.to_string().bright_green());

    Ok(())
}

async fn mostrar_categoria(repo: CategoriaRepository, args: MostrarArgs) -> Result<()> {
    println!("{}", "🔍 Buscando categoría...".cyan().bold());

    let id = Uuid::parse_str(&args.id)
        .map_err(|_| AppError::Validation("ID inválido".to_string()))?;

    match repo.get_by_id(id).await? {
        Some(categoria) => {
            println!("{}", "✅ Categoría encontrada!".green().bold());
            println!();
            println!("📋 {}", "Detalles completos:".bold());
            println!("   ID: {}", categoria.id.to_string().bright_blue());
            println!("   Nombre: {}", categoria.nombre.bright_white());
            println!("   Tipo: {}", format!("{}", categoria.tipo).bright_yellow());
            println!("   Color: {} {}", categoria.color.bright_magenta(), "●".color(categoria.color.as_str()));
        }
        None => {
            println!("{}", "❌ Categoría no encontrada.".red().bold());
            return Err(AppError::NotFound("Categoría".to_string()));
        }
    }

    Ok(())
}

async fn actualizar_categoria(repo: CategoriaRepository, args: ActualizarArgs) -> Result<()> {
    println!("{}", "✏️  Actualizando categoría...".cyan().bold());

    let id = Uuid::parse_str(&args.id)
        .map_err(|_| AppError::Validation("ID inválido".to_string()))?;

    // Obtener categoría actual
    let categoria_actual = match repo.get_by_id(id).await? {
        Some(cat) => cat,
        None => {
            println!("{}", "❌ Categoría no encontrada.".red().bold());
            return Err(AppError::NotFound("Categoría".to_string()));
        }
    };

    // Crear datos de actualización usando valores actuales como default
    let update_data = CreateCategoria {
        nombre: args.nombre.unwrap_or(categoria_actual.nombre),
        tipo: args.tipo.map(|t| t.into()).unwrap_or(categoria_actual.tipo),
        color: args.color.unwrap_or(categoria_actual.color),
    };

    // Validar datos
    if let Err(e) = update_data.validate() {
        println!("{} {}", "❌ Error de validación:".red().bold(), e);
        return Err(AppError::Validation(e.to_string()));
    }

    match repo.update(id, update_data).await? {
        Some(categoria) => {
            println!("{}", "✅ Categoría actualizada exitosamente!".green().bold());
            println!();
            println!("📋 {}", "Nuevos detalles:".bold());
            println!("   ID: {}", categoria.id.to_string().bright_blue());
            println!("   Nombre: {}", categoria.nombre.bright_white());
            println!("   Tipo: {}", format!("{}", categoria.tipo).bright_yellow());
            println!("   Color: {} {}", categoria.color.bright_magenta(), "●".color(categoria.color.as_str()));
        }
        None => {
            println!("{}", "❌ Error: Categoría no encontrada durante la actualización.".red().bold());
            return Err(AppError::NotFound("Categoría".to_string()));
        }
    }

    Ok(())
}

async fn eliminar_categoria(repo: CategoriaRepository, args: EliminarArgs) -> Result<()> {
    let id = Uuid::parse_str(&args.id)
        .map_err(|_| AppError::Validation("ID inválido".to_string()))?;

    // Verificar que la categoría existe
    let categoria = match repo.get_by_id(id).await? {
        Some(cat) => cat,
        None => {
            println!("{}", "❌ Categoría no encontrada.".red().bold());
            return Err(AppError::NotFound("Categoría".to_string()));
        }
    };

    if !args.force {
        println!("{}", "⚠️  ¿Estás seguro de que quieres eliminar esta categoría?".yellow().bold());
        println!("   Nombre: {}", categoria.nombre.bright_white());
        println!("   Tipo: {}", format!("{}", categoria.tipo).bright_yellow());
        println!();
        println!("{}", "Usa --force para confirmar la eliminación.".bright_black());
        return Ok(());
    }

    println!("{}", "🗑️  Eliminando categoría...".cyan().bold());

    match repo.delete(id).await? {
        true => {
            println!("{}", "✅ Categoría eliminada exitosamente!".green().bold());
        }
        false => {
            println!("{}", "❌ Error: No se pudo eliminar la categoría.".red().bold());
            return Err(AppError::Internal("Error eliminando categoría".to_string()));
        }
    }

    Ok(())
}
