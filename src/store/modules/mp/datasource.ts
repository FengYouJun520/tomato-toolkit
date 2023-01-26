import { DatasourceConfig } from '@/types/type'

export const useDatesource = defineStore('datasourceConfigState', {
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
