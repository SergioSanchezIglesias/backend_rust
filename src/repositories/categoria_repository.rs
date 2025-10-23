use crate::models::{Categoria, CreateCategoria, TipoCategoria};
use crate::{AppError, Result};
use sqlx::SqlitePool;
use uuid::Uuid;
use validator::Validate;

pub struct CategoriaRepository {
    pool: SqlitePool,
}

impl CategoriaRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Crear una nueva categoría
    pub async fn create(&self, data: CreateCategoria) -> Result<Categoria> {
        // Validar datos de entrada
        data.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let categoria = Categoria::new(data);

        // Crear variables para evitar problemas de lifetime
        let id_str = categoria.id.to_string();
        let tipo_str = categoria.tipo.to_string();

        sqlx::query!(
            r#"
            INSERT INTO categorias (id, nombre, tipo, color)
            VALUES (?1, ?2, ?3, ?4)
            "#,
            id_str,
            categoria.nombre,
            tipo_str,
            categoria.color
        )
        .execute(&self.pool)
        .await?;

        Ok(categoria)
    }

    /// Obtener una categoría por ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Categoria>> {
        let id_str = id.to_string();
        let row = sqlx::query!(
            "SELECT id, nombre, tipo, color, created_at, updated_at FROM categorias WHERE id = ?1",
            id_str
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let categoria = Categoria {
                    id: Uuid::parse_str(&row.id)
                        .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                    nombre: row.nombre,
                    tipo: match row.tipo.as_str() {
                        "Ingreso" => TipoCategoria::Ingreso,
                        "Gasto" => TipoCategoria::Gasto,
                        _ => return Err(AppError::Internal("Invalid tipo categoria".to_string())),
                    },
                    color: row.color,
                };
                Ok(Some(categoria))
            }
            None => Ok(None),
        }
    }

    /// Obtener todas las categorías
    pub async fn get_all(&self) -> Result<Vec<Categoria>> {
        let rows = sqlx::query!(
            "SELECT id, nombre, tipo, color, created_at, updated_at FROM categorias ORDER BY nombre"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut categorias = Vec::new();
        for row in rows {
            let categoria = Categoria {
                id: Uuid::parse_str(&row.id)
                    .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                nombre: row.nombre,
                tipo: match row.tipo.as_str() {
                    "Ingreso" => TipoCategoria::Ingreso,
                    "Gasto" => TipoCategoria::Gasto,
                    _ => return Err(AppError::Internal("Invalid tipo categoria".to_string())),
                },
                color: row.color,
            };
            categorias.push(categoria);
        }

        Ok(categorias)
    }

    /// Obtener categorías por tipo
    pub async fn get_by_tipo(&self, tipo: TipoCategoria) -> Result<Vec<Categoria>> {
        let tipo_str = tipo.to_string();
        let rows = sqlx::query!(
            "SELECT id, nombre, tipo, color, created_at, updated_at FROM categorias WHERE tipo = ?1 ORDER BY nombre",
            tipo_str
        )
        .fetch_all(&self.pool)
        .await?;

        let mut categorias = Vec::new();
        for row in rows {
            let categoria = Categoria {
                id: Uuid::parse_str(&row.id)
                    .map_err(|e| AppError::Internal(format!("Invalid UUID: {}", e)))?,
                nombre: row.nombre,
                tipo: match row.tipo.as_str() {
                    "Ingreso" => TipoCategoria::Ingreso,
                    "Gasto" => TipoCategoria::Gasto,
                    _ => return Err(AppError::Internal("Invalid tipo categoria".to_string())),
                },
                color: row.color,
            };
            categorias.push(categoria);
        }

        Ok(categorias)
    }

    /// Actualizar una categoría
    pub async fn update(&self, id: Uuid, data: CreateCategoria) -> Result<Option<Categoria>> {
        // Validar datos de entrada
        data.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let tipo_str = data.tipo.to_string();
        let id_str = id.to_string();

        let result = sqlx::query!(
            r#"
            UPDATE categorias 
            SET nombre = ?1, tipo = ?2, color = ?3, updated_at = datetime('now')
            WHERE id = ?4
            "#,
            data.nombre,
            tipo_str,
            data.color,
            id_str
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        // Devolver la categoría actualizada
        self.get_by_id(id).await
    }

    /// Eliminar una categoría
    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let id_str = id.to_string();
        let result = sqlx::query!("DELETE FROM categorias WHERE id = ?1", id_str)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Contar categorías por tipo
    pub async fn count_by_tipo(&self, tipo: TipoCategoria) -> Result<i64> {
        let tipo_str = tipo.to_string();
        let row = sqlx::query!(
            "SELECT COUNT(*) as count FROM categorias WHERE tipo = ?1",
            tipo_str
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.count.into())
    }
}
