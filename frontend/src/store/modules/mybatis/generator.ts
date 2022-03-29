import { defineStore } from 'pinia'

interface MybatisPlusState {}

export const useMybatisPlus = defineStore('mybatis-plus', {
  state: (): MybatisPlusState => {
    return {}
  },
})
