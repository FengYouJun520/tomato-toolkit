import { GlobalConfig } from '@/types/mybatis-plus'
import { defineStore } from 'pinia'

interface GlobalConfigState {
  global: GlobalConfig
}

export const useGlobal = defineStore('global-config', {
  state: (): GlobalConfigState => {
    return {
      global: {
        fileOverride: false,
        disableOpenDir: true,
        outputDir: 'D:/',
        author: 'FengYouJun',
        enableKotlin: false,
        enableSwagger: false,
        dateType: 'TIME_PACK',
        commentDate: 'yyyy-MM-dd',
      },
    }
  },
  actions: {
    clearGlobal() {
      this.$reset()
      localStorage.removeItem('global-config')
    },
  },
  getters: {
    getGlobal: (state) => {
      return state.global
    },
  },
  persistedState: {
    persist: true,
  },
})
