import { DataType, GlobalConfig } from '@/types/type'

export const useGlobalConfigStore = defineStore('globalConfigState', {
  state: (): GlobalConfig => ({
    outputDir: 'D://',
    open: false,
    author: 'baomidou',
    kotlin: false,
    swagger: false,
    springdoc: false,
    dateType: 'TIME_PACK',
    commentDate: '%Y-%m-%d',
  }),
  persistedState: {
  },
})
