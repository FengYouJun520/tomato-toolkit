import { defineStore } from 'pinia'

export type ThemeType = 'light' | 'dark'

interface ThemeState {
  theme: ThemeType
}

export const useThemeStore = defineStore('theme', {
  state: (): ThemeState => {
    return {
      theme: 'light',
    }
  },
  actions: {
    toggleTheme() {
      this.theme = this.theme === 'dark' ? 'light' : 'dark'
    },
  },
  persistedState: {
    persist: true,
  },
})
