import {InjectionConfig} from '@/types/mybatis-plus'
import {defineStore} from 'pinia'

interface InjectConfigState {
  injection: InjectionConfig
}

export const useInject = defineStore('injection-config', {
  state: (): InjectConfigState => {
    return {
      injection: {},
    }
  },
  actions: {
    clearInject() {
      this.$reset()
      localStorage.removeItem('injection-config')
    },
  },
  getters: {
    getInjection: (state) => {
      return state.injection
    },
  },
  persistedState: {
    persist: true,
  },
})
