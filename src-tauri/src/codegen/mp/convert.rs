use std::{collections::HashSet, fmt::Debug};

use crate::error::Result;

use super::{
    config::{GlobalConfig, NamingStrategy, StrategyConfig},
    model::{TableField, TableInfo},
    types::{
        DateType, DbColumnType, DbType, TypeConvert, BIG_DECIMAL, BLOB, BOOLEAN, BYTE_ARRAY, CLOB,
        DATE, DATE_SQL, DOUBLE, FLOAT, INTEGER, LOCAL_DATE, LOCAL_DATE_TIME, LOCAL_TIME, LONG,
        STRING, TIME, TIMESTAMP,
    },
};

/// 名称转换器
pub trait NameConvert: Debug {
    fn entity_name_convert(&self, table_info: TableInfo) -> Result<String>;
    fn property_name_convert(&self, field: TableField) -> Result<String>;
}

/// 默认名称转换器
#[derive(Debug)]
pub struct DefaultNameConvert {
    strategy_config: StrategyConfig,
}

impl DefaultNameConvert {
    pub fn new(strategy_config: StrategyConfig) -> Self {
        Self { strategy_config }
    }

    fn process_name(
        &self,
        name: &str,
        strategy: NamingStrategy,
        prefix: &HashSet<String>,
        suffix: &HashSet<String>,
    ) -> Result<String> {
        let mut property_name = name.to_string();

        // 删除前缀
        if !prefix.is_empty() {
            property_name = NamingStrategy::remove_prefix(&property_name, prefix);
        }

        // 删除后缀
        if !suffix.is_empty() {
            property_name = NamingStrategy::remove_suffix(&property_name, suffix);
        }

        if property_name.is_empty() {
            return Err(format!("{:?} 的名称转换结果为空，请检查是否配置问题", name).into());
        }

        if matches!(strategy, NamingStrategy::UnderlineToCamel) {
            return Ok(NamingStrategy::underline_to_camel(&property_name));
        }

        Ok(property_name)
    }
}

impl NameConvert for DefaultNameConvert {
    fn entity_name_convert(&self, table_info: TableInfo) -> Result<String> {
        let name = self.process_name(
            &table_info.name,
            self.strategy_config.entity.naming,
            &self.strategy_config.table_prefix,
            &self.strategy_config.table_suffix,
        )?;
        let res = NamingStrategy::capital(&name);

        Ok(res)
    }

    fn property_name_convert(&self, field: TableField) -> Result<String> {
        self.process_name(
            &field.name,
            self.strategy_config
                .entity
                .column_naming
                .unwrap_or(self.strategy_config.entity.naming),
            &self.strategy_config.field_prefix,
            &self.strategy_config.field_suffix,
        )
    }
}

/// 类型转换器集合
pub struct TypeConverts;

impl TypeConverts {
    pub fn get_type_convert(db_type: DbType) -> Box<dyn TypeConvert> {
        match db_type {
            DbType::MYSQL => Box::new(MysqlTypeConvert),
            DbType::SQLITE => Box::new(SqliteTypeConvert),
            DbType::POSTGRES_SQL => Box::new(PostgresSqlTypeConvert),
            DbType::SQL_SERVER => Box::new(SqliteTypeConvert),
            _ => Box::new(MysqlTypeConvert),
        }
    }
}

/// sqlite类型转换器
pub struct SqliteTypeConvert;

impl TypeConvert for SqliteTypeConvert {
    fn type_convert(&self, config: &GlobalConfig, field_type: &str) -> DbColumnType {
        match field_type.to_lowercase().as_str() {
            "bigint" => LONG,
            "tinyint(1)" | "boolean" => BOOLEAN,
            "int" => INTEGER,
            "text" | "char" | "enum" => STRING,
            "decimal" | "numberic" => BIG_DECIMAL,
            "clob" => CLOB,
            "blob" => BLOB,
            "float" => FLOAT,
            "duble" => DOUBLE,
            "date" | "time" | "year" => self.to_date_type(config, field_type),
            _ => STRING,
        }
    }
}

/// mysql类型转换器
pub struct MysqlTypeConvert;

impl TypeConvert for MysqlTypeConvert {
    fn type_convert(&self, config: &GlobalConfig, field_type: &str) -> DbColumnType {
        match field_type.to_lowercase().as_str() {
            "bigint" => LONG,
            "tinyint(1)" | "boolean" => BOOLEAN,
            "int" => INTEGER,
            "text" | "char" | "enum" => STRING,
            "decimal" | "numberic" => BIG_DECIMAL,
            "clob" => CLOB,
            "blob" => BLOB,
            "float" => FLOAT,
            "duble" => DOUBLE,
            "date" | "time" | "year" => self.to_date_type(config, field_type),
            _ => STRING,
        }
    }
}

pub struct SqlServerTypeConvert;

impl TypeConvert for SqlServerTypeConvert {
    fn type_convert(&self, config: &GlobalConfig, field_type: &str) -> DbColumnType {
        match field_type.to_lowercase().as_str() {
            "char" | "xml" | "text" => STRING,
            "bigint" => LONG,
            "int" => INTEGER,
            "bit" => BOOLEAN,
            "decimal" | "numberic" => DOUBLE,
            "money" => BIG_DECIMAL,
            "binary" | "image" => BYTE_ARRAY,
            "float" | "real" => FLOAT,
            "date" | "time" => self.to_date_type(config, field_type),
            _ => STRING,
        }
    }

    fn to_date_type(&self, config: &GlobalConfig, r#type: &str) -> DbColumnType {
        match config.date_type {
            DateType::SQL_PACK => match r#type {
                "date" => DATE_SQL,
                "time" => TIME,
                _ => TIMESTAMP,
            },
            DateType::TIME_PACK => match r#type {
                "date" => LOCAL_DATE,
                "time" => LOCAL_TIME,
                _ => LOCAL_DATE_TIME,
            },
            _ => DATE,
        }
    }
}

/// mysql类型转换器
pub struct PostgresSqlTypeConvert;

impl TypeConvert for PostgresSqlTypeConvert {
    fn type_convert(&self, config: &GlobalConfig, field_type: &str) -> DbColumnType {
        match field_type.to_lowercase().as_str() {
            "char" | "text" | "json" | "enum" => STRING,
            "bigint" => LONG,
            "int" => INTEGER,
            "bit" => BOOLEAN,
            "decimal" | "numeric" => BIG_DECIMAL,
            "bytea" => BYTE_ARRAY,
            "float" => FLOAT,
            "double" => DOUBLE,
            "boolean" => BOOLEAN,
            "date" | "time" => self.to_date_type(config, field_type),
            _ => STRING,
        }
    }

    fn to_date_type(&self, config: &GlobalConfig, r#type: &str) -> DbColumnType {
        match config.date_type {
            DateType::SQL_PACK => match r#type {
                "date" => DATE_SQL,
                "time" => TIME,
                _ => TIMESTAMP,
            },
            DateType::TIME_PACK => match r#type {
                "date" => LOCAL_DATE,
                "time" => LOCAL_TIME,
                _ => LOCAL_DATE_TIME,
            },
            _ => DATE,
        }
    }
}
