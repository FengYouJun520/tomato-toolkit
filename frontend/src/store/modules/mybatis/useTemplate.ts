import { TemplateConfig } from '@/types/mybatis-plus'
import { defineStore } from 'pinia'

interface TemplateConfigState {
  template: TemplateConfig
}

export const useTemplate = defineStore('template-config', {
  state: (): TemplateConfigState => {
    return {
      template: {},
    }
  },
  actions: {
    clearTemplate() {
      this.$reset()
      localStorage.removeItem('template-config')
    },
  },
  getters: {
    getTemplate: (state) => {
      return state.template
    },
  },
  persistedState: {
    persist: true,
  },
})
