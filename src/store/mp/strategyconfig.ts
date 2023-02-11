import { Controller, Entity, Mapper, Service, StrategyConfig } from '@/types/type'

const defaultEntity: Entity = {
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

const defaultController: Controller = {
  superClass: '',
  restStyle: false,
  hyphenStyle: false,
  formatFilename: '{}Controller',
  fileOverride: false,
}

const defaultMapper: Mapper = {
  superClass: '',
  mapperAnnotation: false,
  baseResultMap: false,
  baseColumnList: false,
  formatMapperFilename: '{}Mapper',
  formatXmlFilename: '{}Mapper',
  fileOverride: false,
}

const defaultService: Service = {
  superServiceClass: '',
  superServiceImplClass: '',
  formatServiceFilename: '{}IService',
  formatServiceImplFilename: '{}ServiceImpl',
  fileOverride: false,
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
    appendInitTableFill() {
      this.entity.tableFillList.push({
        propertyName: '',
        fieldFill: 'DEFAULT',
      })
    },
    removeFillTable(index: number) {
      if (index < 0 || index >= this.entity.tableFillList.length) {
        return
      }

      this.entity.tableFillList.splice(index, 1)
    },
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
  persist: {
    paths: [
      'isCapitalMode',
      'skipView',
      'tablePrefix',
      'tableSuffix',
      'fieldPrefix',
      'fieldSuffix',
      'enableSqlFilter',
      'enableSchema',
      'entity',
      'controller',
      'mapper',
      'service',
    ],
  },
})
