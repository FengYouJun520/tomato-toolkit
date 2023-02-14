import { Controller, Entity, Mapper, Service, StrategyConfig } from '@/types/type'
import { isHydrated, makePersistable } from 'mobx-persist-store'

export const defaultEntity: Entity = {
  superClass: '',
  superEntityColumns: [],
  ignoreColumns: [],
  serialVersionUid: false,
  columnConstant: false,
  chainMode: false,
  lombok: false,
  booleanColumnRemoveIsPrefix: false,
  enableTableFieldAnnotation: false,
  versionColumnName: '',
  versionPropertyName: '',
  logicDeleteColumnName: '',
  logicDeletePropertyName: '',
  tableFillList: [],
  naming: 'UnderlineToCamel',
  activeRecord: false,
  idType: 'AUTO',
  formatFilename: '{}',
  fileOverride: false,
}

export const defaultController: Controller = {
  superClass: '',
  restStyle: false,
  hyphenStyle: false,
  formatFilename: '{}Controller',
  fileOverride: false,
}

export const defaultMapper: Mapper = {
  superClass: '',
  mapperAnnotation: false,
  baseResultMap: false,
  baseColumnList: false,
  formatMapperFilename: '{}Mapper',
  formatXmlFilename: '{}Mapper',
  fileOverride: false,
}

export const defaultService: Service = {
  superServiceClass: '',
  superServiceImplClass: '',
  formatServiceFilename: '{}IService',
  formatServiceImplFilename: '{}ServiceImpl',
  fileOverride: false,
}

export interface StrategyBaseConfig {
  isCapitalMode: false,
  skipView: false,
  tablePrefix: [],
  tableSuffix: [],
  fieldPrefix: [],
  fieldSuffix: [],
  enableSqlFilter: true,
  enableSchema: false,
}

export const initialStrategyBaseConfig: StrategyBaseConfig = {
  isCapitalMode: false,
  skipView: false,
  tablePrefix: [],
  tableSuffix: [],
  fieldPrefix: [],
  fieldSuffix: [],
  enableSqlFilter: true,
  enableSchema: false,
}

export class StrategyConfigStore {
  strategyBase: StrategyBaseConfig = initialStrategyBaseConfig
  include: string[] = []
  exclude: string[] = []
  entity: Entity = defaultEntity
  controller: Controller = defaultController
  service: Service = defaultService
  mapper: Mapper = defaultMapper

  constructor() {
    makeAutoObservable(this, {}, {autoBind: true})
    makePersistable(this, {
      name: 'StrategyConfigStore',
      properties: ['strategyBase', 'entity', 'controller', 'service', 'mapper'],
    })
  }

  resetSelectTable() {
    this.include = []
    this.exclude = []
  }

  get isHydrated() {
    return isHydrated(this)
  }

  setStrategyBaseConfig(strategy: Partial<StrategyBaseConfig>) {
    this.strategyBase = {
      ...this.strategyBase,
      ...strategy,
    }
  }

  setEntityConfig(entity: Partial<Entity>) {
    this.entity = {
      ...this.entity,
      ...entity,
    }
  }

  setControllerConfig(controller: Partial<Controller>) {
    this.controller = {
      ...this.controller,
      ...controller,
    }
  }

  setServiceConfig(service: Partial<Service>) {
    this.service = {
      ...this.service,
      ...service,
    }
  }

  setMapperConfig(mapper: Partial<Mapper>) {
    this.mapper = {
      ...this.mapper,
      ...mapper,
    }
  }

  setTables(table: Partial<{include: string[], exclude: string[]}>) {
    this.include = [...table.include || []]
    this.exclude = [...table.exclude || []]
  }

  get strategyConfig(): StrategyConfig {
    return {
      ...this.strategyBase,
      include: this.include,
      exclude: this.exclude,
      entity: this.entity,
      controller: this.controller,
      service: this.service,
      mapper: this.mapper,
    }
  }
}
