import { DatabaseOptions, DataSourceConfig, TablesOptions } from '@/types/mybatis-plus'
import { defineStore } from 'pinia'

interface BasicState {
  dataSource: DataSourceConfig
  executeDisable: boolean
  tablesOptions: TablesOptions[]
}

export const useBasic = defineStore('basic-config', {
  state: (): BasicState => {
    return {
      dataSource: {
        typ: 'mysql',
        database: 'blog',
        username: 'root',
        password: 'root',
        host: 'localhost',
        port: 3306,
      },
      executeDisable: true,
      tablesOptions: [],
    }
  },
  actions: {
    clearBasic() {
      this.$reset()
      localStorage.removeItem('basic-config')
    },
    setOptions(options: DatabaseOptions[]) {
      this.tablesOptions = []
      options.forEach((option) => {
        this.tablesOptions.push({
          label: option.name,
          value: option.name,
          comment: option.comment,
        })
      })
    },
    clearOptions() {
      this.tablesOptions = []
    },
  },
  getters: {
    getDataSource: (state) => {
      return state.dataSource
    },
    tableIsEmpty: (state) => {
      return state.tablesOptions.length <= 0
    },
  },
  persistedState: {
    persist: true,
    includePaths: ['dataSource'],
  },
})
