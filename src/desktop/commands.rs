#[cfg(feature = "desktop")]
use crate::database::connection::Database;
#[cfg(feature = "desktop")]
use crate::models::*;
#[cfg(feature = "desktop")]
use crate::repositories::*;
#[cfg(feature = "desktop")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "desktop")]
use uuid::Uuid;

#[cfg(feature = "desktop")]
async fn get_database_pool() -> Result<sqlx::SqlitePool, String> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:retiros.db".to_string());
    
    let db = Database::new(&database_url).await
        .map_err(|e| format!("Error conectando a la base de datos: {}", e))?;
    
    Ok(db.pool().clone())
}

// ============================================================================
// COMANDOS PARA CATEGORÍAS
// ============================================================================

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn get_categorias() -> Result<Vec<Categoria>, String> {
    let pool = get_database_pool().await?;
    let repo = CategoriaRepository::new(pool);
    
    repo.get_all().await.map_err(|e| e.to_string())
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn create_categoria(data: CreateCategoria) -> Result<Categoria, String> {
    let pool = get_database_pool().await?;
    let repo = CategoriaRepository::new(pool);
    
    repo.create(data).await.map_err(|e| e.to_string())
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn update_categoria(id: String, data: CreateCategoria) -> Result<Option<Categoria>, String> {
    let pool = get_database_pool().await?;
    let repo = CategoriaRepository::new(pool);
    
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    repo.update(uuid, data).await.map_err(|e| e.to_string())
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn delete_categoria(id: String) -> Result<bool, String> {
    let pool = get_database_pool().await?;
    let repo = CategoriaRepository::new(pool);
    
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    repo.delete(uuid).await.map_err(|e| e.to_string())
}

// ============================================================================
// COMANDOS PARA RETIROS
// ============================================================================

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn get_retiros() -> Result<Vec<Retiro>, String> {
    let pool = get_database_pool().await?;
    let repo = RetiroRepository::new(pool);
    
    repo.get_all().await.map_err(|e| e.to_string())
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn create_retiro(data: CreateRetiro) -> Result<Retiro, String> {
    let pool = get_database_pool().await?;
    let repo = RetiroRepository::new(pool);
    
    repo.create(data).await.map_err(|e| e.to_string())
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn update_retiro(id: String, data: CreateRetiro) -> Result<Option<Retiro>, String> {
    let pool = get_database_pool().await?;
    let repo = RetiroRepository::new(pool);
    
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    repo.update(uuid, data).await.map_err(|e| e.to_string())
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn update_retiro_estado(id: String, estado: String) -> Result<Option<Retiro>, String> {
    let pool = get_database_pool().await?;
    let repo = RetiroRepository::new(pool);
    
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    // Convertir string a EstadoRetiro
    let nuevo_estado = match estado.as_str() {
        "Planificacion" => EstadoRetiro::Planificacion,
        "Activo" => EstadoRetiro::Activo,
        "Finalizado" => EstadoRetiro::Finalizado,
        _ => return Err("Estado no válido".to_string()),
    };
    
    repo.update_estado(uuid, nuevo_estado).await.map_err(|e| e.to_string())
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn delete_retiro(id: String) -> Result<bool, String> {
    let pool = get_database_pool().await?;
    let repo = RetiroRepository::new(pool);
    
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    repo.delete(uuid).await.map_err(|e| e.to_string())
}

// ============================================================================
// COMANDOS PARA TRANSACCIONES
// ============================================================================

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn get_transacciones(retiro_id: Option<String>) -> Result<Vec<Transaccion>, String> {
    let pool = get_database_pool().await?;
    let repo = TransaccionRepository::new(pool);
    
    match retiro_id {
        Some(id_str) => {
            let uuid = Uuid::parse_str(&id_str).map_err(|e| e.to_string())?;
            repo.get_by_retiro(uuid).await.map_err(|e| e.to_string())
        }
        None => {
            // No hay get_all para transacciones, devolver lista vacía
            Ok(vec![])
        }
    }
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn create_transaccion(data: CreateTransaccion) -> Result<Transaccion, String> {
    let pool = get_database_pool().await?;
    let repo = TransaccionRepository::new(pool);
    
    repo.create(data).await.map_err(|e| e.to_string())
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn delete_transaccion(id: String) -> Result<bool, String> {
    let pool = get_database_pool().await?;
    let repo = TransaccionRepository::new(pool);
    
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    repo.delete(uuid).await.map_err(|e| e.to_string())
}

// ============================================================================
// COMANDOS PARA BALANCE Y ESTADÍSTICAS
// ============================================================================

#[cfg(feature = "desktop")]
#[derive(Serialize, Deserialize)]
pub struct BalanceRetiro {
    pub retiro_id: String,
    pub balance: f64,
    pub total_ingresos: f64,
    pub total_gastos: f64,
    pub transacciones_count: i32,
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn get_balance_retiro(retiro_id: String) -> Result<BalanceRetiro, String> {
    let pool = get_database_pool().await?;
    let transaccion_repo = TransaccionRepository::new(pool.clone());
    
    let uuid = Uuid::parse_str(&retiro_id).map_err(|e| e.to_string())?;
    
    let total_ingresos = transaccion_repo.calculate_balance(uuid, Some(TipoTransaccion::Ingreso)).await.map_err(|e| e.to_string())?;
    let total_gastos = transaccion_repo.calculate_balance(uuid, Some(TipoTransaccion::Gasto)).await.map_err(|e| e.to_string())?;
    let transacciones_count = transaccion_repo.count_by_retiro(uuid).await.map_err(|e| e.to_string())?;
    
    Ok(BalanceRetiro {
        retiro_id,
        balance: total_ingresos - total_gastos,
        total_ingresos,
        total_gastos,
        transacciones_count: transacciones_count as i32,
    })
}

#[cfg(feature = "desktop")]
#[derive(Serialize, Deserialize)]
pub struct BalanceGlobal {
    pub balance: f64,
    pub total_ingresos: f64,
    pub total_gastos: f64,
    pub transacciones_count: i32,
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn get_balance_global() -> Result<BalanceGlobal, String> {
    let pool = get_database_pool().await?;
    let transaccion_repo = TransaccionRepository::new(pool);
    
    let (total_ingresos, total_gastos, transacciones_count) = transaccion_repo
        .calculate_global_balance()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(BalanceGlobal {
        balance: total_ingresos - total_gastos,
        total_ingresos,
        total_gastos,
        transacciones_count: transacciones_count as i32,
    })
}

#[cfg(feature = "desktop")]
#[derive(Serialize, Deserialize)]
pub struct EstadisticasAdmin {
    pub promedio_gasto_por_participante: f64,
    pub total_participantes: i64,
    pub promedio_balance_por_retiro: f64,
    pub promedio_ingreso_por_retiro: f64,
    pub promedio_gasto_por_retiro: f64,
    pub retiros_con_transacciones: i32,
    pub top_categorias_gastos: Vec<TopCategoria>,
}

#[cfg(feature = "desktop")]
#[derive(Serialize, Deserialize)]
pub struct TopCategoria {
    pub nombre: String,
    pub color: String,
    pub total: f64,
}

#[cfg(feature = "desktop")]
#[derive(Serialize, Deserialize)]
pub struct RetiroFinalizadoConBalance {
    pub id: String,
    pub nombre: String,
    pub fecha_fin: String,
    pub numero_participantes: i32,
    pub balance: f64,
    pub total_gastos: f64,
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn get_estadisticas_admin() -> Result<EstadisticasAdmin, String> {
    let pool = get_database_pool().await?;
    let transaccion_repo = TransaccionRepository::new(pool.clone());
    let retiro_repo = RetiroRepository::new(pool.clone());
    
    // Calcular promedio de gasto por participante
    let (_total_ingresos, total_gastos, _) = transaccion_repo
        .calculate_global_balance()
        .await
        .map_err(|e| e.to_string())?;
    
    let total_participantes = retiro_repo
        .get_total_participantes()
        .await
        .map_err(|e| e.to_string())?;
    
    let promedio_gasto_por_participante = if total_participantes > 0 {
        total_gastos / total_participantes as f64
    } else {
        0.0
    };
    
    // Estadísticas por retiro
    let (promedio_balance, promedio_ingreso, promedio_gasto, retiros_con_transacciones) = transaccion_repo
        .get_estadisticas_por_retiro()
        .await
        .map_err(|e| e.to_string())?;
    
    // Top categorías de gastos
    let top_categorias_raw = transaccion_repo
        .get_top_categorias_gastos(5)
        .await
        .map_err(|e| e.to_string())?;
    
    let top_categorias_gastos: Vec<TopCategoria> = top_categorias_raw
        .into_iter()
        .map(|(nombre, color, total)| TopCategoria {
            nombre,
            color,
            total,
        })
        .collect();
    
    Ok(EstadisticasAdmin {
        promedio_gasto_por_participante,
        total_participantes,
        promedio_balance_por_retiro: promedio_balance,
        promedio_ingreso_por_retiro: promedio_ingreso,
        promedio_gasto_por_retiro: promedio_gasto,
        retiros_con_transacciones,
        top_categorias_gastos,
    })
}

#[cfg(feature = "desktop")]
#[tauri::command]
pub async fn get_retiros_finalizados_recientes() -> Result<Vec<RetiroFinalizadoConBalance>, String> {
    let pool = get_database_pool().await?;
    let retiro_repo = RetiroRepository::new(pool.clone());
    let transaccion_repo = TransaccionRepository::new(pool);
    
    let retiros_finalizados = retiro_repo
        .get_finalizados_recientes(3)
        .await
        .map_err(|e| e.to_string())?;
    
    let mut resultados = Vec::new();
    
    for retiro in retiros_finalizados {
        let total_ingresos = transaccion_repo
            .calculate_balance(retiro.id, Some(TipoTransaccion::Ingreso))
            .await
            .map_err(|e| e.to_string())?;
        
        let total_gastos = transaccion_repo
            .calculate_balance(retiro.id, Some(TipoTransaccion::Gasto))
            .await
            .map_err(|e| e.to_string())?;
        
        resultados.push(RetiroFinalizadoConBalance {
            id: retiro.id.to_string(),
            nombre: retiro.nombre,
            fecha_fin: retiro.fecha_fin.format("%Y-%m-%d").to_string(),
            numero_participantes: retiro.numero_participantes,
            balance: total_ingresos - total_gastos,
            total_gastos,
        });
    }
    
    Ok(resultados)
}