use super::{
    config::{
        DataSourceConfig, GlobalConfig, InjectConfig, PackageConfig, StrategyConfig, TemplateConfig,
    },
    model::{TableField, TableInfo},
};
use crate::error::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use serde::Deserialize;
use sqlx::{Executor, Row};

#[derive(Debug, Builder, Deserialize)]
#[builder(setter(strip_option))]
pub struct MpConfig {
    pub injection: Option<InjectConfig>,
    pub datasource: DataSourceConfig,
    pub strategy: StrategyConfig,
    pub package: PackageConfig,
    pub template: TemplateConfig,
    pub global: GlobalConfig,
}

#[async_trait]
pub trait DbQuery: Sync + Send {
    /// 查询表元信息
    async fn table_infos(&self, config: &MpConfig) -> Result<Vec<TableInfo>>;
    /// 查询表中列的元信息
    async fn table_field(
        &self,
        table_info: &TableInfo,
        config: &MpConfig,
    ) -> Result<Vec<TableField>>;
}

/// mysql查询
pub struct MysqlQuery;

#[async_trait]
impl DbQuery for MysqlQuery {
    async fn table_infos(&self, config: &MpConfig) -> Result<Vec<TableInfo>> {
        let mut conn = config.datasource.connect_mysql().await?;
        let includes: Vec<&String> = config.strategy.include.iter().collect();
        let table_names: Vec<String> = includes.into_iter().map(|tb| format!("'{}'", tb)).collect();
        let datasource = &config.datasource;

        let sql = format!(
            r#"SELECT
    table_name 'name',
    IFNULL( TABLE_COMMENT, table_name ) 'comment',
    TABLE_SCHEMA 'table_schema'
FROM
	INFORMATION_SCHEMA.TABLES
WHERE
	UPPER( table_type )= 'BASE TABLE'
	AND LOWER( table_schema ) = '{}'
	AND table_name IN ({});"#,
            &datasource.database,
            &table_names.join(","),
        );
        let table_infos = conn.fetch_all(sql.as_ref()).await?;

        let table_infos = table_infos
            .into_iter()
            .map(|row| TableInfo {
                name: row.get(0),
                comment: row.get(1),
                fields: None,
                schema: "".to_string(),
            })
            .collect();

        Ok(table_infos)
    }

    async fn table_field(
        &self,
        table_info: &TableInfo,
        config: &MpConfig,
    ) -> Result<Vec<TableField>> {
        let mut conn = config.datasource.connect_mysql().await?;
        let datasource = &config.datasource;
        let sql = format!(
            r#"SELECT
	COLUMN_NAME 'name',
	column_comment 'comment',
	DATA_TYPE 'data_type',
	IS_NULLABLE 'is_nullable',
	IFNULL( CHARACTER_MAXIMUM_LENGTH, 0 ) 'length',
    COLUMN_KEY 'key_flag' 
FROM
	information_schema.COLUMNS 
WHERE
	table_schema = '{}'
	AND table_name = '{}';"#,
            &datasource.database, &table_info.name
        );

        let table_fields = conn.fetch_all(sql.as_ref()).await?;

        let table_fields = table_fields
            .into_iter()
            .map(|row| {
                let nullable: String = row.get(3);
                let primary: String = row.get(5);
                let key_flag = primary == "PRI";
                TableField {
                    name: row.get(0),
                    comment: row.get(1),
                    r#type: row.get(2),
                    is_nullable: nullable == "YES",
                    length: row.get(4),
                    key_flag,
                }
            })
            .collect();

        Ok(table_fields)
    }
}

pub struct SqliteQuery;

#[async_trait]
impl DbQuery for SqliteQuery {
    async fn table_infos(&self, config: &MpConfig) -> Result<Vec<TableInfo>> {
        Ok(vec![])
    }

    async fn table_field(
        &self,
        table_info: &TableInfo,
        config: &MpConfig,
    ) -> Result<Vec<TableField>> {
        Ok(vec![])
    }
}

pub struct MsSqlQuery;

#[async_trait]
impl DbQuery for MsSqlQuery {
    async fn table_infos(&self, config: &MpConfig) -> Result<Vec<TableInfo>> {
        Ok(vec![])
    }

    async fn table_field(
        &self,
        table_info: &TableInfo,
        config: &MpConfig,
    ) -> Result<Vec<TableField>> {
        Ok(vec![])
    }
}

pub struct PostgresQuery;

#[async_trait]
impl DbQuery for PostgresQuery {
    async fn table_infos(&self, config: &MpConfig) -> Result<Vec<TableInfo>> {
        Ok(vec![])
    }

    async fn table_field(
        &self,
        table_info: &TableInfo,
        config: &MpConfig,
    ) -> Result<Vec<TableField>> {
        Ok(vec![])
    }
}
