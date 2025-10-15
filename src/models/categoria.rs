use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoCategoria {
    Ingreso,
    Gasto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Categoria {
    pub id: Uuid,
    
    #[validate(length(min = 1, max = 100))]
    pub nombre: String,
    
    pub tipo: TipoCategoria,
    
    #[validate(length(min = 7, max = 7))] // Formato #RRGGBB
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCategoria {
    #[validate(length(min = 1, max = 100))]
    pub nombre: String,
    
    pub tipo: TipoCategoria,
    
    #[validate(length(min = 7, max = 7))]
    pub color: String,
}

impl Categoria {
    pub fn new(data: CreateCategoria) -> Self {
        Self {
            id: Uuid::new_v4(),
            nombre: data.nombre,
            tipo: data.tipo,
            color: data.color,
        }
    }
}
