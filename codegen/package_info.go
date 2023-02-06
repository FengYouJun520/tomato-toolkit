package codegen

import "encoding/json"

// TemplateData 模板所需的数据信息
type TemplateData struct {
	RestControllerStyle          bool
	Author                       string
	ActiveRecord                 bool
	Date                         string
	Kotlin                       bool
	Swagger                      bool
	EntityLombokModel            bool
	ChainModel                   bool
	EnableCache                  bool
	BaseColumnList               bool
	BaseResultMap                bool
	CacheClassName               string
	SuperEntityClass             string
	IdType                       string
	SchemaName                   string
	Entity                       string
	EntitySerialVersionUID       bool
	EntityColumnConstant         bool
	MapperAnnotation             bool
	SuperControllerClassPackage  string
	SuperControllerClass         string
	SuperMapperClassPackage      string
	SuperMapperClass             string
	SuperServiceClassPackage     string
	SuperServiceClass            string
	SuperServiceImplClassPackage string
	SuperServiceImplClass        string
	ControllerMappingHyphenStyle bool
	ControllerMappingHyphen      string
	Package                      *Package
	Table                        *Table
}

// Package 包配置信息
type Package struct {
	Controller  string
	Entity      string
	Mapper      string
	Service     string
	ServiceImpl string
	ModuleName  string
}

// Table 表信息
type Table struct {
	Name            string
	Comment         string
	ControllerName  string
	EntityPath      string
	Convert         bool
	SchemaName      string
	Fields          []Field
	CommonFields    []Field
	MapperName      string
	FieldNames      []string
	ServiceName     string
	ServiceImplName string
	ImportPackages  []string
}

// Field 表字段信息
type Field struct {
	ColumnName           string
	KeyFlag              bool
	Comment              string
	KeyIdentityFlag      bool
	Convert              bool
	AnnotationColumnName string
	Fill                 string
	VersionField         bool
	LogicDeleteField     bool
	PropertyType         string
	PropertyName         string
	CapitalName          string
}

func (td *TemplateData) String() string {
	data, _ := json.MarshalIndent(td, "", "  ")
	return string(data)
}

func (t *Table) String() string {
	data, _ := json.MarshalIndent(t, "", "  ")
	return string(data)
}

func (f *Field) String() string {
	data, _ := json.MarshalIndent(f, "", "  ")
	return string(data)
}
