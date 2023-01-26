<script setup lang="ts">
import DatasourceConfig from './DatasourceConfig.vue'
import GenerateConfig from './GenerateConfig.vue'
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
      <GenerateConfig />
    </n-form>
  </n-card>
</template>

<style lang="css" scoped>
</style>
