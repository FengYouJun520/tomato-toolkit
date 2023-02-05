import { CustomFile, InjectConfig } from '@/types/type'
import { logDark } from 'naive-ui'

export const useInjectConfigStore = defineStore('injectConfigState', {
  state: (): InjectConfig => ({
    customMap: {},
    customFiles: [],
  }),
  actions: {
    addCustomData(custom: Record<string, unknown>) {
      console.log(custom)

      this.customMap = {
        ...this.customMap,
        ...custom,
      }
      console.log(this.customMap)
    },
    addCustomFile(custom: CustomFile) {
      this.customFiles.push({ ...custom })
    },
    editCustomFile(custom: CustomFile) {
      const index = this.customFiles.findIndex(c => c.id === custom.id)
      if (index === -1) {
        return
      }
      this.customFiles.splice(index, 1, { ...custom })
    },
    removeCustomFile(id: string){
      const index = this.customFiles.findIndex(c => c.id === id)
      if (index !== -1) {
        this.customFiles.splice(index, 1)
      }
    },
  },
  getters: {
    getCustomFiles: state => state.customFiles,
  },
  persist: true,
})
