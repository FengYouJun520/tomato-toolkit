<script setup lang="ts">
import GlobalConfig from './GlobalConfig.vue'
import InjectConfig from './InjectConfig.vue'
import PackageConfig from './PackageConfig.vue'
import StrategyConfig from './strategy/StrategyConfig.vue'
import TemplateConfig from './TemplateConfig.vue'

const active = ref('global-config')
</script>

<template>
  <el-tabs v-model="active" type="border-card">
    <el-tab-pane label="全局配置" name="global-config">
      <GlobalConfig />
    </el-tab-pane>
    <el-tab-pane label="包配置" name="package-config">
      <PackageConfig />
    </el-tab-pane>
    <el-tab-pane label="模板配置" name="template-config">
      <TemplateConfig />
    </el-tab-pane>
    <el-tab-pane label="策略配置" name="strategy-config">
      <StrategyConfig />
    </el-tab-pane>
    <el-tab-pane label="注入配置" name="inject-config">
      <InjectConfig />
    </el-tab-pane>
  </el-tabs>
</template>

<style lang="css" scoped>
</style>
