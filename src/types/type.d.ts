export interface MpConfig {
  datasourceConfig: DatasourceConfig
  globalConfig: GlobalConfig
  packageConfig: PackageConfig
  templateConfig: TemplateConfig
  strategyConfig: StrategyConfig
}

export interface DatasourceConfig {
  type: string
  database: string
  host: string
  port: number
  username: string
  password: string
}

export interface GlobalConfig {
  outputDir: string
  open: boolean
  author: string
  kotlin: boolean
  swagger: boolean
  springdoc: boolean
  dateType: DateType
  commentDate: string
}

export enum DataType {
  ONLY_DATE,
  SQL_PACK,
  TIME_PACK
}

export interface PackageConfig {
  parent: string
  moduleName: string
  entity: string
  service: string
  serviceImpl: string
  mapper: string
  xml: string
  controller: string
  pathinfo?: Map<OutputFile, string>
  packageInfos?: Map<string, string>
}

export enum OutputFile {
  ENTITY,
  SERVICE,
  SERVICE_IMPL,
  MAPPER,
  XML,
  CONTROLLER,
  PARENT
}

export interface TemplateConfig {
  entity: string
  entityKt: string
  controller: string
  mapper: string
  xml: string
  service: string
  serviceImpl: string
  disableEntity: boolean
}

export interface StrategyConfig {
  isCapitalMode: boolean
  skipView: boolean
  tablePrefix: Set<string>
  tablesuffix: Set<string>
  fieldPrefix: Set<string>
  fieldsuffix: Set<string>
  include: Set<string>
  exclude: Set<string>
  enableSqlFilter: boolean
  enableSchema: boolean
  entity: Entity
  controller: Controller
  mapper: Mapper
  service: Service
}

export interface Entity {
  superClass: string
  superEntityColumns: Set<string>
  ignoreColumns: Set<string>
  disableSerialVersionUid: boolean
  columnContant: boolean
  chainMode: boolean
  lombok: boolean
  booleanColumnRemoveIsPrefix: boolean
  tableFieldAnnotationEnable: boolean
  versionColumnName: string
  versionPropertyName: string
  logicDeleteColumnName: string
  logicDeletePropertyName: string
  tableFillList: TableFill[]
  naming: NamingStrategy
  columnNaming?: NamingStrategy
  activeRecord: boolean
  idType: IdType
  fileOverride: boolean
  formatFilename: string
}

export interface Controller {
  restStyle: boolean
  hyphenStyle: boolean
  superClass: string
  fileOverride: boolean
  formatFilename: string
}

export interface Mapper {
  superClass: string
  mapperAnnotation: boolean
  mapperAnnotationClass?: String
  baseResultMap: boolean
  baseColumnList: boolean
  fileOverride: boolean
  formatMapperFilename: string
  formatXmlFilename: string
}

export interface Service {
  superServiceClass: string
  superServiceImplClass: string
  fileOverride: boolean
  formatServiceFilename: string
  formatServiceImplFilename: string
}

export interface TableFill {
  key: string
  value: string
}

export enum NamingStrategy {
  NO_CHANGE,
  UNDERLINE_TO_CAMEL
}

export enum IdType {
    /**
     * 数据库ID自增, 该类型请确保数据库设置了 ID自增 否则无效
     */
    AUTO,
    /**
     * 该类型为未设置主键类型(注解里等于跟随全局,全局里约等于 INPUT)
     */
    NONE,
    /**
     * 用户输入ID, 该类型可以通过自己注册自动填充插件进行填充
     */
    INPUT,
    /** 
     * 以下2种类型、只有当插入对象ID 为空，才自动填充。
     * 分配ID (主键类型为number或string）, 默认实现类 com.baomidou.mybatisplus.core.incrementer.DefaultIdentifierGenerator(雪花算法)
     */
    ASSIGN_ID,
    /**
     * 分配UUID (主键类型为 string) 默认实现类 com.baomidou.mybatisplus.core.incrementer.DefaultIdentifierGenerator(UUID.replace("-",""))
     */
    ASSIGN_UUID,
}
