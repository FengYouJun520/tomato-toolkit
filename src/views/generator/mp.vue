<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/modules/mp/strategyconfig'
import DatasourceConfig from './DatasourceConfig.vue'
import GenerateConfig from './GenerateConfig.vue'
import mybatisIcon from '@/assets/mybatis.svg'
import { createTableContext } from './useTables'
import { BasicTableInfo, MpConfig } from '@/types/type'
import { invoke } from '@tauri-apps/api'
import { useDatasourceStore } from '@/store/modules/mp/datasource'
import { useGlobalConfigStore } from '@/store/modules/mp/globalconfig'
import { usePackageConfigStore } from '@/store/modules/mp/packageconfig'
import { useTemplateConfigStore } from '@/store/modules/mp/templateconfig'

const message = useMessage()
const datasourceStore = useDatasourceStore()
const globalStore = useGlobalConfigStore()
const packageStore = usePackageConfigStore()
const templateStore = useTemplateConfigStore()
const strategyStore = useStrategyConfigStore()
const disabled = computed(() => strategyStore.include.length <= 0)

const tables = ref<BasicTableInfo[]>([])

createTableContext(tables)

const handleMpGenerate = async () => {
  try {
    const config: MpConfig = {
      datasource: datasourceStore.$state,
      global: globalStore.$state,
      package: packageStore.$state,
      template: templateStore.$state,
      strategy: strategyStore.$state,
    }
    await invoke('mp_codegen', { config })
  } catch (error) {
    message.error(error as string)
  }
}
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
      <n-button block type="primary" :disabled="disabled" @click="handleMpGenerate">
        生成
      </n-button>
    </template>
  </n-card>
</template>

<style lang="css" scoped>
</style>
