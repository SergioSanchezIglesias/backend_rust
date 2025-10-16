use backend_rust::models::*;
use backend_rust::database::Database;
use backend_rust::repositories::CategoriaRepository;
use chrono::Utc;
use tracing::{info, error, Level};
use tracing_subscriber;
use std::env;
use sqlx::Row;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configurar logging
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // Cargar variables de entorno
    dotenvy::dotenv().ok();

    info!("🚀 Iniciando Sistema de Gestión Financiera para Retiros");

    // Conectar a la base de datos
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./retiros.db".to_string());
    
    match Database::new(&database_url).await {
        Ok(db) => {
            info!("✅ Conexión a base de datos establecida: {}", database_url);
            
            // Crear repositorio de categorías
            let categoria_repo = CategoriaRepository::new(db.pool().clone());
            
            // Probar el repositorio creando algunas categorías de ejemplo
            info!("🧪 Probando repositorio de categorías...");
            
            // Crear categoría de gasto
            let categoria_alojamiento = CreateCategoria {
                nombre: "Alojamiento".to_string(),
                tipo: TipoCategoria::Gasto,
                color: "#FF5733".to_string(),
            };
            
            match categoria_repo.create(categoria_alojamiento).await {
                Ok(categoria) => {
                    info!("✅ Categoría creada: {} ({})", categoria.nombre, categoria.id);
                }
                Err(e) => {
                    error!("❌ Error creando categoría: {}", e);
                }
            }
            
            // Crear categoría de ingreso
            let categoria_inscripciones = CreateCategoria {
                nombre: "Inscripciones".to_string(),
                tipo: TipoCategoria::Ingreso,
                color: "#28A745".to_string(),
            };
            
            match categoria_repo.create(categoria_inscripciones).await {
                Ok(categoria) => {
                    info!("✅ Categoría creada: {} ({})", categoria.nombre, categoria.id);
                }
                Err(e) => {
                    error!("❌ Error creando categoría: {}", e);
                }
            }
            
            // Listar todas las categorías
            match categoria_repo.get_all().await {
                Ok(categorias) => {
                    info!("📋 Total de categorías: {}", categorias.len());
                    for categoria in categorias {
                        info!("  - {} ({}) - {}", categoria.nombre, categoria.tipo, categoria.color);
                    }
                }
                Err(e) => {
                    error!("❌ Error listando categorías: {}", e);
                }
            }
        }
        Err(e) => {
            error!("❌ Error conectando a base de datos: {}", e);
            return Err(e.into());
        }
    }

    // Ejemplo de creación de un retiro
    let retiro_data = CreateRetiro {
        nombre: "Retiro de Primavera 2024".to_string(),
        descripcion: Some("Retiro espiritual en las montañas".to_string()),
        fecha_inicio: Utc::now(),
        fecha_fin: Utc::now(),
        ubicacion: Some("Sierra de Madrid".to_string()),
        numero_participantes: 25,
    };

    let retiro = Retiro::new(retiro_data);
    info!("✅ Retiro creado: {}", retiro.nombre);

    // Ejemplo de creación de categoría
    let categoria_data = CreateCategoria {
        nombre: "Alojamiento".to_string(),
        tipo: TipoCategoria::Gasto,
        color: "#FF5733".to_string(),
    };

    let categoria = Categoria::new(categoria_data);
    info!("✅ Categoría creada: {}", categoria.nombre);

    info!("🎯 Sistema inicializado correctamente");

    Ok(())
}
