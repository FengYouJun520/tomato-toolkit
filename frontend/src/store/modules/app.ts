import { defineStore } from 'pinia'

interface AppState {
  collapse: boolean
}

export const useAppStore = defineStore('app', {
  state: (): AppState => {
    return {
      collapse: false,
    }
  },
  actions: {
    onCollapsed(collapse: boolean) {
      this.collapse = collapse
    },
  },
})
