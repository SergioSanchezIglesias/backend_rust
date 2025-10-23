use crate::models::{CreateRetiro, EstadoRetiro, Retiro};
use crate::{AppError, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;
use validator::Validate;

pub struct RetiroRepository {
    pool: SqlitePool,
}

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

impl RetiroRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Crear un nuevo retiro
    pub async fn create(&self, data: CreateRetiro) -> Result<Retiro> {
        // Validar datos de entrada
        data.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let retiro = Retiro::new(data);

        // Crear variables para evitar problemas de lifetime
        let id_str = retiro.id.to_string();
        let estado_str = retiro.estado.to_string();
        let fecha_inicio_str = retiro.fecha_inicio.to_rfc3339();
        let fecha_fin_str = retiro.fecha_fin.to_rfc3339();
        let created_at_str = retiro.created_at.to_rfc3339();
        let updated_at_str = retiro.updated_at.to_rfc3339();

        sqlx::query!(
            r#"
            INSERT INTO retiros (id, nombre, descripcion, fecha_inicio, fecha_fin, ubicacion, numero_participantes, estado, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            id_str,
            retiro.nombre,
            retiro.descripcion,
            fecha_inicio_str,
            fecha_fin_str,
            retiro.ubicacion,
            retiro.numero_participantes,
            estado_str,
            created_at_str,
            updated_at_str
        )
        .execute(&self.pool)
        .await?;

        Ok(retiro)
    }

    /// Obtener un retiro por ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Retiro>> {
        let id_str = id.to_string();
        let row = sqlx::query!(
            "SELECT id, nombre, descripcion, fecha_inicio, fecha_fin, ubicacion, numero_participantes, estado, created_at, updated_at FROM retiros WHERE id = ?1",
            id_str
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let retiro = Retiro {
                    id: Uuid::parse_str(&row.id)
                        .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                    nombre: row.nombre,
                    descripcion: row.descripcion,
                    fecha_inicio: parse_flexible_datetime(&row.fecha_inicio)?,
                    fecha_fin: parse_flexible_datetime(&row.fecha_fin)?,
                    ubicacion: row.ubicacion,
                    numero_participantes: row.numero_participantes as i32,
                    estado: match row.estado.as_str() {
                        "Planificacion" => EstadoRetiro::Planificacion,
                        "Activo" => EstadoRetiro::Activo,
                        "Finalizado" => EstadoRetiro::Finalizado,
                        _ => return Err(AppError::Internal("Invalid estado retiro".to_string())),
                    },
                    created_at: parse_flexible_datetime(&row.created_at)?,
                    updated_at: parse_flexible_datetime(&row.updated_at)?,
                };
                Ok(Some(retiro))
            }
            None => Ok(None),
        }
    }

    /// Obtener todos los retiros
    pub async fn get_all(&self) -> Result<Vec<Retiro>> {
        let rows = sqlx::query!(
            "SELECT id, nombre, descripcion, fecha_inicio, fecha_fin, ubicacion, numero_participantes, estado, created_at, updated_at FROM retiros ORDER BY fecha_inicio DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut retiros = Vec::new();
        for row in rows {
            let retiro = Retiro {
                id: Uuid::parse_str(&row.id)
                    .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                nombre: row.nombre,
                descripcion: row.descripcion,
                fecha_inicio: parse_flexible_datetime(&row.fecha_inicio)?,
                fecha_fin: parse_flexible_datetime(&row.fecha_fin)?,
                ubicacion: row.ubicacion,
                numero_participantes: row.numero_participantes as i32,
                estado: match row.estado.as_str() {
                    "Planificacion" => EstadoRetiro::Planificacion,
                    "Activo" => EstadoRetiro::Activo,
                    "Finalizado" => EstadoRetiro::Finalizado,
                    _ => return Err(AppError::Internal("Invalid estado retiro".to_string())),
                },
                created_at: parse_flexible_datetime(&row.created_at)?,
                updated_at: parse_flexible_datetime(&row.updated_at)?,
            };
            retiros.push(retiro);
        }

        Ok(retiros)
    }

    /// Obtener retiros por estado
    pub async fn get_by_estado(&self, estado: EstadoRetiro) -> Result<Vec<Retiro>> {
        let estado_str = estado.to_string();
        let rows = sqlx::query!(
            "SELECT id, nombre, descripcion, fecha_inicio, fecha_fin, ubicacion, numero_participantes, estado, created_at, updated_at FROM retiros WHERE estado = ?1 ORDER BY fecha_inicio DESC",
            estado_str
        )
        .fetch_all(&self.pool)
        .await?;

        let mut retiros = Vec::new();
        for row in rows {
            let retiro = Retiro {
                id: Uuid::parse_str(&row.id)
                    .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                nombre: row.nombre,
                descripcion: row.descripcion,
                fecha_inicio: parse_flexible_datetime(&row.fecha_inicio)?,
                fecha_fin: parse_flexible_datetime(&row.fecha_fin)?,
                ubicacion: row.ubicacion,
                numero_participantes: row.numero_participantes as i32,
                estado: match row.estado.as_str() {
                    "Planificacion" => EstadoRetiro::Planificacion,
                    "Activo" => EstadoRetiro::Activo,
                    "Finalizado" => EstadoRetiro::Finalizado,
                    _ => return Err(AppError::Internal("Invalid estado retiro".to_string())),
                },
                created_at: parse_flexible_datetime(&row.created_at)?,
                updated_at: parse_flexible_datetime(&row.updated_at)?,
            };
            retiros.push(retiro);
        }

        Ok(retiros)
    }

    /// Actualizar un retiro
    pub async fn update(&self, id: Uuid, data: CreateRetiro) -> Result<Option<Retiro>> {
        // Validar datos de entrada
        data.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let id_str = id.to_string();
        let fecha_inicio_str = data.fecha_inicio.to_rfc3339();
        let fecha_fin_str = data.fecha_fin.to_rfc3339();

        let updated_at_str = Utc::now().to_rfc3339();
        let result = sqlx::query!(
            r#"
            UPDATE retiros 
            SET nombre = ?1, descripcion = ?2, fecha_inicio = ?3, fecha_fin = ?4, ubicacion = ?5, numero_participantes = ?6, updated_at = ?7
            WHERE id = ?8
            "#,
            data.nombre,
            data.descripcion,
            fecha_inicio_str,
            fecha_fin_str,
            data.ubicacion,
            data.numero_participantes,
            updated_at_str,
            id_str
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        // Devolver el retiro actualizado
        self.get_by_id(id).await
    }

    /// Actualizar el estado de un retiro
    pub async fn update_estado(
        &self,
        id: Uuid,
        nuevo_estado: EstadoRetiro,
    ) -> Result<Option<Retiro>> {
        let id_str = id.to_string();
        let estado_str = nuevo_estado.to_string();

        let updated_at_str = Utc::now().to_rfc3339();
        let result = sqlx::query!(
            r#"
            UPDATE retiros 
            SET estado = ?1, updated_at = ?2
            WHERE id = ?3
            "#,
            estado_str,
            updated_at_str,
            id_str
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        // Devolver el retiro actualizado
        self.get_by_id(id).await
    }

    /// Eliminar un retiro
    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let id_str = id.to_string();
        let result = sqlx::query!("DELETE FROM retiros WHERE id = ?1", id_str)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Contar retiros por estado
    pub async fn count_by_estado(&self, estado: EstadoRetiro) -> Result<i64> {
        let estado_str = estado.to_string();
        let row = sqlx::query!(
            "SELECT COUNT(*) as count FROM retiros WHERE estado = ?1",
            estado_str
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.count.into())
    }

    /// Obtener retiros activos (útil para dashboard)
    pub async fn get_activos(&self) -> Result<Vec<Retiro>> {
        self.get_by_estado(EstadoRetiro::Activo).await
    }

    /// Buscar retiros por nombre (búsqueda parcial)
    pub async fn search_by_name(&self, query: &str) -> Result<Vec<Retiro>> {
        let search_pattern = format!("%{}%", query);
        let rows = sqlx::query!(
            "SELECT id, nombre, descripcion, fecha_inicio, fecha_fin, ubicacion, numero_participantes, estado, created_at, updated_at FROM retiros WHERE nombre LIKE ?1 ORDER BY fecha_inicio DESC",
            search_pattern
        )
        .fetch_all(&self.pool)
        .await?;

        let mut retiros = Vec::new();
        for row in rows {
            let retiro = Retiro {
                id: Uuid::parse_str(&row.id)
                    .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                nombre: row.nombre,
                descripcion: row.descripcion,
                fecha_inicio: parse_flexible_datetime(&row.fecha_inicio)?,
                fecha_fin: parse_flexible_datetime(&row.fecha_fin)?,
                ubicacion: row.ubicacion,
                numero_participantes: row.numero_participantes as i32,
                estado: match row.estado.as_str() {
                    "Planificacion" => EstadoRetiro::Planificacion,
                    "Activo" => EstadoRetiro::Activo,
                    "Finalizado" => EstadoRetiro::Finalizado,
                    _ => return Err(AppError::Internal("Invalid estado retiro".to_string())),
                },
                created_at: parse_flexible_datetime(&row.created_at)?,
                updated_at: parse_flexible_datetime(&row.updated_at)?,
            };
            retiros.push(retiro);
        }

        Ok(retiros)
    }
}
