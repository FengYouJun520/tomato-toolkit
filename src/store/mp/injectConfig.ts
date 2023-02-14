import { CustomFile, InjectConfig } from '@/types/type'
import { isHydrated, makePersistable } from 'mobx-persist-store'

export const initialInjectConfig: InjectConfig = {
  customMap: {},
  customFiles: [],
}

export class InjectConfigStore {

  injection: InjectConfig = initialInjectConfig

  constructor() {
    makeAutoObservable(this, {}, {autoBind: true})
    makePersistable(this, {
      name: 'InjectConfigStore',
      properties: ['injection'],
    })
  }

  setCustomMap(custom: Record<string, any>) {
    this.injection.customMap = custom
  }

  addCustomFile(custom: CustomFile) {
    this.injection.customFiles = [...this.customFiles, custom]
  }

  editCustomFile(customFile: CustomFile) {
    const index = this.customFiles.findIndex(c => c.id === customFile.id)

    if(index  === -1) {
      return
    }

    const customFiles = this.injection.customFiles
    customFiles.splice(index, 1, customFile)
    this.injection.customFiles = [...customFiles]
  }

  deleteCustomFile(id: string) {
    this.injection.customFiles = this.injection.customFiles.filter(c => c.id !== id)
  }

  get isHydrate() {
    return isHydrated(this)
  }

  get customMap() {
    return this.injection.customMap
  }

  get customFiles() {
    return this.injection.customFiles
  }
}
