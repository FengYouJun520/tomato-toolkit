import { BasicTableInfo, MpConfig } from '@/types/type'
import { DataSourceConfigStore } from './datasource'
import { GlobalConfigStore } from './globalConfig'
import { InjectConfigStore } from './injectConfig'
import { PackageConfigStore } from './packageConfig'
import { StrategyConfigStore } from './strategyConfig'
import { TemplateConfigStore } from './templateConfig'

export class MyBaTisPlusStore {
  dataSourceStore: DataSourceConfigStore
  globalStore: GlobalConfigStore
  packageStore: PackageConfigStore
  templateStore: TemplateConfigStore
  strategyStore: StrategyConfigStore
  injectStore: InjectConfigStore

  basicTableInfos: BasicTableInfo[] = []

  constructor() {
    makeAutoObservable(this, {}, {autoBind: true})
    this.globalStore = new GlobalConfigStore()
    this.dataSourceStore = new DataSourceConfigStore()
    this.packageStore = new PackageConfigStore()
    this.templateStore = new TemplateConfigStore()
    this.strategyStore = new StrategyConfigStore()
    this.injectStore= new InjectConfigStore()
  }

  setBasicTableInfos(basicTableInfos: BasicTableInfo[]) {
    this.basicTableInfos = basicTableInfos
  }

  get mpConfig(): MpConfig {
    return {
      datasource: this.dataSourceStore.dataSource,
      global: this.globalStore.global,
      package: this.packageStore.package,
      strategy: this.strategyStore.strategyConfig,
      template: this.templateStore.template,
      injection: this.injectStore.injection,
    }
  }

  get isInclude() {
    return this.strategyStore.include && this.strategyStore.include.length > 0
  }

  get outputDir() {
    return this.globalStore.global.outputDir
  }
}
