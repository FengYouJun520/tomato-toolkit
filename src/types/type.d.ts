
export interface MpConfig {
  datasource: DatasourceConfig
  global: GlobalConfig
  injection?: InjectConfig
  package: PackageConfig
  template: TemplateConfig
  strategy: StrategyConfig
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
  kotlin: boolean
  swagger: boolean
  springdoc: boolean
  dateType: DataType
  commentDate: string
}

export interface InjectConfig {
  custom_map?: Record<string, string>
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
  entity: string
  entityKt: string
  controller: string
  mapper: string
  xml: string
  service: string
  serviceImpl: string
  disableEntity: boolean
  disableController: boolean
  disableMapper: boolean
  disableXml: boolean
  disableService: boolean
  disableServiceImpl: boolean
}

export interface StrategyConfig {
  isCapitalMode: boolean
  skipView: boolean
  tablePrefix: string[]
  tableSuffix: string[]
  fieldPrefix: string[]
  fieldSuffix: string[]
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
  serialVersionUid: boolean
  columnConstant: boolean
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
  propertyName: string
  fieldFill: FieldFill
}

export type NamingStrategy = 'NoChange' | 'UnderlineToCamel'

export type IdType = 'AUTO' | 'NONE'|'INPUT'|'ASSIGN_ID'|'ASSIGN_UUID'
export type FieldFill = 'DEFAULT'|'INSERT'|'UPDATE'|'INSERT_UPDATE'
