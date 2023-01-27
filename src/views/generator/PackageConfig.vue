<script setup lang="ts">
import { usePackageConfigStore } from '@/store/modules/mp/packageconfig'

const packageConfigStore = usePackageConfigStore()

const handleReset = () => {
  packageConfigStore.$reset()
}
</script>

<template>
  <n-form label-placement="left" :label-width="120" label-align="right">
    <n-form-item>
      <n-button type="warning" @click="handleReset">
        重置
      </n-button>
    </n-form-item>

    <n-grid :cols="2" :x-gap="24">
      <n-form-item-gi label="父包名">
        <n-input v-model:value="packageConfigStore.parent" />
      </n-form-item-gi>
      <n-form-item-gi label="父模块名">
        <n-input v-model:value="packageConfigStore.moduleName" />
      </n-form-item-gi>

      <n-form-item-gi label="Entity包名">
        <n-input v-model:value="packageConfigStore.entity" />
      </n-form-item-gi>
      <n-form-item-gi label="Controller名">
        <n-input v-model:value="packageConfigStore.controller" />
      </n-form-item-gi>

      <n-form-item-gi label="Service包名">
        <n-input v-model:value="packageConfigStore.service" />
      </n-form-item-gi>
      <n-form-item-gi label="ServiceImpl名">
        <n-input v-model:value="packageConfigStore.serviceImpl" />
      </n-form-item-gi>

      <n-form-item-gi label="Mapper包名">
        <n-input v-model:value="packageConfigStore.mapper" />
      </n-form-item-gi>
      <n-form-item-gi label="MapperXml包名">
        <n-input v-model:value="packageConfigStore.xml" />
      </n-form-item-gi>
    </n-grid>
  </n-form>
</template>

<style lang="css" scoped>
</style>
