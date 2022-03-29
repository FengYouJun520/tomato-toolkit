import { defineStore } from 'pinia'

export type ThemeType = 'light' | 'dark' | 'auto'

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
    toggleTheme(theme: ThemeType) {
      this.theme = theme
    },
  },
  getters: {
    getTheme: (state): 'light' | 'dark' => {
      let theme = state.theme
      if (state.theme === 'auto') {
        const mediaQueryListDark = window.matchMedia('(prefers-color-scheme: dark)')
        if (mediaQueryListDark.matches) {
          // 系统当前是暗色(dark)主题
          theme = 'dark'
        } else {
          theme = 'light'
        }
      }
      return theme as 'light' | 'dark'
    },
  },

  persistedState: {
    persist: true,
  },
})
