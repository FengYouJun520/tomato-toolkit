

interface UiState {
  collapse: boolean
}

export const useUiState = defineStore('UI_STATE', {
  state: (): UiState => ({
    collapse: false,
  }),
  actions: {
    toggleCollapse() {
      this.collapse = !this.collapse
    },
  },
  getters: {
    isCollapse: state => state.collapse,
  },
})
