package codegen

import "encoding/json"

// ConfigContext 配置信息上下文
type ConfigContext struct {
    DataSource     *DataSourceConfig `json:"dataSource"`
    GlobalConfig   *GlobalConfig     `json:"globalConfig"`
    PackageConfig  *PackageConfig    `json:"packageConfig"`
    TemplateConfig *TemplateConfig   `json:"templateConfig"`
    StrategyConfig *StrategyConfig   `json:"strategyConfig"`
}

func (cc *ConfigContext) String() string {
    data, _ := json.MarshalIndent(cc, "", "  ")
    return string(data)
}

func (dsc *DataSourceConfig) String() string {
    data, _ := json.MarshalIndent(dsc, "", "  ")
    return string(data)
}

// DataSourceConfig 数据源配置
type DataSourceConfig struct {
    Typ      string `json:"typ"`
    Database string `json:"database"`
    Username string `json:"username"`
    Password string `json:"password"`
    Host     string `json:"host"`
    Port     int    `json:"port"`
}

// GlobalConfig 全局配置
type GlobalConfig struct {
    FileOverride   bool   `json:"fileOverride"`
    DisableOpenDir bool   `json:"disableOpenDir"`
    OutputDir      string `json:"outputDir"`
    Author         string `json:"author"`
    EnableKotlin   bool   `json:"enableKotlin"`
    EnableSwagger  bool   `json:"enableSwagger"`
    DateType       string `json:"dateType"`
    CommentDate    string `json:"commentDate"`
}

// PathInfo 路径信息
type PathInfo struct {
    Name  string `json:"name"`
    Value string `json:"value"`
}

// PackageConfig 包配置
type PackageConfig struct {
    Parent      string     `json:"parent"`
    ModuleName  string     `json:"moduleName"`
    Entity      string     `json:"entity"`
    Service     string     `json:"service"`
    ServiceImpl string     `json:"serviceImpl"`
    Mapper      string     `json:"mapper"`
    MapperXml   string     `json:"mapperXml"`
    Controller  string     `json:"controller"`
    Other       string     `json:"other"`
    PathInfo    []PathInfo `json:"pathInfo"`
}

// TemplateConfig 模板配置
type TemplateConfig struct {
    DisableAll  bool   `json:"disableAll"`
    Disable     bool   `json:"disable"`
    Entity      string `json:"entity"`
    EntityKt    string `json:"entityKt"`
    Service     string `json:"service"`
    ServiceImpl string `json:"serviceImpl"`
    Mapper      string `json:"mapper"`
    MapperXml   string `json:"mapperXml"`
    Controller  string `json:"controller"`
}

// InjectionConfig 注入配置
type InjectionConfig struct {
}

// StrategyConfig 生成策略配置
type StrategyConfig struct {
    EnableCapitalMode bool        `json:"enableCapitalMode"`
    EnableSkipView    bool        `json:"enableSkipView"`
    DisableSqlFilter  bool        `json:"disableSqlFilter"`
    EnableSchema      bool        `json:"enableSchema"`
    AddIncludes       []string    `json:"addIncludes"`
    Entity            *Entity     `json:"entity"`
    Controller        *Controller `json:"controller"`
    Mapper            *Mapper     `json:"mapper"`
    Service           *Service    `json:"service"`
}

// FieldTypeKeyVal 自动填充key-value
type FieldTypeKeyVal struct {
    Name  string `json:"name"`
    Value string `json:"value"`
}

// Entity 实体类配置
type Entity struct {
    SuperClass                 string            `json:"superClass"`
    DisableSerialVersionUID    bool              `json:"disableSerialVersionUID"`
    EnableColumnConstant       bool              `json:"enableColumnConstant"`
    EnableChainModel           bool              `json:"enableChainModel"`
    EnableLombok               bool              `json:"enableLombok"`
    EnableRemoveIsPrefix       bool              `json:"enableRemoveIsPrefix"`
    EnableTableFieldAnnotation bool              `json:"enableTableFieldAnnotation"`
    EnableActiveRecord         bool              `json:"enableActiveRecord"`
    VersionColumnName          string            `json:"versionColumnName"`
    VersionPropertyName        string            `json:"versionPropertyName"`
    LogicDeleteColumnName      string            `json:"logicDeleteColumnName"`
    LogicDeletePropertyName    string            `json:"logicDeletePropertyName"`
    Naming                     string            `json:"naming"`
    AddSuperEntityColumns      []string          `json:"addSuperEntityColumns"`
    AddIgnoreColumns           []string          `json:"addIgnoreColumns"`
    AddTableFills              []FieldTypeKeyVal `json:"addTableFills"`
    IdType                     string            `json:"idType"`
    FormatFileName             string            `json:"formatFileName"`
}

// Controller Controller类配置
type Controller struct {
    SuperClass        string `json:"superClass"`
    EnableHyphenStyle bool   `json:"enableHyphenStyle"`
    EnableRestStyle   bool   `json:"enableRestStyle"`
    FormatFileName    string `json:"formatFileName"`
}

// Mapper  Mapper类配置
type Mapper struct {
    SuperClass             string `json:"superClass"`
    EnableMapperAnnotation bool   `json:"enableMapperAnnotation"`
    FormatMapperFileName   string `json:"formatMapperFileName"`
    FormatXmlFileName      string `json:"formatXmlFileName"`
}

// DatabaseOptions 数据库信息
type DatabaseOptions struct {
    Name    string `json:"name"`
    Comment string `json:"comment"`
}

// Service Service类配置
type Service struct {
    SuperServiceClass         string `json:"superServiceClass"`
    SuperServiceImplClass     string `json:"superServiceImplClass"`
    FormatServiceFileName     string `json:"formatServiceFileName"`
    FormatServiceImplFileName string `json:"formatServiceImplFileName"`
}
