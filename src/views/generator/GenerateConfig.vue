<script setup lang="ts">


</script>

<template>
  <n-form-item>
    <n-tabs
      default-value="global-config"
      type="card"
    >
      <n-tab-pane tab="全局配置" name="global-config">
        全局配置
      </n-tab-pane>
      <n-tab-pane tab="包配置" name="package-config">
        包配置
      </n-tab-pane>
      <n-tab-pane tab="模板配置" name="template-config">
        模板配置
      </n-tab-pane>
      <n-tab-pane tab="策略配置" name="strategy-config">
        策略配置
      </n-tab-pane>
    </n-tabs>
  </n-form-item>
</template>

<style lang="css" scoped>
</style>
