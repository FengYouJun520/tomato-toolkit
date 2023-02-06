package codegen

import (
	"embed"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"text/template"
	"time"

	"tomoto/util"

	"gorm.io/driver/mysql"
	"gorm.io/gorm"
)

const (
	OnlyDate         = "ONLY_DATE"
	TimePack         = "TIME_PACK"
	underlineToCamel = "underline_to_camel"
	AUTO             = "AUTO"
	NONE             = "NONE"
	INPUT            = "INPUT"
	AssignId         = "ASSIGN_ID"
	AssignUuid       = "ASSIGN_UUID"
)

// Manager 代码生成管理器
type Manager struct {
	config     *ConfigContext // 代码生成所需配置信息
	templates  embed.FS
	TableInfos []*TableInfo // 所有表信息
	t          *template.Template
	db         *gorm.DB
}

func NewManager(templates embed.FS) *Manager {
	return &Manager{
		templates: templates,
	}
}

func (m *Manager) init() error {
	if err := m.initTemplate(); err != nil {
		return err
	}
	if err := m.initDb(); err != nil {
		return err
	}

	return nil
}

var funcMaps = template.FuncMap{
	"ToUpper": strings.ToUpper,
}

func (m *Manager) initTemplate() error {
	t := template.New("mybatis-code-generator").Funcs(funcMaps)
	t, err := t.ParseFS(m.templates, "templates/*.tmpl")
	if err != nil {
		return err
	}
	m.t = t
	return nil
}

func (m *Manager) initDb() error {
	var (
		dsn = ""
		db  *gorm.DB
		err error
	)

	switch m.config.DataSource.Typ {
	case "mysql":
		dsn = fmt.Sprintf("%v:%v@tcp(%v:%v)/%v?charset=utf8mb4&parseTime=True&loc=Local&timeout=5s",
			m.config.DataSource.Username, m.config.DataSource.Password, m.config.DataSource.Host,
			m.config.DataSource.Port, m.config.DataSource.Database)
		db, err = gorm.Open(mysql.Open(dsn), &gorm.Config{})
		if err != nil {
			return err
		}
	}

	m.db = db
	return nil
}

func (m *Manager) mappingTableInfos() error {
	db := m.db
	tables := m.config.StrategyConfig.AddIncludes
	tableSchema := m.config.DataSource.Database
	m.TableInfos = make([]*TableInfo, 0, len(tables))

	db = db.Raw(TableQuerySql, tableSchema, tables)
	rows, _ := db.Rows()
	defer rows.Close()

	// 获取表信息
	for rows.Next() {
		var tableInfo TableInfo
		_ = rows.Scan(&tableInfo.Name, &tableInfo.Comment, &tableInfo.SchemaName)
		m.TableInfos = append(m.TableInfos, &tableInfo)
	}

	// 获取列信息
	for _, tableInfo := range m.TableInfos {
		if err := tableInfo.ExecuteColumns(db); err != nil {
			return err
		}
	}

	return nil
}

func (m *Manager) columnsInfo() {
	for _, tableInfo := range m.TableInfos {
		_ = tableInfo.ExecuteColumns(m.db)
	}
}

// CodeGenerate 代码生成
func (m *Manager) CodeGenerate(config *ConfigContext) (string, error) {
	m.config = config
	strategy := m.config.StrategyConfig

	// 初始化模板和db
	if err := m.init(); err != nil {
		return "", err
	}

	// 获取数据库元信息
	if err := m.mappingTableInfos(); err != nil {
		return "", err
	}

	// 1.生成 entity
	var importPackages []string

	m.onCodeGenerate(importPackages, config.PackageConfig.Entity, strategy.Entity.FormatFileName, "entity.java.tmpl", ".java")

	importPackages = []string{}
	m.onCodeGenerate(importPackages, config.PackageConfig.Controller, strategy.Controller.FormatFileName, "controller.java.tmpl", ".java")
	m.onCodeGenerate(importPackages, config.PackageConfig.Service, strategy.Service.FormatServiceFileName, "service.java.tmpl", ".java")
	m.onCodeGenerate(importPackages, config.PackageConfig.ServiceImpl, strategy.Service.FormatServiceImplFileName, "service.impl.java.tmpl", ".java")
	m.onCodeGenerate(importPackages, config.PackageConfig.Mapper, strategy.Mapper.FormatMapperFileName, "mapper.java.tmpl", ".java")
	m.onCodeGenerate(importPackages, config.PackageConfig.MapperXml, strategy.Mapper.FormatXmlFileName, "mapper.xml.tmpl", ".xml")

	if !config.GlobalConfig.DisableOpenDir {
		if err := openDir(config.GlobalConfig.OutputDir); err != nil {
			return "", nil
		}
	}

	return "代码生成完成！", nil
}

