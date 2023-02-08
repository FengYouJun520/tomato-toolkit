use std::collections::HashSet;

use enum_dispatch::enum_dispatch;
use regex::Regex;

use crate::error::Result;

use super::{
    config::{GlobalConfig, NamingStrategy, StrategyConfig},
    model::{Field, TableInfo},
    types::{
        DateType, DbColumnType, DbType, BIG_DECIMAL, BLOB, BOOLEAN, BYTE_ARRAY, CLOB, DATE,
        DATE_SQL, DOUBLE, FLOAT, INTEGER, LOCAL_DATE, LOCAL_DATE_TIME, LOCAL_TIME, LONG, STRING,
        TIME, TIMESTAMP, YEAR,
    },
};

/// 名称转换器
pub trait NameConvert {
    fn entity_name_convert(&self, table_info: &TableInfo) -> Result<String>;
    fn property_name_convert(&self, field: &Field) -> Result<String>;
}

/// 默认名称转换器
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
    fn entity_name_convert(&self, table_info: &TableInfo) -> Result<String> {
        let name = self.process_name(
            &table_info.name,
            self.strategy_config.entity.naming,
            &self.strategy_config.table_prefix,
            &self.strategy_config.table_suffix,
        )?;
        Ok(NamingStrategy::capital(&name))
    }

    fn property_name_convert(&self, field: &Field) -> Result<String> {
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

#[enum_dispatch]
pub trait TypeConvert {
    fn type_convert(&self, config: &GlobalConfig, field: &Field) -> DbColumnType;

    /// 日期转换器，默认为mysql日期转换
    fn to_date_type(&self, config: &GlobalConfig, r#type: &str) -> DbColumnType {
        let date_type = Regex::new(r"\(\d+\)").unwrap().replace_all(r#type, "");
        match config.date_type {
            DateType::ONLY_DATE => DATE,
            DateType::SQL_PACK => match date_type.as_ref() {
                "date" | "year" => DATE_SQL,
                "time" => TIME,
                _ => TIMESTAMP,
            },
            DateType::TIME_PACK => match date_type.as_ref() {
                "date" => LOCAL_DATE,
                "time" => LOCAL_TIME,
                "year" => YEAR,
                _ => LOCAL_DATE_TIME,
            },
        }
    }
}

/// 类型转换器集合
pub struct TypeConverts;

impl TypeConverts {
    pub fn get_type_convert(db_type: DbType) -> TypeConvertHandler {
        match db_type {
            DbType::MYSQL => TypeConvertHandler::from(MysqlTypeConvert),
            DbType::SQLITE => TypeConvertHandler::from(SqliteTypeConvert),
            DbType::POSTGRES_SQL => TypeConvertHandler::from(PostgresSqlTypeConvert),
            DbType::SQL_SERVER => TypeConvertHandler::from(SqlServerTypeConvert),
            _ => TypeConvertHandler::from(MysqlTypeConvert),
        }
    }
}

#[enum_dispatch(TypeConvert)]
#[allow(non_camel_case_types)]
pub enum TypeConvertHandler {
    MysqlTypeConvert,
    SqliteTypeConvert,
    PostgresSqlTypeConvert,
    SqlServerTypeConvert,
}

/// sqlite类型转换器
pub struct SqliteTypeConvert;

impl TypeConvert for SqliteTypeConvert {
    fn type_convert(&self, config: &GlobalConfig, field: &Field) -> DbColumnType {
        let length = field.length.unwrap_or_default();
        match field.r#type.to_lowercase().as_str() {
            "bigint" => LONG,
            "tinyint" if length == 0 || length == 1 => BOOLEAN,
            "boolean" => BOOLEAN,
            "int" => INTEGER,
            "text" | "char" | "enum" => STRING,
            "decimal" | "numberic" => BIG_DECIMAL,
            "clob" => CLOB,
            "blob" => BLOB,
            "float" => FLOAT,
            "duble" => DOUBLE,
            "date" | "time" | "year" => self.to_date_type(config, &field.r#type),
            _ => STRING,
        }
    }
}

/// mysql类型转换器
pub struct MysqlTypeConvert;

impl TypeConvert for MysqlTypeConvert {
    fn type_convert(&self, config: &GlobalConfig, field: &Field) -> DbColumnType {
        let length = field.length.unwrap_or_default();
        match field.r#type.to_lowercase().as_str() {
            "bigint" => LONG,
            "tinyint" if length == 0 || length == 1 => BOOLEAN,
            "boolean" => BOOLEAN,
            "int" => INTEGER,
            "text" | "char" | "enum" => STRING,
            "decimal" | "numberic" => BIG_DECIMAL,
            "clob" => CLOB,
            "blob" => BLOB,
            "float" => FLOAT,
            "duble" => DOUBLE,
            "date" | "time" | "year" | "datetime" => self.to_date_type(config, &field.r#type),
            _ => STRING,
        }
    }
}

pub struct SqlServerTypeConvert;

impl TypeConvert for SqlServerTypeConvert {
    fn type_convert(&self, config: &GlobalConfig, field: &Field) -> DbColumnType {
        match field.r#type.to_lowercase().as_str() {
            "char" | "xml" | "text" => STRING,
            "bigint" => LONG,
            "int" => INTEGER,
            "bit" => BOOLEAN,
            "decimal" | "numberic" => DOUBLE,
            "money" => BIG_DECIMAL,
            "binary" | "image" => BYTE_ARRAY,
            "float" | "real" => FLOAT,
            "date" | "time" => self.to_date_type(config, &field.r#type),
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
    fn type_convert(&self, config: &GlobalConfig, field: &Field) -> DbColumnType {
        match field.r#type.to_lowercase().as_str() {
            "char" | "text" | "json" | "enum" => STRING,
            "bigint" => LONG,
            "int" => INTEGER,
            "bit" => BOOLEAN,
            "decimal" | "numeric" => BIG_DECIMAL,
            "bytea" => BYTE_ARRAY,
            "float" => FLOAT,
            "double" => DOUBLE,
            "boolean" => BOOLEAN,
            "date" | "time" => self.to_date_type(config, &field.r#type),
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
