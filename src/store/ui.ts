import { makeAutoObservable } from 'mobx'
import { makePersistable } from 'mobx-persist-store'

export class UiState {
  asideWidth = 240
  collapse = false
  theme = 'system'

  constructor() {
    makeAutoObservable(this, {}, {autoBind: true})
    makePersistable(this, {
      name: 'UiStore',
      properties: ['asideWidth', 'theme'],
    })
  }

  toggleCollapse() {
    this.collapse = !this.collapse
  }

  changeTheme(theme: string) {
    this.theme = theme
  }

  get currentTheme() {
    let theme = this.theme
    if(theme === 'system') {
      const darkThemeMq = window.matchMedia('(prefers-color-scheme: dark)')
      if(darkThemeMq.matches) {
        theme = 'dark'
      }else{
        theme = 'light'
      }
    }
    return theme
  }

  get headerWidth() {
    return this.collapse ? 48 : this.asideWidth
  }
}
