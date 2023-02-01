use std::collections::{BTreeSet, HashMap};

use convert_case::{Case, Casing};
use derive_builder::Builder;
use serde::Serialize;
use sqlx::FromRow;

use crate::error::Result;

use super::{
    config::{DataSourceConfig, Entity, GlobalConfig, StrategyConfig},
    config_builder::ConfigBuilder,
    types::{DbColumnType, STRING},
};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Table {
    /// 表名
    pub name: String,
    /// 注释
    #[sqlx(default)]
    pub comment: Option<String>,
    /// 表所属数据库名
    #[sqlx(default)]
    pub schema: String,
}

/// 表信息
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct TableInfo {
    strategy_config: StrategyConfig,
    global_config: GlobalConfig,
    pub import_packages: BTreeSet<String>,
    /// 表名
    pub name: String,
    /// 注释
    pub comment: Option<String>,
    pub entity: Entity,
    pub entity_name: String,
    /// 表所有列信息
    pub fields: Vec<TableField>,
}

impl TableInfo {
    pub fn new(
        config: &ConfigBuilder,
        comment: Option<String>,
        table_name: impl Into<String>,
    ) -> Self {
        // TODO: 设置entity名称
        Self {
            strategy_config: config.strategy_config.clone(),
            global_config: config.global_config.clone(),
            name: table_name.into(),
            entity: config.strategy_config.entity.clone(),
            import_packages: BTreeSet::new(),
            comment,
            entity_name: "".to_string(),
            fields: vec![],
        }
    }

    /// 处理文件名与导包
    pub fn process_table(&mut self) -> Result<()> {
        self.entity_name = self
            .entity
            .name_convert(self.strategy_config.clone())
            .entity_name_convert(self)?;

        self.import_packages();

        Ok(())
    }

    pub fn import_packages(&mut self) {}
}

/// 列信息
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Field {
    /// 列名
    pub name: String,
    ///注释
    #[sqlx(default)]
    pub comment: Option<String>,
    /// 列类型
    pub r#type: String,
    /// 是否可为空
    pub is_nullable: bool,
    /// 列长度
    pub length: Option<i64>,
    ///是否是主键
    pub key_flag: bool,
    #[sqlx(default)]
    pub default_value: Option<String>,
    #[sqlx(default)]
    pub auto_increment: bool,
}

/// 列信息
#[derive(Debug, Clone, FromRow, Serialize, Builder)]
pub struct TableField {
    pub convert: bool,
    pub key_flag: bool,
    pub key_identity_flag: bool,
    pub name: String,
    pub r#type: String,
    pub property_name: String,
    pub column_type: DbColumnType,
    pub comment: String,
    pub fill: String,
    pub keywords: bool,
    pub column_name: String,
    pub custom_map: Option<HashMap<String, serde_json::Value>>,
    pub entity: Entity,
    pub datasource_config: DataSourceConfig,
    pub global_config: GlobalConfig,
}

impl TableField {
    pub fn set_property_name(&mut self, property_name: &str, column_type: DbColumnType) {
        self.column_type = column_type;

        if self.entity.boolean_column_remove_is_prefix
            && column_type.get_type() == "boolean"
            && self.property_name.starts_with("is")
        {
            self.convert = true;
            self.property_name = (&property_name[2..]).to_case(Case::Camel);
        }
    }
}
