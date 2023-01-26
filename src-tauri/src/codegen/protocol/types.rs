use serde::Deserialize;

use crate::generateColumnType;

pub trait ColumnType {
    fn name(&self) -> &'static str;
    fn java_type(&self) -> JavaType;
}

pub enum MysqlColumnType {
    Tinyint,
    Char,
    Varchar(usize),
    LongText,
    Int,
    Integer,
    BigInt,
    Decimal,
    Float,
    Double,
    Timestamp,
    Date,
    Datetime,
    Json,
}

pub enum SqliteColumnType {}

pub enum MsSqlColumnType {}

pub enum PostgresColumnType {}

pub enum JavaType {
    Bool,
    Integer,
    Long,
    BigDecimal,
    Float,
    Double,
    String,
    Date,
    Datetime,
}

pub struct DbColumnType(pub &'static str, pub Option<&'static str>);

impl Eq for DbColumnType {}
impl PartialEq for DbColumnType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
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
generateColumnType!(BLOB, "Blob", Some("java.sql.Blob"));
generateColumnType!(CLOB, "Clob", Some("java.sql.Clob"));

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deserialize)]
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
