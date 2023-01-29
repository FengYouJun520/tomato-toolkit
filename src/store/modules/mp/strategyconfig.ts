import { Controller, Entity, Mapper, Service, StrategyConfig } from '@/types/type'

const defaultEntity: Entity = {
  superClass: '',
  superEntityColumns: [],
  ignoreColumns: [],
  disableSerialVersionUid: true,
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
}

const defaultController: Controller = {
  superClass: '',
  restStyle: false,
  hyphenStyle: false,
  formatFilename: '{}Controller',
}

const defaultMapper: Mapper = {
  superClass: '',
  mapperAnnotation: false,
  mapperAnnotationClass: '',
  baseResultMap: false,
  baseColumnList: false,
  formatMapperFilename: '{}Mapper',
  formatXmlFilename: '{}Mapper',
}

const defaultService: Service = {
  superServiceClass: '',
  superServiceImplClass: '',
  formatServiceFilename: '{}IService',
  formatServiceImplFilename: '{}ServiceImpl',
}

export const useStrategyConfigStore = defineStore('strategyConfigState', {
  state: (): StrategyConfig => ({
    isCapitalMode: false,
    skipView: false,
    tablePrefix: [],
    tableSuffix: [],
    fieldPrefix: [],
    fieldSuffix: [],
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
