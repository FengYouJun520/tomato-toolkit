<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/mp/strategyconfig'
import DatasourceConfig from './DatasourceConfig.vue'
import GenerateConfig from './GenerateConfig.vue'
import mybatisIcon from '@/assets/mybatis.svg'
import { BasicTableInfo, MpConfig } from '@/types/type'
import { invoke } from '@tauri-apps/api'
import { useDatasourceStore } from '@/store/mp/datasource'
import { useGlobalConfigStore } from '@/store/mp/globalconfig'
import { usePackageConfigStore } from '@/store/mp/packageconfig'
import { useTemplateConfigStore } from '@/store/mp/templateconfig'
import { useInjectConfigStore } from '@/store/mp/injectConfig'
import PageLayout from '@/components/PageLayout/index.vue'

const message = useMessage()
const datasourceConfigStore = useDatasourceStore()
const globalConfigStore = useGlobalConfigStore()
const packageConfigStore = usePackageConfigStore()
const templateConfigStore = useTemplateConfigStore()
const strategyConfigStore = useStrategyConfigStore()
const injectConfigStore = useInjectConfigStore()
const disabled = computed(() => strategyConfigStore.include.length <= 0)

const handleMpGenerate = async () => {
  try {
    const config: MpConfig = {
      datasource: datasourceConfigStore.$state,
      global: globalConfigStore.$state,
      package: packageConfigStore.$state,
      template: templateConfigStore.$state,
      strategy: strategyConfigStore.$state,
      injection: injectConfigStore.$state,
    }
    await invoke('mp_codegen', { config })
    message.success('代码生成成功！')
  } catch (error) {
    message.error(error as string)
  }
}
</script>

<template>
  <PageLayout>
    <template #header>
      <div class="flex items-center space-x-2">
        <el-image
          :src="mybatisIcon"
          style="width: 64px; height: 64px;"
        />
        <span class="text-xl">MyBatisPlus代码生成器</span>
      </div>
    </template>
    <DatasourceConfig />
    <GenerateConfig class="mt-5" />

    <template #footer>
      <el-button
        type="primary"
        class="w-full"
        :disabled="disabled"
        @click="handleMpGenerate"
      >
        生成
      </el-button>
    </template>
  </PageLayout>
</template>

<style lang="css" scoped>
</style>