func openDir(dir string) error {
	dir = filepath.Clean(dir)

	switch runtime.GOOS {
	case "windows":
		return exec.Command("explorer", dir).Start()
	case "darwin":
		return exec.Command("open", dir).Start()
	default:
		return errors.New(runtime.GOOS + "系统暂不支持打开文件夹命令")
	}
}

func (m *Manager) onCodeGenerate(importPackages []string, packageName, FormatFileName, templateName, suffix string) {
	config := m.config
	pack := strings.Join([]string{config.PackageConfig.Parent, packageName}, ".")
	pack = strings.ReplaceAll(pack, ".", "/")
	path := filepath.Join(config.GlobalConfig.OutputDir, pack)
	fileOverride := config.GlobalConfig.FileOverride
	_ = os.MkdirAll(path, 0666)

	// 遍历要生成的表
	for _, tableInfo := range m.TableInfos {
		className := util.SnakeCaseToTitleCamel(tableInfo.Name)
		path := filepath.Join(path, fmt.Sprintf(FormatFileName, className)+suffix)
		// 文件已存在，如果不覆盖则跳过
		if m.skipCreateFile(path, fileOverride) {
			continue
		}
		// 生成模板需要的数据
		templateData := m.transformTemplateData(tableInfo, className, importPackages)

		file, _ := os.OpenFile(path, os.O_CREATE|os.O_TRUNC|os.O_WRONLY, 0666)
		// 执行渲染
		_ = m.t.ExecuteTemplate(file, templateName, templateData)

		_ = file.Close()
	}
}

func (m *Manager) skipCreateFile(path string, fileOverride bool) bool {
	_, err := os.Stat(path)
	return err == nil && !fileOverride
}

