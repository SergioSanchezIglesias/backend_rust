#[cfg(feature = "desktop")]
pub mod commands;

#[cfg(feature = "desktop")]
use crate::Result;
#[cfg(feature = "desktop")]
use tauri::Builder;

#[cfg(feature = "desktop")]
pub async fn run_desktop() -> Result<()> {
    Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_categorias,
            commands::create_categoria,
            commands::update_categoria,
            commands::delete_categoria,
            commands::get_retiros,
            commands::create_retiro,
            commands::update_retiro,
            commands::update_retiro_estado,
            commands::delete_retiro,
            commands::get_transacciones,
            commands::create_transaccion,
            commands::delete_transaccion,
            commands::get_balance_retiro
        ])
        .run(tauri::generate_context!())
        .map_err(|e| crate::AppError::Desktop(e.to_string()))?;
    
    Ok(())
}
