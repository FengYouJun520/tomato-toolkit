import { GlobalConfig } from '@/types/type'
import { makeAutoObservable } from 'mobx'
import { isHydrated, makePersistable } from 'mobx-persist-store'

export const initialGlobalConfig: GlobalConfig = {
  outputDir: 'D:\\',
  author: 'baomidou',
  open: false,
  springdoc: false,
  swagger: false,
  kotlin: false,
  dateType: 'TIME_PACK',
  commentDate: '%Y-%m-%d',
}

export class GlobalConfigStore {
  global: GlobalConfig = initialGlobalConfig

  constructor() {
    makeAutoObservable(this, {}, {autoBind: true})
    makePersistable(this, {
      name: 'GlobalConfigStore',
      properties: ['global'],
    })
  }

  setGlobalConfig(global: Partial<GlobalConfig>) {
    this.global = {
      ...this.global,
      ...global,
    }
  }

  get isHydrated() {
    return isHydrated(this)
  }
}
