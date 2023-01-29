#![feature(return_position_impl_trait_in_trait)]

use async_trait::async_trait;
use derive_builder::Builder;
use serde::Deserialize;
use sqlx::{
    mssql::MssqlConnectOptions, mysql::MySqlConnectOptions, postgres::PgConnectOptions,
    sqlite::SqliteConnectOptions, ConnectOptions, Connection, MssqlConnection, MySqlConnection,
    PgConnection, SqliteConnection,
};

use crate::error::Result;

use super::{
    config::{
        DataSourceConfig, GlobalConfig, InjectConfig, PackageConfig, StrategyConfig, TemplateConfig,
    },
    model::{TableField, TableInfo},
};

#[derive(Builder, Deserialize)]
#[builder(setter(strip_option))]
pub struct MpConfig {
    injection: Option<InjectConfig>,
    datasource: DataSourceConfig,
    strategy: StrategyConfig,
    package: PackageConfig,
    tempate: TemplateConfig,
    global: GlobalConfig,
}

#[async_trait]
pub trait DbQuery {
    type Item: Connection;
    async fn connect(&self, datasource: &DataSourceConfig) -> Result<Self::Item>;
    async fn table_infos(&self, config: &MpConfig) -> Result<Vec<TableInfo>>;
    async fn table_field(&self, table_info: &TableInfo) -> Result<Vec<TableField>>;
}

pub struct MysqlQuery {}

#[async_trait]
impl DbQuery for MysqlQuery {
    type Item = MySqlConnection;

    async fn connect(&self, datasource: &DataSourceConfig) -> Result<Self::Item> {
        let conn = MySqlConnectOptions::new()
            .host(&datasource.host)
            .port(datasource.port)
            .username(&datasource.username)
            .password(&datasource.password)
            .connect()
            .await?;

        Ok(conn)
    }

    async fn table_infos(&self, config: &MpConfig) -> Result<Vec<TableInfo>> {
        Ok(vec![])
    }

    async fn table_field(&self, table_info: &TableInfo) -> Result<Vec<TableField>> {
        Ok(vec![])
    }
}

pub struct SqliteQuery {}

#[async_trait]
impl DbQuery for SqliteQuery {
    type Item = SqliteConnection;

    async fn connect(&self, datasource: &DataSourceConfig) -> Result<Self::Item> {
        let conn = SqliteConnectOptions::new()
            .filename(&datasource.database)
            .connect()
            .await?;
        Ok(conn)
    }

    async fn table_infos(&self, config: &MpConfig) -> Result<Vec<TableInfo>> {
        Ok(vec![])
    }

    async fn table_field(&self, table_info: &TableInfo) -> Result<Vec<TableField>> {
        Ok(vec![])
    }
}

pub struct MsSqlQuery {}

#[async_trait]
impl DbQuery for MsSqlQuery {
    type Item = MssqlConnection;

    async fn connect(&self, datasource: &DataSourceConfig) -> Result<Self::Item> {
        let conn = MssqlConnectOptions::new()
            .host(&datasource.host)
            .port(datasource.port)
            .database(&datasource.database)
            .username(&datasource.username)
            .password(&datasource.password)
            .connect()
            .await?;
        Ok(conn)
    }

    async fn table_infos(&self, config: &MpConfig) -> Result<Vec<TableInfo>> {
        Ok(vec![])
    }

    async fn table_field(&self, table_info: &TableInfo) -> Result<Vec<TableField>> {
        Ok(vec![])
    }
}

pub struct PostgresQuery {}

#[async_trait]
impl DbQuery for PostgresQuery {
    type Item = PgConnection;

    async fn connect(&self, datasource: &DataSourceConfig) -> Result<Self::Item> {
        let conn = PgConnectOptions::new()
            .host(&datasource.host)
            .port(datasource.port)
            .database(&datasource.database)
            .username(&datasource.username)
            .password(&datasource.password)
            .connect()
            .await?;
        Ok(conn)
    }

    async fn table_infos(&self, config: &MpConfig) -> Result<Vec<TableInfo>> {
        Ok(vec![])
    }

    async fn table_field(&self, table_info: &TableInfo) -> Result<Vec<TableField>> {
        Ok(vec![])
    }
}