// transformTemplateData 将配置信息转为模板数据
func (m *Manager) transformTemplateData(tableInfo *TableInfo, className string, importPackages []string) *TemplateData {
	config := m.config
	global := config.GlobalConfig
	strategy := config.StrategyConfig
	// 获取entity通用父类
	entitySuperClassPackage := strategy.Entity.SuperClass
	entitySuperClass := getSuperClass(entitySuperClassPackage)
	if entitySuperClass != "" {
		importPackages = append(importPackages, entitySuperClassPackage)
	}
	if !strategy.Entity.DisableSerialVersionUID {
		importPackages = append(importPackages, SerializablePackage)
	}
	// 获取controller通用父类
	controllerSuperClass := getSuperClass(strategy.Controller.SuperClass)
	// 获取service通用父类，默认是IService，由mybatis-plus提供
	serviceSuperClassPackage := strategy.Service.SuperServiceClass
	serviceSuperClass := getSuperClass(serviceSuperClassPackage)
	if serviceSuperClass == "" {
		serviceSuperClass = "IService"
		serviceSuperClassPackage = IServicePackage
	}
	// 获取service实现类的通用父类，默认是ServiceImpl，由mybatis-plus提供
	serviceImplSuperClassPackage := strategy.Service.SuperServiceImplClass
	serviceImplSuperClass := getSuperClass(serviceImplSuperClassPackage)
	if serviceImplSuperClass == "" {
		serviceImplSuperClass = "ServiceImpl"
		serviceImplSuperClassPackage = ServiceImplPackage
	}
	// 获取mapper的通用父类，默认是BaseMapper，由mybatis-plus提供
	mapperSuperClassPackage := strategy.Mapper.SuperClass
	mapperSuperClass := getSuperClass(mapperSuperClassPackage)
	if mapperSuperClass == "" {
		mapperSuperClass = "BaseMapper"
		mapperSuperClassPackage = BaseMapperPackage
	}
	// 是否开启schema
	schemaName := ""
	if strategy.EnableSchema {
		schemaName = config.DataSource.Database + "_"
	}

	table := m.createTable(tableInfo, className, importPackages)

	templateData := &TemplateData{
		Author:                       global.Author,
		ActiveRecord:                 strategy.Entity.EnableActiveRecord,
		RestControllerStyle:          strategy.Controller.EnableRestStyle,
		Date:                         time.Now().Format("2006-01-03"),
		Kotlin:                       global.EnableKotlin,
		Swagger:                      global.EnableSwagger,
		EntityLombokModel:            strategy.Entity.EnableLombok,
		ChainModel:                   strategy.Entity.EnableChainModel,
		Entity:                       fmt.Sprintf(strategy.Entity.FormatFileName, className),
		EntitySerialVersionUID:       !strategy.Entity.DisableSerialVersionUID,
		EntityColumnConstant:         strategy.Entity.EnableColumnConstant,
		SuperEntityClass:             entitySuperClass,
		SuperControllerClassPackage:  strategy.Controller.SuperClass,
		SuperControllerClass:         controllerSuperClass,
		SuperServiceClassPackage:     serviceSuperClassPackage,
		SuperServiceClass:            serviceSuperClass,
		SuperServiceImplClassPackage: serviceImplSuperClassPackage,
		SuperServiceImplClass:        serviceImplSuperClass,
		SuperMapperClassPackage:      mapperSuperClassPackage,
		SuperMapperClass:             mapperSuperClass,
		MapperAnnotation:             strategy.Mapper.EnableMapperAnnotation,
		ControllerMappingHyphenStyle: strategy.Controller.EnableHyphenStyle,
		ControllerMappingHyphen:      util.CamelToKebabCase(table.EntityPath),
		IdType:                       strategy.Entity.IdType,
		SchemaName:                   schemaName,
		EnableCache:                  false,
		BaseColumnList:               false,
		BaseResultMap:                false,
		CacheClassName:               "",
		Package:                      m.createPackage(tableInfo, className),
		Table:                        table,
	}

	return templateData
}

func (m *Manager) createPackage(tableInfo *TableInfo, className string) *Package {
	config := m.config
	pkg := config.PackageConfig

	pack := &Package{
		Controller:  strings.Join([]string{pkg.Parent, pkg.Controller}, "."),
		Entity:      strings.Join([]string{pkg.Parent, pkg.Entity}, "."),
		Mapper:      strings.Join([]string{pkg.Parent, pkg.Mapper}, "."),
		Service:     strings.Join([]string{pkg.Parent, pkg.Service}, "."),
		ServiceImpl: strings.Join([]string{pkg.Parent, pkg.ServiceImpl}, "."),
		ModuleName:  pkg.ModuleName,
	}

	return pack
}

