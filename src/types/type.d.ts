export interface MpConfig {
  datasourceConfig: DatasourceConfig
  globalConfig: GlobalConfig
  packageConfig: PackageConfig
  templateConfig: TemplateConfig
  strategyConfig: StrategyConfig
}

export interface BasicTableInfo {
  name: string
  comment: string
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
  fileOverride: boolean
  kotlin: boolean
  swagger: boolean
  springdoc: boolean
  dateType: DataType
  commentDate: string
}

export type DataType = 'ONLY_DATE' | 'SQL_PACK' | 'TIME_PACK'

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

export type OutputFile = 'entity'|'service'|'serviceImpl'|'mapper'|'xml'|'controller'|'parent'

export interface TemplateConfig {
  disable: boolean
  entity: string
  entityKt: string
  controller: string
  mapper: string
  xml: string
  service: string
  serviceImpl: string
}

export interface StrategyConfig {
  isCapitalMode: boolean
  skipView: boolean
  tablePrefix: string[]
  tablesuffix: string[]
  fieldPrefix: string[]
  fieldsuffix: string[]
  include: string[]
  exclude: string[]
  enableSqlFilter: boolean
  enableSchema: boolean
  entity: Entity
  controller: Controller
  mapper: Mapper
  service: Service
}

export interface Entity {
  superClass: string
  superEntityColumns: string[]
  ignoreColumns: string[]
  disableSerialVersionUid: boolean
  columnContant: boolean
  chainMode: boolean
  lombok: boolean
  booleanColumnRemoveIsPrefix: boolean
  enableTableFieldAnnotation: boolean
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

export type NamingStrategy = 'noChange' | 'underlineToCamel'

export type IdType = 'auto' | 'none'|'input'|'assignId'|'assignUuid'
