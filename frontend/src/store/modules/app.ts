import {defineStore} from 'pinia'

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
    toggleCollapsed() {
      this.collapse = !this.collapse
    },
  },
})
