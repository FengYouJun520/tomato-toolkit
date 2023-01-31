use serde::{Deserialize, Serialize};

use crate::generateColumnType;

use super::config::GlobalConfig;

pub trait TypeConvert {
    fn type_convert(&self, config: &GlobalConfig) -> DbColumnType;
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
generateColumnType!(BLOG, "Blob", Some("java.sql.Blob"));

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum DateType {
    ONLY_DATE,
    SQL_PACK,
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

impl SqliteColumnType {
    fn to_date_type(&self, config: &GlobalConfig) -> DbColumnType {
        match config.date_type {
            DateType::ONLY_DATE => DATE,
            DateType::SQL_PACK => match self {
                Self::Date | Self::Year => DATE_SQL,
                Self::Time => TIME,
                _ => TIMESTAMP,
            },
            DateType::TIME_PACK => match self {
                Self::Date => LOCAL_DATE,
                Self::Time => LOCAL_TIME,
                Self::Year => YEAR,
                _ => LOCAL_DATE_TIME,
            },
        }
    }
}

impl TypeConvert for SqliteColumnType {
    fn type_convert(&self, config: &GlobalConfig) -> DbColumnType {
        match self {
            SqliteColumnType::Bigint => LONG,
            SqliteColumnType::Boolean => BOOLEAN,
            SqliteColumnType::Tinyint(Some(size)) if *size == 1 => BOOLEAN,
            SqliteColumnType::Tinyint(_) => INTEGER,
            SqliteColumnType::Int => INTEGER,
            SqliteColumnType::Text | SqliteColumnType::Char | SqliteColumnType::Enum => STRING,
            SqliteColumnType::Decimal | SqliteColumnType::Numberic => BIG_DECIMAL,
            SqliteColumnType::Clob => CLOB,
            SqliteColumnType::Blob => BLOG,
            SqliteColumnType::Float => FLOAT,
            SqliteColumnType::Double => DOUBLE,
            SqliteColumnType::Date | SqliteColumnType::Time | SqliteColumnType::Year => {
                self.to_date_type(config)
            }
        }
    }
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

impl MysqlColumnType {
    fn to_date_type(&self, config: &GlobalConfig) -> DbColumnType {
        match config.date_type {
            DateType::ONLY_DATE => DATE,
            DateType::SQL_PACK => match self {
                Self::Date | Self::Year => DATE_SQL,
                Self::Time => TIME,
                _ => TIMESTAMP,
            },
            DateType::TIME_PACK => match self {
                Self::Date => LOCAL_DATE,
                Self::Time => LOCAL_TIME,
                Self::Year => YEAR,
                Self::Timestamp => TIMESTAMP,
                _ => LOCAL_DATE_TIME,
            },
        }
    }
}

impl TypeConvert for MysqlColumnType {
    fn type_convert(&self, config: &GlobalConfig) -> DbColumnType {
        match self {
            MysqlColumnType::Bigint => LONG,
            MysqlColumnType::Tinyint(Some(size)) | MysqlColumnType::Bit(Some(size))
                if *size == 1 =>
            {
                BOOLEAN
            }
            MysqlColumnType::Bit(_) => BYTE,
            MysqlColumnType::Tinyint(_) => INTEGER,
            MysqlColumnType::Int => INTEGER,
            MysqlColumnType::Text
            | MysqlColumnType::Char
            | MysqlColumnType::Json
            | MysqlColumnType::Enum => STRING,
            MysqlColumnType::Decimal => BIG_DECIMAL,
            MysqlColumnType::Clob => CLOB,
            MysqlColumnType::Blob => BLOG,
            MysqlColumnType::Binary => BYTE_ARRAY,
            MysqlColumnType::Float => FLOAT,
            MysqlColumnType::Double => DOUBLE,
            MysqlColumnType::Date
            | MysqlColumnType::Time
            | MysqlColumnType::Year
            | MysqlColumnType::DateTime
            | MysqlColumnType::Timestamp => self.to_date_type(config),
        }
    }
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

impl MsSqlColumnType {
    fn to_date_type(&self, config: &GlobalConfig) -> DbColumnType {
        match config.date_type {
            DateType::SQL_PACK => match self {
                Self::Date => DATE_SQL,
                Self::Time => TIME,
                _ => TIMESTAMP,
            },
            DateType::TIME_PACK => match self {
                Self::Date => LOCAL_DATE,
                Self::Time => LOCAL_TIME,
                _ => LOCAL_DATE_TIME,
            },
            _ => DATE,
        }
    }
}

impl TypeConvert for MsSqlColumnType {
    fn type_convert(&self, config: &GlobalConfig) -> DbColumnType {
        match self {
            MsSqlColumnType::Char | MsSqlColumnType::Xml | MsSqlColumnType::Text => STRING,
            MsSqlColumnType::Bit => BOOLEAN,
            MsSqlColumnType::Bigint => LONG,
            MsSqlColumnType::Int => INTEGER,
            MsSqlColumnType::Decimal | MsSqlColumnType::Numberic => DOUBLE,
            MsSqlColumnType::Money => BIG_DECIMAL,
            MsSqlColumnType::Binary | MsSqlColumnType::Image => BYTE_ARRAY,
            MsSqlColumnType::Float | MsSqlColumnType::Real => FLOAT,
            MsSqlColumnType::Date | MsSqlColumnType::Time => self.to_date_type(config),
        }
    }
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

impl PostgresColumnType {
    fn to_date_type(&self, config: &GlobalConfig) -> DbColumnType {
        match config.date_type {
            DateType::SQL_PACK => match self {
                Self::Date => DATE_SQL,
                Self::Time => TIME,
                _ => TIMESTAMP,
            },
            DateType::TIME_PACK => match self {
                Self::Date => LOCAL_DATE,
                Self::Time => LOCAL_TIME,
                _ => LOCAL_DATE_TIME,
            },
            _ => DATE,
        }
    }
}

impl TypeConvert for PostgresColumnType {
    fn type_convert(&self, config: &GlobalConfig) -> DbColumnType {
        match self {
            PostgresColumnType::Char
            | PostgresColumnType::Text
            | PostgresColumnType::Json
            | PostgresColumnType::Enum => STRING,
            PostgresColumnType::Bigint => LONG,
            PostgresColumnType::Int => INTEGER,
            PostgresColumnType::Date | PostgresColumnType::Time => self.to_date_type(config),
            PostgresColumnType::Bit | PostgresColumnType::Boolean => BOOLEAN,
            PostgresColumnType::Decimal | PostgresColumnType::Numberic => BIG_DECIMAL,
            PostgresColumnType::Bytea => BYTE_ARRAY,
            PostgresColumnType::FLoat => FLOAT,
            PostgresColumnType::Double => DOUBLE,
        }
    }
}