func (m *Manager) createTable(tableInfo *TableInfo, className string, importPackages []string) *Table {
	config := m.config
	strategy := config.StrategyConfig
	addFills := strategy.Entity.AddTableFills
	superEntityColumns := strategy.Entity.AddSuperEntityColumns
	addIgnoreColumns := strategy.Entity.AddIgnoreColumns
	columns := tableInfo.Columns
	var fieldNames []string
	var fields []Field

	for _, column := range columns {
		// 如果当前字段是父类公共字段或在忽略字段中，则不存储字段信息
		if _, ok := util.InSlice(superEntityColumns, column.Name); ok {
			continue
		}
		if _, ok := util.InSlice(addIgnoreColumns, column.Name); ok {
			continue
		}

		fieldNames = append(fieldNames, column.Name)
		dataType := TypeMappings[column.DataType]
		// 自动填充字段
		fieldFills := util.MapTo(addFills, func(k FieldTypeKeyVal) string {
			return k.Name
		})
		fill, ok := util.InSlice(fieldFills, column.Name)

		if ok {
			// 防止重复填充
			res, _ := util.First(addFills, func(t FieldTypeKeyVal) bool {
				return t.Name == column.Name
			})
			// 填充字段类型
			fill = res.Value
		}

		// 是否是主键
		keyFlag := column.KeyFlag == "PRI"

		propertyName := util.SnakeCaseToCamel(column.Name)
		capitalName := column.Name
		// getter,setter方法是否移除Boolean类型的is前缀
		if strings.HasPrefix(capitalName, "is_") &&
			dataType == JavaBoolean &&
			strategy.Entity.EnableRemoveIsPrefix {
			capitalName = capitalName[3:]
		}
		capitalName = util.SnakeCaseToTitleCamel(capitalName)
		// 是否开启乐观锁
		versionField := strategy.Entity.VersionPropertyName == propertyName ||
			strategy.Entity.VersionColumnName == column.Name

		// 是否开启逻辑删除
		logicDeleteField := strategy.Entity.LogicDeleteColumnName == column.Name ||
			strategy.Entity.LogicDeletePropertyName == propertyName

		field := Field{
			PropertyType:         dataType,
			PropertyName:         propertyName,
			ColumnName:           column.Name,
			Comment:              column.Comment,
			KeyIdentityFlag:      false,
			KeyFlag:              keyFlag,
			Fill:                 fill,
			Convert:              true,
			AnnotationColumnName: column.Name,
			VersionField:         versionField,
			LogicDeleteField:     logicDeleteField,
			CapitalName:          capitalName,
		}
		fields = append(fields, field)
		// 添加所需要的包
		addImportPackage(field, dataType, importPackages, strategy)
	}

	convert := true
	if convert {
		importPackages = append(importPackages, TableNamePackage)
	}

	t := &Table{
		Name:            tableInfo.Name,
		Comment:         tableInfo.Comment,
		ControllerName:  fmt.Sprintf(strategy.Controller.FormatFileName, className),
		EntityPath:      util.SnakeCaseToCamel(tableInfo.Name),
		Fields:          fields,
		CommonFields:    []Field{},
		ImportPackages:  importPackages,
		SchemaName:      tableInfo.SchemaName,
		MapperName:      fmt.Sprintf(strategy.Mapper.FormatMapperFileName, className),
		FieldNames:      fieldNames,
		Convert:         convert,
		ServiceName:     fmt.Sprintf(strategy.Service.FormatServiceFileName, className),
		ServiceImplName: fmt.Sprintf(strategy.Service.FormatServiceImplFileName, className),
	}

	return t
}

func addImportPackage(field Field, dataType string, importPackages []string, strategy *StrategyConfig) {
	if importPackage := importPackagesByType(dataType); importPackage != "" {
		if _, ok := util.InSlice(importPackages, importPackage); ok {
			importPackages = append(importPackages, importPackage)
		}
	}
	if field.VersionField {
		if _, ok := util.InSlice(importPackages, VersionPackage); !ok {
			importPackages = append(importPackages, VersionPackage)
		}
	}
	if field.LogicDeleteField {
		if _, ok := util.InSlice(importPackages, TableLogicPackage); !ok {
			importPackages = append(importPackages, TableLogicPackage)
		}
	}
	if field.Fill != "" || field.Convert {
		if _, ok := util.InSlice(importPackages, TableFieldPackage); !ok {
			importPackages = append(importPackages, TableFieldPackage)
		}
	}
	if field.KeyIdentityFlag || field.Convert || strategy.Entity.IdType != "" {
		if _, ok := util.InSlice(importPackages, TableIdPackage); !ok {
			importPackages = append(importPackages, TableIdPackage)
		}
		if _, ok := util.InSlice(importPackages, IdTypePackage); !ok {
			importPackages = append(importPackages, IdTypePackage)
		}
	}
	if field.Fill != "" {
		if _, ok := util.InSlice(importPackages, FieldFillPackage); !ok {
			field.Fill = FieldFillDefault
			importPackages = append(importPackages, FieldFillPackage)
		}
	}
}

// importPackagesByType 标准包的路径导入
func importPackagesByType(typ string) string {
	if path, ok := TypePackages[typ]; ok {
		return path
	}
	return ""
}

func getSuperClass(superClassPack string) (superEntityClass string) {
	if superClassPack == "" {
		return
	}
	superClasses := strings.Split(superClassPack, ".")
	if len(superClasses) > 1 {
		// BaseModel
		superEntityClass = superClasses[len(superClasses)-1]
	} else {
		superEntityClass = util.SnakeCaseToTitleCamel(superClassPack)
	}
	return
}
