use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EstadoRetiro {
    Planificacion,
    Activo,
    Finalizado,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromRow)]
pub struct Retiro {
    pub id: Uuid,

    #[validate(length(min = 1, max = 200))]
    pub nombre: String,

    #[validate(length(max = 500))]
    pub descripcion: Option<String>,

    pub fecha_inicio: DateTime<Utc>,
    pub fecha_fin: DateTime<Utc>,

    #[validate(length(max = 200))]
    pub ubicacion: Option<String>,

    #[validate(range(min = 1))]
    pub numero_participantes: i32,

    pub estado: EstadoRetiro,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateRetiro {
    #[validate(length(min = 1, max = 200))]
    pub nombre: String,

    #[validate(length(max = 500))]
    pub descripcion: Option<String>,

    pub fecha_inicio: DateTime<Utc>,
    pub fecha_fin: DateTime<Utc>,

    #[validate(length(max = 200))]
    pub ubicacion: Option<String>,

    #[validate(range(min = 1))]
    pub numero_participantes: i32,
}

impl Retiro {
    pub fn new(data: CreateRetiro) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            nombre: data.nombre,
            descripcion: data.descripcion,
            fecha_inicio: data.fecha_inicio,
            fecha_fin: data.fecha_fin,
            ubicacion: data.ubicacion,
            numero_participantes: data.numero_participantes,
            estado: EstadoRetiro::Planificacion,
            created_at: now,
            updated_at: now,
        }
    }
}

// Implementar Display para facilitar la conversión a string
impl std::fmt::Display for EstadoRetiro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EstadoRetiro::Planificacion => write!(f, "Planificacion"),
            EstadoRetiro::Activo => write!(f, "Activo"),
            EstadoRetiro::Finalizado => write!(f, "Finalizado"),
        }
    }
}
