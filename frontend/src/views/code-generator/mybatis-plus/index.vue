<template>
  <to-page>
    <template #title>
      <div flex items-center gap-3>
        <img :src="MybatisIcon" alt="mybatis-plus" h-12 />
        <div>Mybatis-Plus代码生成器</div>
      </div>
    </template>
    <div flex flex-col gap-y-3>
      <!-- 基础配置 -->
      <basic-config />

      <t-tabs v-model="tabsValue" theme="card">
        <t-tab-panel label="全局配置" value="global-config">
          <global-config />
        </t-tab-panel>
        <t-tab-panel label="包配置" value="package-config">
          <package-config />
        </t-tab-panel>
        <t-tab-panel label="模板配置" value="template-config">
          <template-config />
        </t-tab-panel>
        <t-tab-panel disabled label="注入配置" value="inject-config">
          <inject-config />
        </t-tab-panel>
        <t-tab-panel label="策略配置" value="strategy-config">
          <strategy-config />
        </t-tab-panel>
      </t-tabs>
    </div>

    <template #footer>
      <div flex justify-center>
        <t-button
          block
          @click="executeGenerate"
          :loading="loading"
          :disabled="basicStore.executeDisable"
        >
          生成
        </t-button>
      </div>
    </template>
  </to-page>
</template>
<script setup lang="ts">
import MybatisIcon from '@/assets/mybatis.png'
import ToPage from '@/components/ToPage/index.vue'
import { useBasic } from '@/store/modules/mybatis/useBasic'
import { useGlobal } from '@/store/modules/mybatis/useGlobal'
import { usePackage } from '@/store/modules/mybatis/usePackage'
import { useStrategy } from '@/store/modules/mybatis/useStrategy'
import { useTemplate } from '@/store/modules/mybatis/useTemplate'
import { CodeGenerate } from '@/wailsjs/go/codegen/Manager'
import { codegen } from '@/wailsjs/go/models'
import { MessagePlugin } from 'tdesign-vue-next'
import BasicConfig from './components/basic-config/index.vue'
import GlobalConfig from './components/global-config/index.vue'
import InjectConfig from './components/inject-config/index.vue'
import PackageConfig from './components/package-config/index.vue'
import StrategyConfig from './components/strategy-config/index.vue'
import TemplateConfig from './components/template-config/index.vue'

const loading = ref(false)
const basicStore = useBasic()
const globalStore = useGlobal()
const packageStore = usePackage()
const templateStore = useTemplate()
const strategyStore = useStrategy()

const tabsValue = ref('global-config')

const executeGenerate = () => {
  if (!strategyStore.strategy.addIncludes || !strategyStore.strategy.addIncludes?.length) {
    MessagePlugin.warning('请在策略配置中选择要生成的表！')
    return
  }

  const configContext = codegen.ConfigContext.createFrom({
    dataSource: codegen.DataSourceConfig.createFrom(toRaw(basicStore.dataSource)),
    globalConfig: codegen.GlobalConfig.createFrom(toRaw(globalStore.global)),
    packageConfig: codegen.PackageConfig.createFrom(toRaw(packageStore.package)),
    templateConfig: codegen.TemplateConfig.createFrom(toRaw(templateStore.template)),
    strategyConfig: codegen.StrategyConfig.createFrom(toRaw(strategyStore.strategy)),
  })

  loading.value = true

  CodeGenerate(configContext)
    .then((msg: any) => {
      MessagePlugin.success(msg)
    })
    .catch((err: any) => {
      MessagePlugin.error(err)
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<style scoped></style>
