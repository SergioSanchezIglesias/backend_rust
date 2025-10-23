// Módulo para comandos generales del CLI
// Aquí se pueden agregar comandos que no sean específicos de una entidad

use crate::Result;

pub async fn show_help() -> Result<()> {
    println!("Sistema de Gestión Financiera para Retiros");
    println!("Usa --help para ver todos los comandos disponibles");
    Ok(())
}
