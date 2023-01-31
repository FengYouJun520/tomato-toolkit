use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::generateColumnType;

use super::config::GlobalConfig;

pub trait TypeConvert {
    fn type_convert(&self, config: &GlobalConfig, field_type: &str) -> DbColumnType;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum DbType {
    MYSQL,
    MARIADB,
    ORACLE,
    DB2,
    H2,
    SQLITE,
    POSTGRES_SQL,
    SQL_SERVER,
    OTHER,
}

impl From<&str> for DbType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "mysql" => DbType::MYSQL,
            "oracle" => DbType::ORACLE,
            "db2" => DbType::DB2,
            "h2" => DbType::H2,
            "sqlite" => DbType::SQLITE,
            "postgressql" => DbType::POSTGRES_SQL,
            "sqlserver" => DbType::SQL_SERVER,
            _ => DbType::OTHER,
        }
    }
}

impl From<String> for DbType {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&String> for DbType {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DbColumnType(pub &'static str, pub Option<&'static str>);

impl DbColumnType {
    pub fn get_type(&self) -> &'static str {
        self.0
    }

    pub fn get_pkg(&self) -> Option<&'static str> {
        self.1
    }
}

// 基本类型
generateColumnType!(BASE_BYTE, "byte", None);
generateColumnType!(BASE_SHORT, "short", None);
generateColumnType!(BASE_CHAR, "char", None);
generateColumnType!(BASE_INT, "int", None);
generateColumnType!(BASE_LONG, "long", None);
generateColumnType!(BASE_FLOAT, "float", None);
generateColumnType!(BASE_DOUBLE, "double", None);
generateColumnType!(BASE_BOOLEAN, "boolean", None);

// 包装类型
generateColumnType!(BYTE, "Byte", None);
generateColumnType!(SHORT, "Short", None);
generateColumnType!(CHARACTER, "Character", None);
generateColumnType!(INTEGER, "Integer", None);
generateColumnType!(LONG, "Long", None);
generateColumnType!(FLOAT, "Float", None);
generateColumnType!(DOUBLE, "Double", None);
generateColumnType!(BOOLEAN, "Boolean", None);
generateColumnType!(STRING, "String", None);

// sql 包下数据类型
generateColumnType!(DATE_SQL, "Date", Some("java.sql.Date"));
generateColumnType!(TIME, "Time", Some("java.sql.Time"));
generateColumnType!(TIMESTAMP, "Timestamp", Some("java.sql.Timestamp"));
generateColumnType!(CLOB, "Clob", Some("java.sql.Clob"));
generateColumnType!(BLOB, "Blob", Some("java.sql.Blob"));

// java8 新时间类型
generateColumnType!(LOCAL_DATE, "LocalDate", Some("java.time.LocalDate"));
generateColumnType!(LOCAL_TIME, "LocalTime", Some("java.time.LocalTime"));
generateColumnType!(YEAR, "Year", Some("java.time.Year"));
generateColumnType!(YEAR_MONTH, "YearMonth", Some("java.time.LocalDateTime"));
generateColumnType!(
    LOCAL_DATE_TIME,
    "LocalDateTime",
    Some("java.time.YearMonth")
);
generateColumnType!(INSTANT, "Instant", Some("java.time.Instant"));

// 其他杂类
generateColumnType!(MAP, "Map", Some("java.util.Map"));
generateColumnType!(BYTE_ARRAY, "byte[]", None);
generateColumnType!(OBJECT, "Object", None);
generateColumnType!(DATE, "Date", Some("java.util.Date"));
generateColumnType!(BIG_INTEGER, "BigInteger", Some("java.math.BigInteger"));
generateColumnType!(BIG_DECIMAL, "BigDecimal", Some("java.math.BigDecimal"));

pub enum FieldFill {
    Default,
    Insert,
    Update,
    InsertUpdate,
}

impl FieldFill {
    pub fn name(&self) -> &'static str {
        match self {
            FieldFill::Default => "DEFAULT",
            FieldFill::Insert => "INSERT",
            FieldFill::Update => "UPDATE",
            FieldFill::InsertUpdate => "INSERT_UPDATE",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum DateType {
    ONLY_DATE,
    SQL_PACK,
    #[default]
    TIME_PACK,
}

impl TryFrom<&str> for DateType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_ref() {
            "ONLY_DATE" => Ok(Self::ONLY_DATE),
            "SQL_PACK" => Ok(Self::SQL_PACK),
            "TIME_PACK" => Ok(Self::TIME_PACK),
            _ => Err(format!("时间策略类型不支持: {}", value)),
        }
    }
}

pub enum SqliteColumnType {
    Bigint,
    Tinyint(Option<usize>),
    Boolean,
    Int,
    Text,
    Char,
    Enum,
    Decimal,
    Numberic,
    Clob,
    Blob,
    Float,
    Double,
    Date,
    Time,
    Year,
}

pub enum MysqlColumnType {
    Bigint,
    Bit(Option<usize>),
    Tinyint(Option<usize>),
    Int,
    Char,
    Text,
    Json,
    Enum,
    Decimal,
    Clob,
    Blob,
    Binary,
    Float,
    Double,
    Date,
    Time,
    DateTime,
    Timestamp,
    Year,
}

pub enum MsSqlColumnType {
    Char,
    Bit,
    Xml,
    Text,
    Bigint,
    Int,
    Decimal,
    Numberic,
    Money,
    Binary,
    Image,
    Float,
    Real,
    Date,
    Time,
}

pub enum PostgresColumnType {
    Char,
    Text,
    Json,
    Enum,
    Bigint,
    Int,
    Date,
    Time,
    Bit,
    Decimal,
    Numberic,
    Bytea,
    FLoat,
    Double,
    Boolean,
}
