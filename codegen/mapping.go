package codegen

import (
	"encoding/json"

	"gorm.io/gorm"
)

const (
	DbVarchar   = "varchar"
	DbChar      = "char"
	DbLongtext  = "longtext"
	DbTinyint   = "tinyint"
	DbInt       = "int"
	DbInteger   = "integer"
	DbBigint    = "bigint"
	DbDecimal   = "decimal"
	DbFloat     = "float"
	DbDouble    = "double"
	DbTimestamp = "timestamp"
	DbDate      = "date"
	DbDatetime  = "datetime"
)

// java类型的包路径
const (
	BooleanPackage       = "java.lang.Boolean"
	StringPackage        = "java.lang.String"
	IntegerPackage       = "java.lang.Integer"
	LongPath             = "java.lang.Long"
	BigDecimalPackage    = "java.lang.math.BigDecimal"
	FloatPackage         = "java.lang.Float"
	DoublePackage        = "java.lang.Double"
	LocalDatePackage     = "java.lang.LocalDate"
	LocalDateTimePackage = "java.lang.LocalDateTime"
	SerializablePackage  = "java.io.Serializable"
	TableNamePackage     = "com.baomidou.mybatisplus.annotation.TableName"
	TableIdPackage       = "com.baomidou.mybatisplus.annotation.TableId"
	TableFieldPackage    = "com.baomidou.mybatisplus.annotation.TableField"
	TableLogicPackage    = "com.baomidou.mybatisplus.annotation.TableLogic"
	VersionPackage       = "com.baomidou.mybatisplus.annotation.Version"
	IServicePackage      = "com.baomidou.mybatisplus.extension.service.IService"
	ServiceImplPackage   = "com.baomidou.mybatisplus.extension.service.impl.ServiceImpl"
	BaseMapperPackage    = "com.baomidou.mybatisplus.core.mapper.BaseMapper"
	FieldFillPackage     = "com.baomidou.mybatisplus.annotation.FieldFill"
	IdTypePackage        = "com.baomidou.mybatisplus.annotation.IdType"
)

const (
	FieldFillDefault      = "DEFAULT"
	FieldFillInsert       = "INSERT"
	FieldFillUpdate       = "UPDATE"
	FieldFillInsertUpdate = "INSERT_UPDATE"
)

const (
	JavaString        = "String"
	JavaBoolean       = "Boolean"
	JavaInteger       = "Integer"
	JavaLong          = "Long"
	JavaBigDecimal    = "BigDecimal"
	JavaFloat         = "Float"
	JavaDouble        = "Double"
	JavaLocalDate     = "LocalDate"
	JavaLocalDateTime = "LocalDateTime"
)

var (
	TableQuerySql = `SELECT
	table_name 'name',
	IFNULL( TABLE_COMMENT, table_name ) 'comment',
	TABLE_SCHEMA 'table_schema'
FROM
	INFORMATION_SCHEMA.TABLES 
WHERE
	UPPER( table_type )= 'BASE TABLE' 
	AND LOWER( table_schema ) = ? 
	AND table_name IN (?);
`
	ColumnQuerySql = `SELECT
	COLUMN_NAME 'name',
	column_comment 'comment',
	DATA_TYPE 'data_type',
	IS_NULLABLE 'is_nullable',
	IFNULL( CHARACTER_MAXIMUM_LENGTH, 0 ) 'length',
    COLUMN_KEY 'key_flag' 
FROM
	information_schema.COLUMNS 
WHERE
	table_schema = ? 
	AND table_name = ?;
`
)

// TypeMappings 数据库和java类型的映射集合
var TypeMappings = map[string]string{
	DbVarchar:   JavaString,
	DbChar:      JavaString,
	DbLongtext:  JavaString,
	DbTinyint:   JavaBoolean,
	DbInt:       JavaInteger,
	DbInteger:   JavaInteger,
	DbBigint:    JavaLong,
	DbDecimal:   JavaBigDecimal,
	DbFloat:     JavaFloat,
	DbDouble:    JavaDouble,
	DbTimestamp: JavaLong,
	DbDate:      JavaLocalDate,
	DbDatetime:  JavaLocalDateTime,
}

// TypePackages Java类型映射
var TypePackages = map[string]string{
	JavaString:        StringPackage,
	JavaBoolean:       BooleanPackage,
	JavaInteger:       IntegerPackage,
	JavaLong:          LongPath,
	JavaBigDecimal:    BigDecimalPackage,
	JavaFloat:         FloatPackage,
	JavaDouble:        DoublePackage,
	JavaLocalDate:     LocalDatePackage,
	JavaLocalDateTime: LocalDateTimePackage,
}

// TableInfo 数据库表信息
type TableInfo struct {
	Name       string    // 表名
	Comment    string    // 描述
	SchemaName string    // 表所属数据库
	Columns    []*Column // 表所有列信息
}

// Column 列信息
type Column struct {
	Name       string // 列名
	Comment    string // 描述
	DataType   string // 数据库类型
	IsNullable string // 是否可为空
	Length     int    // 类型长度
	KeyFlag    string // 是否是主键PRI
}

func NewTableInfo(name, tableSchema string) *TableInfo {
	return &TableInfo{
		Name:       name,
		SchemaName: tableSchema,
	}
}

func (t *TableInfo) ExecuteColumns(db *gorm.DB) error {
	db = db.Raw(ColumnQuerySql, t.SchemaName, t.Name)
	if db.Error != nil {
		return db.Error
	}
	var columns []*Column

	rows, _ := db.Rows()
	defer rows.Close()

	for rows.Next() {
		column := Column{}
		_ = db.ScanRows(rows, &column)
		columns = append(columns, &column)
	}

	t.Columns = columns
	return nil
}

func (t *TableInfo) String() string {
	data, _ := json.MarshalIndent(t, "", "  ")
	return string(data)
}
