use crate::models::{CreateTransaccion, TipoTransaccion, Transaccion};
use crate::{AppError, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;
use validator::Validate;

// Función helper para parsear fechas en múltiples formatos
fn parse_flexible_datetime(date_str: &str) -> Result<DateTime<Utc>> {
    // Intentar RFC3339 primero
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Intentar formato SQLite datetime: "YYYY-MM-DD HH:MM:SS"
    if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        return Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
    }

    // Intentar formato SQLite datetime con microsegundos: "YYYY-MM-DD HH:MM:SS.ffffff"
    if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S%.f") {
        return Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
    }

    Err(AppError::Internal(format!(
        "Invalid date format: {}",
        date_str
    )))
}

pub struct TransaccionRepository {
    pool: SqlitePool,
}

impl TransaccionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Crear una nueva transacción
    pub async fn create(&self, data: CreateTransaccion) -> Result<Transaccion> {
        // Validar datos de entrada
        data.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let transaccion = Transaccion::new(data);

        // Crear variables para evitar problemas de lifetime
        let id_str = transaccion.id.to_string();
        let retiro_id_str = transaccion.retiro_id.to_string();
        let categoria_id_str = transaccion.categoria_id.to_string();
        let tipo_str = transaccion.tipo.to_string();
        let created_at_str = transaccion.created_at.to_rfc3339();
        let updated_at_str = transaccion.updated_at.to_rfc3339();

        sqlx::query!(
            r#"
            INSERT INTO transacciones (id, retiro_id, categoria_id, tipo, monto, descripcion, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            id_str,
            retiro_id_str,
            categoria_id_str,
            tipo_str,
            transaccion.monto,
            transaccion.descripcion,
            created_at_str,
            updated_at_str
        )
        .execute(&self.pool)
        .await?;

        Ok(transaccion)
    }

    /// Obtener una transacción por ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Transaccion>> {
        let id_str = id.to_string();
        let row = sqlx::query!(
            "SELECT id, retiro_id, categoria_id, tipo, monto, descripcion, created_at, updated_at FROM transacciones WHERE id = ?1",
            id_str
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let transaccion = Transaccion {
                    id: Uuid::parse_str(&row.id)
                        .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                    retiro_id: Uuid::parse_str(&row.retiro_id)
                        .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                    categoria_id: Uuid::parse_str(&row.categoria_id)
                        .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                    tipo: match row.tipo.as_str() {
                        "Ingreso" => TipoTransaccion::Ingreso,
                        "Gasto" => TipoTransaccion::Gasto,
                        _ => {
                            return Err(AppError::Internal("Invalid tipo transaccion".to_string()))
                        }
                    },
                    monto: row.monto,
                    descripcion: row.descripcion,
                    created_at: parse_flexible_datetime(&row.created_at)?,
                    updated_at: parse_flexible_datetime(&row.updated_at)?,
                };
                Ok(Some(transaccion))
            }
            None => Ok(None),
        }
    }

    /// Obtener transacciones por retiro
    pub async fn get_by_retiro(&self, retiro_id: Uuid) -> Result<Vec<Transaccion>> {
        let retiro_id_str = retiro_id.to_string();
        let rows = sqlx::query!(
            "SELECT id, retiro_id, categoria_id, tipo, monto, descripcion, created_at, updated_at FROM transacciones WHERE retiro_id = ?1 ORDER BY created_at DESC",
            retiro_id_str
        )
        .fetch_all(&self.pool)
        .await?;

        let mut transacciones = Vec::new();
        for row in rows {
            let transaccion = Transaccion {
                id: Uuid::parse_str(&row.id)
                    .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                retiro_id: Uuid::parse_str(&row.retiro_id)
                    .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                categoria_id: Uuid::parse_str(&row.categoria_id)
                    .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                tipo: match row.tipo.as_str() {
                    "Ingreso" => TipoTransaccion::Ingreso,
                    "Gasto" => TipoTransaccion::Gasto,
                    _ => return Err(AppError::Internal("Invalid tipo transaccion".to_string())),
                },
                monto: row.monto,
                descripcion: row.descripcion,
                created_at: parse_flexible_datetime(&row.created_at)?,
                updated_at: parse_flexible_datetime(&row.updated_at)?,
            };
            transacciones.push(transaccion);
        }
        Ok(transacciones)
    }

    /// Eliminar una transacción
    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let id_str = id.to_string();
        let result = sqlx::query!("DELETE FROM transacciones WHERE id = ?1", id_str)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Calcular balance por tipo de transacción (opcional)
    pub async fn calculate_balance(&self, retiro_id: Uuid, tipo: Option<TipoTransaccion>) -> Result<f64> {
        let retiro_id_str = retiro_id.to_string();

        match tipo {
            Some(TipoTransaccion::Ingreso) => {
                let row = sqlx::query!(
                    "SELECT COALESCE(SUM(monto), 0) as total FROM transacciones WHERE retiro_id = ?1 AND tipo = 'Ingreso'",
                    retiro_id_str
                )
                .fetch_one(&self.pool)
                .await?;
                Ok(row.total as f64)
            },
            Some(TipoTransaccion::Gasto) => {
                let row = sqlx::query!(
                    "SELECT COALESCE(SUM(monto), 0) as total FROM transacciones WHERE retiro_id = ?1 AND tipo = 'Gasto'",
                    retiro_id_str
                )
                .fetch_one(&self.pool)
                .await?;
                Ok(row.total as f64)
            },
            None => {
                // Calcular balance total (ingresos - gastos)
        let ingresos_row = sqlx::query!(
            "SELECT COALESCE(SUM(monto), 0) as total FROM transacciones WHERE retiro_id = ?1 AND tipo = 'Ingreso'",
            retiro_id_str
        )
        .fetch_one(&self.pool)
        .await?;

        let gastos_row = sqlx::query!(
            "SELECT COALESCE(SUM(monto), 0) as total FROM transacciones WHERE retiro_id = ?1 AND tipo = 'Gasto'",
                    retiro_id_str
                )
                .fetch_one(&self.pool)
                .await?;

                Ok((ingresos_row.total - gastos_row.total) as f64)
            }
        }
    }

    /// Contar transacciones por retiro
    pub async fn count_by_retiro(&self, retiro_id: Uuid) -> Result<i64> {
        let retiro_id_str = retiro_id.to_string();
        let row = sqlx::query!(
            "SELECT COUNT(*) as count FROM transacciones WHERE retiro_id = ?1",
            retiro_id_str
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.count as i64)
    }

    /// Calcular balance global (todos los retiros)
    pub async fn calculate_global_balance(&self) -> Result<(f64, f64, i64)> {
        // Total ingresos
        let ingresos_row = sqlx::query!(
            "SELECT COALESCE(SUM(monto), 0) as total FROM transacciones WHERE tipo = 'Ingreso'"
        )
        .fetch_one(&self.pool)
        .await?;

        // Total gastos
        let gastos_row = sqlx::query!(
            "SELECT COALESCE(SUM(monto), 0) as total FROM transacciones WHERE tipo = 'Gasto'"
        )
        .fetch_one(&self.pool)
        .await?;

        // Total transacciones
        let count_row = sqlx::query!(
            "SELECT COUNT(*) as count FROM transacciones"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((
            ingresos_row.total as f64,
            gastos_row.total as f64,
            count_row.count as i64,
        ))
    }

    /// Obtener top categorías de gastos (por monto total)
    pub async fn get_top_categorias_gastos(&self, limit: i32) -> Result<Vec<(String, String, f64)>> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                c.nombre,
                c.color,
                COALESCE(SUM(t.monto), 0) as total 
            FROM transacciones t
            INNER JOIN categorias c ON t.categoria_id = c.id
            WHERE t.tipo = 'Gasto' 
            GROUP BY c.id, c.nombre, c.color
            ORDER BY total DESC 
            LIMIT ?1
            "#,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        let mut resultados = Vec::new();
        for row in rows {
            resultados.push((
                row.nombre,
                row.color,
                row.total as f64,
            ));
        }

        Ok(resultados)
    }

    /// Calcular estadísticas por retiro (para comparativas)
    pub async fn get_estadisticas_por_retiro(&self) -> Result<(f64, f64, f64, i32)> {
        // Promedio de balance por retiro
        let balance_promedio_row = sqlx::query!(
            r#"
            SELECT 
                COALESCE(AVG(balance), 0) as promedio
            FROM (
                SELECT 
                    retiro_id,
                    COALESCE(SUM(CASE WHEN tipo = 'Ingreso' THEN monto ELSE 0 END), 0) - 
                    COALESCE(SUM(CASE WHEN tipo = 'Gasto' THEN monto ELSE 0 END), 0) as balance
                FROM transacciones
                GROUP BY retiro_id
            ) as balances
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        // Promedio de ingresos por retiro
        let ingresos_promedio_row = sqlx::query!(
            r#"
            SELECT 
                COALESCE(AVG(total_ingresos), 0) as promedio
            FROM (
                SELECT 
                    retiro_id,
                    COALESCE(SUM(monto), 0) as total_ingresos
                FROM transacciones
                WHERE tipo = 'Ingreso'
                GROUP BY retiro_id
            ) as ingresos_por_retiro
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        // Promedio de gastos por retiro
        let gastos_promedio_row = sqlx::query!(
            r#"
            SELECT 
                COALESCE(AVG(total_gastos), 0) as promedio,
                COUNT(DISTINCT retiro_id) as count_retiros
            FROM (
                SELECT 
                    retiro_id,
                    COALESCE(SUM(monto), 0) as total_gastos
                FROM transacciones
                WHERE tipo = 'Gasto'
                GROUP BY retiro_id
            ) as gastos_por_retiro
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((
            balance_promedio_row.promedio as f64,
            ingresos_promedio_row.promedio as f64,
            gastos_promedio_row.promedio as f64,
            gastos_promedio_row.count_retiros as i32,
        ))
    }
}

/// Estructura para resumen financiero
#[derive(Debug)]
pub struct FinancialSummary {
    pub retiro_id: Uuid,
    pub total_ingresos: f64,
    pub total_gastos: f64,
    pub balance: f64,
    pub count_ingresos: i32,
    pub count_gastos: i32,
    pub total_transacciones: i32,
}
