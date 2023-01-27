use serde::Serialize;
use sqlx::{Connection, Executor, Row};

use crate::{codegen::config::DataSourceConfig, error::Result};

pub use super::mp_manager::MpManager;

#[derive(Debug, Serialize)]
pub struct BasicTableInfo {
    pub name: String,
    pub comment: String,
}

#[tauri::command]
pub async fn test_connection(config: DataSourceConfig) -> Result<Vec<BasicTableInfo>> {
    let mut conn = config.connect().await?;
    conn.ping().await?;

    let tables = conn
        .fetch_all(
            format!(
                r#"SELECT table_name name,IFNULL(TABLE_COMMENT,table_name) comment
FROM INFORMATION_SCHEMA.TABLES
WHERE UPPER(table_type)='BASE TABLE'
  AND LOWER(table_schema) = '{}'"#,
                &config.database
            )
            .as_ref(),
        )
        .await?;

    let tables = tables
        .into_iter()
        .map(|table| BasicTableInfo {
            name: table.get(0),
            comment: table.get(1),
        })
        .collect();

    Ok(tables)
}

#[tauri::command]
pub async fn mp_codegen() -> Result<()> {
    Ok(())
}
