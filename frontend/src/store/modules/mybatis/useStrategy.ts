import { StrategyConfig } from '@/types/mybatis-plus'
import { FillInput } from '@/views/mybatis-plus/generator/components/strategy-config/strategy-right/data'
import { nanoid } from 'nanoid'
import { defineStore } from 'pinia'
import { useBasic } from './useBasic'

interface StrategyConfigState {
  strategy: StrategyConfig
}

export const useStrategy = defineStore('strategy-config', {
  state: (): StrategyConfigState => {
    return {
      strategy: {
        enableCapitalMode: false,
        enableSkipView: false,
        disableSqlFilter: true,
        addIncludes: [],
        enableSchema: false,
        entity: {
          disableSerialVersionUID: true,
          enableColumnConstant: false,
          enableChainModel: false,
          enableLombok: false,
          enableRemoveIsPrefix: false,
          enableTableFieldAnnotation: false,
          enableActiveRecord: false,
          addIgnoreColumns: [],
          addSuperEntityColumns: [],
          addTableFills: [],
          naming: 'underline_to_camel',
          idType: 'ASSIGN_ID',
          formatFileName: '%s',
        },
        controller: {
          enableRestStyle: false,
          enableHyphenStyle: false,
          formatFileName: '%sController',
        },
        service: {
          formatServiceFileName: '%sService',
          formatServiceImplFileName: '%sServiceImpl',
        },
        mapper: {
          enableMapperAnnotation: false,
          formatXmlFileName: '%sMapper',
          formatMapperFileName: '%sMapper',
        },
      },
    }
  },
  actions: {
    clearStrategy() {
      this.$reset()
      localStorage.removeItem('strategy-config')
    },
    setAddIncludes() {
      const basicStore = useBasic()
      this.strategy.addIncludes = []
      basicStore.tablesOptions.forEach((option) => {
        this.strategy.addIncludes.push(option.value)
      })
    },
    clearAddIncludes() {
      this.strategy.addIncludes = []
    },
    // 添加要自动填充的字段
    addFillField() {
      this.$patch((state) => {
        state.strategy.entity.addTableFills.push({
          key: nanoid(),
          name: '',
          value: 'DEFAULT',
        })
        console.log(state.strategy.entity.addTableFills)
      })
    },
    // 删除要自动填充的字段
    removeFillField(fillInput: FillInput) {
      if (!this.strategy.entity.addTableFills.length) {
        return
      }

      const index = this.strategy.entity.addTableFills.findIndex(
        (fill) => fill.key === fillInput.key
      )

      if (index !== -1) {
        this.$patch((state) => {
          state.strategy.entity.addTableFills.splice(index, 1)
        })
      }
    },
  },
  getters: {
    getStrategy: (state) => {
      return state.strategy
    },
    addTableFillsIsExist: (state) => {
      return state.strategy.entity.addTableFills && state.strategy.entity.addTableFills.length > 0
    },
  },
  persistedState: {
    persist: true,
    excludePaths: ['strategy.addIncludes'],
  },
})
