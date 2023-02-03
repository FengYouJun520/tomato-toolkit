import { InjectConfig } from '@/types/type'

export const useInjectConfigStore = defineStore('injectConfigState', {
  state: (): InjectConfig => ({
    customMap: {},
    customFiles: [],
  }),
  persistedState: {},
})
