export type databaseType = 'mysql' | 'postgres' | 'sqlite'
export interface DataSourceConfig {
  typ: databaseType
  database: string
  username: string
  password: string
  host: string
  port: number
}

export type DateType = 'ONLY_DATE ' | 'TIME_PACK'

export interface GlobalConfig {
  fileOverride: boolean
  disableOpenDir: boolean
  outputDir: string
  author: string
  enableKotlin: boolean
  enableSwagger: boolean
  dateType: DateType
  commentDate: string
}

export type PathInfo = {
  name: string
  value: string
}

export interface PackageConfig {
  parent: string
  moduleName: string
  entity: string
  service: string
  serviceImpl: string
  mapper: string
  mapperXml: string
  controller: string
  other: string
  pathInfo: PathInfo[]
}

export enum TemplateType {
  ENTITY,
  SERVICE,
  SERVICEIMPL,
  CONTROLLER,
  MAPPER,
  XML,
}

export interface TemplateConfig {
  disableAll?: boolean
  disable?: TemplateType
  entity?: string
  entityKt?: string
  service?: string
  serviceImpl?: string
  mapper?: string
  mapperXml?: string
  controller?: string
}

export interface InjectionConfig {}

export interface StrategyConfig {
  enableCapitalMode: boolean
  enableSkipView: boolean
  disableSqlFilter: boolean
  enableSchema: boolean
  addIncludes: string[]
  entity: Entity
  controller: Controller
  mapper: Mapper
  service: Service
}

export type NamingStrategy = 'underline_to_camel'
export type IdType = 'AUTO' | 'NONE' | 'INPUT' | 'ASSIGN_ID' | 'ASSIGN_UUID'
export type FillType = 'DEFAULT' | 'INSERT' | 'UPDATE' | 'INSERT_UPDATE'

export interface FillKeyVal {
  name: string
  value: FillType
}

export interface Entity {
  superClass?: string
  disableSerialVersionUID: boolean
  enableColumnConstant: boolean
  enableChainModel: boolean
  enableLombok: boolean
  enableRemoveIsPrefix: boolean
  enableTableFieldAnnotation: boolean
  enableActiveRecord: boolean
  versionColumnName?: string
  versionPropertyName?: string
  logicDeleteColumnName?: string
  logicDeletePropertyName?: string
  naming: NamingStrategy
  addSuperEntityColumns?: string[]
  addIgnoreColumns?: string[]
  addTableFills?: Array<FillKeyVal>
  idType: IdType
  formatFileName?: string
}

export interface Controller {
  superClass?: string
  enableHyphenStyle: boolean
  enableRestStyle: boolean
  formatFileName?: string
}
export interface Service {
  superServiceClass?: string
  superServiceImplClass?: string
  formatServiceFileName?: string
  formatServiceImplFileName?: string
}

export interface Mapper {
  superClass?: string
  enableMapperAnnotation: boolean
  formatMapperFileName: string
  formatXmlFileName: string
}
