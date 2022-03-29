<template>
  <to-page>
    <template #title>
      <div class="flex items-center gap-3">
        <img :src="MybatisIcon" alt="mybatis-plus" class="h-12" />
        <div>Mybatis-Plus代码生成器</div>
      </div>
    </template>
    <div class="flex flex-col gap-y-3">
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
      <div class="flex justify-center">
        <t-button block>生成</t-button>
      </div>
    </template>
  </to-page>
</template>
<script setup lang="ts">
import MybatisIcon from '@/assets/mybatis.png'
import ToPage from '@/components/ToPage/index.vue'
import BasicConfig from './components/basic-config/index.vue'
import GlobalConfig from './components/global-config/index.vue'
import InjectConfig from './components/inject-config/index.vue'
import PackageConfig from './components/package-config/index.vue'
import StrategyConfig from './components/strategy-config/index.vue'
import TemplateConfig from './components/template-config/index.vue'

const tabsValue = ref('global-config')
</script>

<style scoped></style>
