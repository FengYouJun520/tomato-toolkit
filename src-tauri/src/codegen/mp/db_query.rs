use std::fmt::Debug;

use super::{
    config::{
        DataSourceConfig, GlobalConfig, InjectConfig, PackageConfig, StrategyConfig, TemplateConfig,
    },
    config_builder::ConfigBuilder,
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
pub trait DbQuery: Sync + Send + Debug {
    /// 查询表元信息
    async fn table_infos(&self, config: &ConfigBuilder) -> Result<Vec<TableInfo>>;
    /// 查询表中列的元信息
    async fn table_field(
        &self,
        table_info: &TableInfo,
        config: &ConfigBuilder,
    ) -> Result<Vec<TableField>>;
}

/// mysql查询
#[derive(Debug)]
pub struct MysqlQuery;

#[async_trait]
impl DbQuery for MysqlQuery {
    async fn table_infos(&self, config: &ConfigBuilder) -> Result<Vec<TableInfo>> {
        let mut conn = config.datasource_config.connect_mysql().await?;
        let includes: Vec<&String> = config.strategy_config.include.iter().collect();
        let table_names: Vec<String> = includes.into_iter().map(|tb| format!("'{}'", tb)).collect();
        let datasource = &config.datasource_config;

        let query = format!(
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
            datasource.database,
            table_names.join(","),
        );
        let table_infos = conn.fetch_all(query.as_str()).await?;

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
        config: &ConfigBuilder,
    ) -> Result<Vec<TableField>> {
        let mut conn = config.datasource_config.connect_mysql().await?;
        let datasource = &config.datasource_config;
        let query = format!(
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
            datasource.database, table_info.name
        );

        let table_fields = conn.fetch_all(query.as_str()).await?;

        let table_fields = table_fields
            .into_iter()
            .map(|row| {
                let nullable: String = row.get(3);
                let primary: String = row.get(5);
                TableField {
                    name: row.get(0),
                    comment: row.get(1),
                    r#type: row.get(2),
                    is_nullable: nullable == "YES",
                    length: row.get(4),
                    key_flag: primary == "PRI",
                }
            })
            .collect();

        Ok(table_fields)
    }
}

#[derive(Debug)]
pub struct SqliteQuery;

#[async_trait]
impl DbQuery for SqliteQuery {
    async fn table_infos(&self, config: &ConfigBuilder) -> Result<Vec<TableInfo>> {
        let datasource = &config.datasource_config;
        let mut conn = datasource.connect_sqlite().await?;
        let includes: Vec<&String> = config.strategy_config.include.iter().collect();
        let table_names: Vec<String> = includes.into_iter().map(|tb| format!("'{}'", tb)).collect();

        let query = "SELECT name FROM sqlite_master WHERE type ='table'";
        let table_infos = conn.fetch_all(query).await?;

        let table_infos = table_infos
            .into_iter()
            .map(|row| TableInfo {
                name: row.get(0),
                comment: None,
                fields: None,
                schema: "".to_string(),
            })
            .filter(|tb| table_names.iter().any(|tbn| *tbn == tb.name))
            .collect();

        Ok(table_infos)
    }

    async fn table_field(
        &self,
        table_info: &TableInfo,
        config: &ConfigBuilder,
    ) -> Result<Vec<TableField>> {
        let datasource = &config.datasource_config;
        let mut conn = datasource.connect_sqlite().await?;

        let query = format!("pragma table_info('{}');", table_info.name);
        let fields = conn.fetch_all(query.as_str()).await?;

        let table_fields = fields
            .into_iter()
            .map(|row| {
                let not_null: u8 = row.get(3);
                let primary: u8 = row.get(5);
                TableField {
                    name: row.get(1),
                    comment: None,
                    r#type: row.get(2),
                    length: None,
                    is_nullable: not_null == 0,
                    key_flag: primary == 1,
                }
            })
            .collect();

        Ok(table_fields)
    }
}

#[derive(Debug)]
pub struct MsSqlQuery;

#[async_trait]
impl DbQuery for MsSqlQuery {
    async fn table_infos(&self, _config: &ConfigBuilder) -> Result<Vec<TableInfo>> {
        Ok(vec![])
    }

    async fn table_field(
        &self,
        _table_info: &TableInfo,
        _config: &ConfigBuilder,
    ) -> Result<Vec<TableField>> {
        Ok(vec![])
    }
}

#[derive(Debug)]
pub struct PostgresQuery;

#[async_trait]
impl DbQuery for PostgresQuery {
    async fn table_infos(&self, config: &ConfigBuilder) -> Result<Vec<TableInfo>> {
        let mut conn = config.datasource_config.connect_mysql().await?;
        let datasource = &config.datasource_config;
        let strategy = &config.strategy_config;
        let includes: Vec<&String> = strategy.include.iter().collect();
        let table_names: Vec<String> = includes.into_iter().map(|tb| format!("'{}'", tb)).collect();

        let query = format!(
            r#"SELECT 
    A.tablename, 
    obj_description(relfilenode, 'pg_class') AS comment 
FROM 
    pg_tables A, pg_class B 
WHERE A.schemaname='{}' 
AND A.tablename = B.relname"#,
            datasource.database
        );
        let table_infos = conn.fetch_all(query.as_str()).await?;

        let table_infos = table_infos
            .into_iter()
            .map(|row| TableInfo {
                name: row.get(0),
                comment: row.get(1),
                fields: None,
                schema: "".to_string(),
            })
            .filter(|tb| table_names.iter().any(|tbn| *tbn == tb.name))
            .collect();

        Ok(table_infos)
    }

    async fn table_field(
        &self,
        table_info: &TableInfo,
        config: &ConfigBuilder,
    ) -> Result<Vec<TableField>> {
        let mut conn = config.datasource_config.connect_mysql().await?;
        let query = format!(
            r#"SELECT
    A.attname AS name,
    format_type (A.atttypid,A.atttypmod) AS type,
    col_description (A.attrelid,A.attnum) AS comment,
    A.attlen AS length,
    A.attnotnull AS notnull,
	CASE WHEN length(B.attname) > 0 THEN 'PRI' ELSE '' END AS key
FROM
    pg_attribute A
LEFT JOIN (
    SELECT
        pg_attribute.attname
    FROM
        pg_index,
        pg_class,
        pg_attribute
    WHERE
        pg_class.oid ='"{}"' :: regclass"
    AND pg_index.indrelid = pg_class.oid
    AND pg_attribute.attrelid = pg_class.oid
    AND pg_attribute.attnum = ANY (pg_index.indkey)
) B ON A.attname = b.attname
INNER JOIN pg_class C on A.attrelid = C.oid
INNER JOIN information_schema.columns D on A.attname = D.column_name
WHERE A.attrelid ='"{}"' :: regclass AND A.attnum> 0 AND NOT A.attisdropped AND D.table_name = '{}'
ORDER BY A.attnum;"#,
            table_info.name, table_info.name, table_info.name
        );

        let table_fields = conn.fetch_all(query.as_str()).await?;

        let table_fields = table_fields
            .into_iter()
            .map(|row| {
                let not_null: u8 = row.get(4);
                let primary: String = row.get(5);
                TableField {
                    name: row.get(0),
                    r#type: row.get(1),
                    comment: row.get(2),
                    is_nullable: not_null == 0,
                    length: row.get(3),
                    key_flag: primary == "PRI",
                }
            })
            .collect();

        Ok(table_fields)
    }
}
