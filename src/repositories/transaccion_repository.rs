// Repositorio de transacciones - implementación pendiente
use sqlx::SqlitePool;

pub struct TransaccionRepository {
    pool: SqlitePool,
}

impl TransaccionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
