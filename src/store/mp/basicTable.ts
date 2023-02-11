import { BasicTableInfo } from '@/types/type'

interface BasicTableInfoState {
  basicTables: BasicTableInfo[]
}

export const useBasicTableInfosStore = defineStore('BasicTable', {
  state: (): BasicTableInfoState => ({
    basicTables: [],
  }),
  actions: {
    setBasicTables(tables: BasicTableInfo[]) {
      this.basicTables = tables
    },
  },
  getters: {
    isEmpty: state => state.basicTables.length === 0,
  },
})
