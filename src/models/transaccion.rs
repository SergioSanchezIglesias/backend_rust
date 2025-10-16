use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TipoTransaccion {
    Ingreso,
    Gasto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct Transaccion {
    pub id: Uuid,
    pub retiro_id: Uuid,
    pub categoria_id: Uuid,

    pub tipo: TipoTransaccion,

    #[validate(range(min = 0.01))]
    pub monto: f64,

    #[validate(length(min = 1, max = 300))]
    pub descripcion: String,

    pub fecha: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTransaccion {
    pub retiro_id: Uuid,
    pub categoria_id: Uuid,

    pub tipo: TipoTransaccion,

    #[validate(range(min = 0.01))]
    pub monto: f64,

    #[validate(length(min = 1, max = 300))]
    pub descripcion: String,

    pub fecha: DateTime<Utc>,
}

impl Transaccion {
    pub fn new(data: CreateTransaccion) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            retiro_id: data.retiro_id,
            categoria_id: data.categoria_id,
            tipo: data.tipo,
            monto: data.monto,
            descripcion: data.descripcion,
            fecha: data.fecha,
            created_at: now,
            updated_at: now,
        }
    }
}
