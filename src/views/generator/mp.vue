<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/modules/mp/strategyconfig'
import DatasourceConfig from './DatasourceConfig.vue'
import GenerateConfig from './GenerateConfig.vue'
import mybatisIcon from '@/assets/mybatis.svg'
import { createTableContext } from './useTables'
import { BasicTableInfo } from '@/types/type'

const strategyStore = useStrategyConfigStore()
const disabled = computed(() => strategyStore.include.size <= 0)
watchEffect(() => {
  console.log(strategyStore.include.size)
})

const tables = ref<BasicTableInfo[]>([{
  name: 'sys_user',
  comment: '用户表',
}])

createTableContext(tables)
</script>

<template>
  <n-card>
    <template #header>
      <div class="flex items-center space-x-2">
        <n-image
          :src="mybatisIcon"
          :width="64"
          :height="64"
          preview-disabled
        />
        <h1>MyBatisPlus代码生成器</h1>
      </div>
    </template>
    <DatasourceConfig />
    <GenerateConfig class="mt-5" />

    <template #footer>
      <n-button block type="primary" :disabled="disabled">
        生成
      </n-button>
    </template>
  </n-card>
</template>

<style lang="css" scoped>
</style>
