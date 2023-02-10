

interface UiState {
  collapse: boolean
  asideWidth: number
}

export const useUiState = defineStore('UI_STATE', {
  state: (): UiState => ({
    collapse: false,
    asideWidth: 240,
  }),
  actions: {
    toggleCollapse() {
      this.collapse = !this.collapse
    },
    setAsideWidth(width: number) {
      this.asideWidth = width
    },
  },
  getters: {
    isCollapse: state => state.collapse,
  },
})
