use std::fmt::Debug;

use super::{
    config::{
        DataSourceConfig, GlobalConfig, InjectConfig, PackageConfig, StrategyConfig, TemplateConfig,
    },
    config_builder::ConfigBuilder,
    model::{Field, Table, TableInfo},
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
    async fn query_tables(&self, config: &ConfigBuilder) -> Result<Vec<Table>>;
    /// 查询表中列的元信息
    async fn query_table_fields(
        &self,
        table_info: &TableInfo,
        config: &ConfigBuilder,
    ) -> Result<Vec<Field>>;
}

/// mysql查询
#[derive(Debug, Default)]
pub struct MysqlQuery;

#[async_trait]
impl DbQuery for MysqlQuery {
    async fn query_tables(&self, config: &ConfigBuilder) -> Result<Vec<Table>> {
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
        let tables = conn.fetch_all(query.as_str()).await?;

        let tables = tables
            .into_iter()
            .map(|row| Table {
                name: row.get(0),
                comment: row.get(1),
                schema: "".to_string(),
            })
            .collect();

        Ok(tables)
    }

    async fn query_table_fields(
        &self,
        table_info: &TableInfo,
        config: &ConfigBuilder,
    ) -> Result<Vec<Field>> {
        let mut conn = config.datasource_config.connect_mysql().await?;
        let datasource = &config.datasource_config;
        let query = format!(
            r#"SELECT
    COLUMN_NAME 'name',
    column_comment 'comment',
    DATA_TYPE 'data_type',
    IS_NULLABLE 'is_nullable',
    IFNULL( CHARACTER_MAXIMUM_LENGTH, 0 ) 'length',
    COLUMN_KEY 'key_flag',
    COLUMN_DEFAULT default_value,
    EXTRA auto_increment
FROM
	information_schema.COLUMNS 
WHERE
	table_schema = '{}'
	AND table_name = '{}'
ORDER BY ORDINAL_POSITION ASC;"#,
            datasource.database, table_info.name
        );

        let fields = conn.fetch_all(query.as_str()).await?;

        let fields = fields
            .into_iter()
            .map(|row| {
                let nullable: String = row.get(3);
                let primary: String = row.get(5);
                let auto_increment: String = row.get(7);
                Field {
                    name: row.get(0),
                    comment: row.get(1),
                    r#type: row.get(2),
                    is_nullable: nullable == "YES",
                    length: row.get(4),
                    key_flag: primary == "PRI",
                    default_value: row.get(6),
                    auto_increment: auto_increment == "auto_increment",
                }
            })
            .collect();

        Ok(fields)
    }
}

#[derive(Debug, Default)]
pub struct SqliteQuery;

#[async_trait]
impl DbQuery for SqliteQuery {
    async fn query_tables(&self, config: &ConfigBuilder) -> Result<Vec<Table>> {
        let datasource = &config.datasource_config;
        let mut conn = datasource.connect_sqlite().await?;
        let includes: Vec<&String> = config.strategy_config.include.iter().collect();
        let query = "SELECT name FROM sqlite_master WHERE type ='table'";
        let tables = conn.fetch_all(query).await?;

        let tables = tables
            .into_iter()
            .map(|row| Table {
                name: row.get(0),
                comment: None,
                schema: "".to_string(),
            })
            .filter(|tb| includes.iter().any(|tbn| tb.name.eq(tbn.as_str())))
            .collect();

        Ok(tables)
    }

    async fn query_table_fields(
        &self,
        table_info: &TableInfo,
        config: &ConfigBuilder,
    ) -> Result<Vec<Field>> {
        let datasource = &config.datasource_config;
        let mut conn = datasource.connect_sqlite().await?;

        let query = format!("pragma table_info('{}');", table_info.name);
        let fields = conn.fetch_all(query.as_str()).await?;

        let fields = fields
            .into_iter()
            .map(|row| {
                let not_null: u8 = row.get(3);
                let primary: u8 = row.get(5);
                let r#type: String = row.get(2);
                let auto_increment = r#type.to_lowercase() == "integer" && primary == 1;
                Field {
                    name: row.get(1),
                    comment: None,
                    r#type,
                    length: None,
                    is_nullable: not_null == 0,
                    key_flag: primary == 1,
                    default_value: None,
                    auto_increment,
                }
            })
            .collect();
        Ok(fields)
    }
}

#[derive(Debug, Default)]
pub struct MsSqlQuery;

#[async_trait]
impl DbQuery for MsSqlQuery {
    async fn query_tables(&self, _config: &ConfigBuilder) -> Result<Vec<Table>> {
        Ok(vec![])
    }

    async fn query_table_fields(
        &self,
        _table_info: &TableInfo,
        _config: &ConfigBuilder,
    ) -> Result<Vec<Field>> {
        Ok(vec![])
    }
}

#[derive(Debug, Default)]
pub struct PostgresQuery;

#[async_trait]
impl DbQuery for PostgresQuery {
    async fn query_tables(&self, config: &ConfigBuilder) -> Result<Vec<Table>> {
        let mut conn = config.datasource_config.connect_mysql().await?;
        let datasource = &config.datasource_config;
        let strategy = &config.strategy_config;
        let includes: Vec<&String> = strategy.include.iter().collect();

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
        let tables = conn.fetch_all(query.as_str()).await?;

        let tables = tables
            .into_iter()
            .map(|row| Table {
                name: row.get(0),
                comment: row.get(1),
                schema: "".to_string(),
            })
            .filter(|tb| includes.iter().any(|tbn| tb.name.eq(tbn.as_str())))
            .collect();

        Ok(tables)
    }

    async fn query_table_fields(
        &self,
        table_info: &TableInfo,
        config: &ConfigBuilder,
    ) -> Result<Vec<Field>> {
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

        let fields = conn.fetch_all(query.as_str()).await?;

        let fields = fields
            .into_iter()
            .map(|row| {
                let not_null: u8 = row.get(4);
                let primary: String = row.get(5);
                Field {
                    name: row.get(0),
                    r#type: row.get(1),
                    comment: row.get(2),
                    is_nullable: not_null == 0,
                    length: row.get(3),
                    key_flag: primary == "PRI",
                    auto_increment: false,
                    default_value: None,
                }
            })
            .collect();

        Ok(fields)
    }
}
