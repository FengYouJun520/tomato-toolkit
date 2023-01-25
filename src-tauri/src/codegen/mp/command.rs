use crate::{codegen::config::DataSourceConfig, error::Result};

pub use super::mp_manager::MpManager;

#[tauri::command]
pub async fn test_connection(config: DataSourceConfig) -> Result<()> {
    Ok(())
}

#[tauri::command]
pub async fn mp_codegen() -> Result<()> {
    Ok(())
}
