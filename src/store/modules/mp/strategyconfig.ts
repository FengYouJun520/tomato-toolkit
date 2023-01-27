import { Controller, Entity, IdType, Mapper, NamingStrategy, Service, StrategyConfig } from '@/types/type'

const defaultEntity: Entity = {
  superClass: '',
  superEntityColumns: [],
  ignoreColumns: [],
  disableSerialVersionUid: true,
  columnContant: false,
  chainMode: false,
  lombok: false,
  booleanColumnRemoveIsPrefix: false,
  enableTableFieldAnnotation: false,
  versionColumnName: '',
  versionPropertyName: '',
  logicDeleteColumnName: '',
  logicDeletePropertyName: '',
  tableFillList: [],
  naming: 'underlineToCamel',
  activeRecord: false,
  idType: 'auto',
  fileOverride: true,
  formatFilename: '',
}

const defaultController: Controller = {
  superClass: '',
  restStyle: false,
  hyphenStyle: false,
  fileOverride: true,
  formatFilename: '',
}

const defaultMapper: Mapper = {
  superClass: '',
  mapperAnnotation: true,
  mapperAnnotationClass: '',
  baseResultMap: false,
  baseColumnList: false,
  fileOverride: true,
  formatMapperFilename: '',
  formatXmlFilename: '',
}

const defaultService: Service = {
  superServiceClass: '',
  superServiceImplClass: '',
  fileOverride: true,
  formatServiceFilename: '',
  formatServiceImplFilename: '',
}

export const useStrategyConfigStore = defineStore('strategyConfigState', {
  state: (): StrategyConfig => ({
    isCapitalMode: false,
    skipView: false,
    tablePrefix: [],
    tablesuffix: [],
    fieldPrefix: [],
    fieldsuffix: [],
    include: [],
    exclude: [],
    enableSqlFilter: true,
    enableSchema: false,
    entity: { ...defaultEntity },
    controller: { ...defaultController },
    mapper: { ...defaultMapper },
    service: { ...defaultService },
  }),
  actions: {
    resetEntity() {
      this.entity = { ...defaultEntity }
    },
    resetController() {
      this.controller = { ...defaultController }
    },
    resetService() {
      this.service = { ...defaultService }
    },
    resetMapper() {
      this.mapper = { ...defaultMapper }
    },
  },
  getters: {
    getEntity: state => state.entity,
    getController: state => state.controller,
    getService: state => state.service,
    getMapper: state => state.mapper,
  },
  persistedState: {
    excludePaths: ['include', 'exclude'],
  },
})
