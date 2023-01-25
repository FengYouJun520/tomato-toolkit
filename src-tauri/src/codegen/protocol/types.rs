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

pub enum JavaPackage {
    Bool,
    String,
    Integer,
    Long,
    BigDecimal,
    Float,
    Double,
    LocalDate,
    LocalDatetime,
    Serializer,
    TableName,
    TableId,
    TableField,
    TableLogic,
    Version,
    IService,
    ServiceImpl,
    BaseMapper,
    FieldFill,
    IdType,
}

impl JavaPackage {
    pub fn name(&self) -> &'static str {
        match self {
            JavaPackage::Bool => "java.lang.Boolean",
            JavaPackage::String => "java.lang.String",
            JavaPackage::Integer => "java.lang.Integer",
            JavaPackage::Long => "java.lang.Long",
            JavaPackage::BigDecimal => "java.lang.math.BigDecimal",
            JavaPackage::Float => "java.lang.Float",
            JavaPackage::Double => "java.lang.Double",
            JavaPackage::LocalDate => "java.lang.LocalDate",
            JavaPackage::LocalDatetime => "java.lang.LocalDateTime",
            JavaPackage::Serializer => "java.io.Serializable",
            JavaPackage::TableName => "com.baomidou.mybatisplus.annotation.TableName",
            JavaPackage::TableId => "com.baomidou.mybatisplus.annotation.TableId",
            JavaPackage::TableField => "com.baomidou.mybatisplus.annotation.TableField",
            JavaPackage::TableLogic => "com.baomidou.mybatisplus.annotation.TableLogic",
            JavaPackage::Version => "com.baomidou.mybatisplus.annotation.Version",
            JavaPackage::IService => "com.baomidou.mybatisplus.extension.service.IService",
            JavaPackage::ServiceImpl => {
                "com.baomidou.mybatisplus.extension.service.impl.ServiceImpl"
            }
            JavaPackage::BaseMapper => "com.baomidou.mybatisplus.core.mapper.BaseMapper",
            JavaPackage::FieldFill => "com.baomidou.mybatisplus.annotation.FieldFill",
            JavaPackage::IdType => "com.baomidou.mybatisplus.annotation.IdType",
        }
    }
}

impl From<JavaType> for JavaPackage {
    fn from(value: JavaType) -> Self {
        match value {
            JavaType::Bool => JavaPackage::Bool,
            JavaType::Integer => JavaPackage::Integer,
            JavaType::Long => JavaPackage::Long,
            JavaType::BigDecimal => JavaPackage::BigDecimal,
            JavaType::Float => JavaPackage::Float,
            JavaType::Double => JavaPackage::Double,
            JavaType::String => JavaPackage::String,
            JavaType::Date => JavaPackage::LocalDate,
            JavaType::Datetime => JavaPackage::LocalDatetime,
        }
    }
}

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
