import { DatasourceConfig } from '@/types/type'
import { makeAutoObservable } from 'mobx'
import { clearPersistedStore, isHydrated, makePersistable } from 'mobx-persist-store'

export const initDataSourceConfig: DatasourceConfig = {
  type: 'mysql',
  host: 'localhost',
  port: 3306,
  username: '',
  password: '',
  database: 'blog',
}

export class DataSourceConfigStore {
  dataSource: DatasourceConfig = initDataSourceConfig

  constructor() {
    makeAutoObservable(this, {}, {autoBind: true})
    makePersistable(this, {
      name: 'DataSourceConfigStore',
      properties: ['dataSource'],
    })
  }

  setDataSourceConfig(datasource: Partial<DatasourceConfig>) {
    this.dataSource = {
      ...this.dataSource,
      ...datasource,
    }
  }
  get isHydrated() {
    return isHydrated(this)
  }

  async clearStoredDate() {
    await clearPersistedStore(this)
  }
}
