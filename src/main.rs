use backend_rust::models::*;
use chrono::Utc;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configurar logging
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // Cargar variables de entorno
    dotenvy::dotenv().ok();

    info!("🚀 Iniciando Sistema de Gestión Financiera para Retiros");

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