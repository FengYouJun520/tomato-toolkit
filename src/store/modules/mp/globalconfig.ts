import { DataType, GlobalConfig } from '@/types/type'

export const useGlobalConfig = defineStore('globalConfigState', {
  state: (): GlobalConfig => ({
    outputDir: 'D://',
    open: false,
    author: 'baomidou',
    kotlin: false,
    swagger: false,
    springdoc: false,
    dateType: DataType.TIME_PACK,
    commentDate: '%Y-%M-%S',
  }),
  persistedState: {
  },
})
