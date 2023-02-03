<script setup lang="ts">
import GlobalConfig from './GlobalConfig.vue'
import InjectConfig from './InjectConfig.vue'
import PackageConfig from './PackageConfig.vue'
import StrategyConfig from './strategy/StrategyConfig.vue'
import TemplateConfig from './TemplateConfig.vue'
</script>

<template>
  <n-tabs
    default-value="global-config"
    type="card"
  >
    <n-tab-pane tab="全局配置" name="global-config">
      <GlobalConfig />
    </n-tab-pane>
    <n-tab-pane tab="包配置" name="package-config">
      <PackageConfig />
    </n-tab-pane>
    <n-tab-pane tab="模板配置" name="template-config">
      <TemplateConfig />
    </n-tab-pane>
    <n-tab-pane tab="策略配置" name="strategy-config">
      <StrategyConfig />
    </n-tab-pane>
    <n-tab-pane disabled tab="注入配置（未完成）" name="inject-config">
      <InjectConfig />
    </n-tab-pane>
  </n-tabs>
</template>

<style lang="css" scoped>
</style>
