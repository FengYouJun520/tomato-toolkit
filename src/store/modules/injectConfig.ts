import { CustomFile, InjectConfig } from '@/types/type'

export const useInjectConfigStore = defineStore('injectConfigState', {
  state: (): InjectConfig => ({
    customMap: {},
    customFiles: [],
  }),
  actions: {
    addCustomData(key: string, value: any) {
      this.customMap[key] = value
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
    removeCustomData(key: string) {
      this.customMap[key] = undefined
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
