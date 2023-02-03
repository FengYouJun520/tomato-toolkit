use crate::error::Result;
use serde::Serialize;
use serde_json::json;
use sqlx::{Connection, Executor, Row};
use tauri::AppHandle;

pub use super::mp_generator::MpGenerator;
use super::{config::DataSourceConfig, db_query::MpConfig};

#[derive(Debug, Serialize)]
pub struct BasicTableInfo {
    pub name: String,
    pub comment: String,
}

// TODO: 添加在线预览、编辑、添加自定义文件功能
// TOTO: 查看生成的数据（方便自定义文件）

#[tauri::command]
pub async fn test_connection(config: DataSourceConfig) -> Result<Vec<BasicTableInfo>> {
    let mut conn = config.connect().await?;
    conn.ping().await?;

    let tables = conn
        .fetch_all(config.table_info_query_sql()?.as_ref())
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
pub async fn mp_codegen(app: AppHandle, config: MpConfig) -> Result<()> {
    let Some(resource_path) =  app.path_resolver().resolve_resource("templates") else {
        return Err("资源路径不存在".into());
    };
    let mut generator = MpGenerator::new(resource_path, config)?;
    generator.execute().await?;
    Ok(())
}

#[tauri::command]
pub async fn generate_preview(app: AppHandle, config: MpConfig) -> Result<serde_json::Value> {
    let Some(resource_path) =  app.path_resolver().resolve_resource("templates") else {
        return Err("资源路径不存在".into());
    };
    let mut generator = MpGenerator::new(resource_path, config)?;
    Ok(json!(generator.preview().await?))
}
