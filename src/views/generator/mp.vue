<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/modules/mp/strategyconfig'
import DatasourceConfig from './DatasourceConfig.vue'
import GenerateConfig from './GenerateConfig.vue'

const strategyStore = useStrategyConfigStore()
const disabled = computed(() => !strategyStore.include || strategyStore.include.size <= 0)
</script>

<template>
  <n-card>
    <template #header>
      <div>
        <h1>MyBatisPlus代码生成器</h1>
      </div>
    </template>
    <n-form label-placement="left" :label-width="100" label-align="left">
      <DatasourceConfig />
      <GenerateConfig class="mt-5" />

      <n-form-item class="mt-5">
        <n-button block type="primary" :disabled="disabled">
          生成
        </n-button>
      </n-form-item>
    </n-form>
  </n-card>
</template>

<style lang="css" scoped>
</style>
