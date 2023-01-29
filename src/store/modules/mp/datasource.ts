import { DatasourceConfig } from '@/types/type'

export const useDatasourceStore = defineStore('datasourceConfigState', {
  state: (): DatasourceConfig => ({
    type: 'mysql',
    database: 'blog',
    host: 'localhost',
    port: 3306,
    username: '',
    password: '',
  }),
  persistedState: {
  },
})
