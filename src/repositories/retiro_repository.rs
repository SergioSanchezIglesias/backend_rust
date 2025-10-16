// Repositorio de retiros - implementación pendiente
use sqlx::SqlitePool;

pub struct RetiroRepository {
    pool: SqlitePool,
}

impl RetiroRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
