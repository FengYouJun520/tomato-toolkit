<script setup lang="ts">
import { useTemplateConfigStore } from '@/store/modules/mp/templateconfig'
import { shell, path } from '@tauri-apps/api'

const message = useMessage()
const templateConfigStore = useTemplateConfigStore()

const handleReset = () => {
  templateConfigStore.$reset()
}

const handleOpenTempelate = async () => {
  try {
    const resourcePath = await path.resolveResource('templates')
    shell.open(resourcePath)
  } catch (error) {
    message.error(error as string)
  }
}
</script>

<template>
  <n-form
    label-placement="left"
    :label-width="120"
    label-align="right"
  >
    <n-form-item>
      <n-button type="warning" @click="handleReset">
        重置
      </n-button>
    </n-form-item>

    <n-grid cols="1 m:2" :x-gap="24" responsive="screen">
      <n-form-item-gi label="Entity模板">
        <n-input
          v-model:value="templateConfigStore.entity"
          placeholder="默认为: templates/entity.java"
        />
      </n-form-item-gi>
      <n-form-item-gi label="EntityKotlin模板">
        <n-input
          v-model:value="templateConfigStore.entityKt"
          placeholder="默认为: templates/entity.kt.java"
        />
      </n-form-item-gi>

      <n-form-item-gi label="Mapper模板">
        <n-input
          v-model:value="templateConfigStore.mapper"
          placeholder="默认为: templates/mapper.java" />
      </n-form-item-gi>
      <n-form-item-gi label="Xml模板">
        <n-input
          v-model:value="templateConfigStore.xml"
          placeholder="默认为: templates/mapper.xml"
        />
      </n-form-item-gi>

      <n-form-item-gi label="Service模板">
        <n-input
          v-model:value="templateConfigStore.service"
          placeholder="默认为: templates/service.java"
        />
      </n-form-item-gi>
      <n-form-item-gi label="ServiceImpl模板">
        <n-input
          v-model:value="templateConfigStore.serviceImpl"
          placeholder="默认为: templates/servicerImpl.java"
        />
      </n-form-item-gi>

      <n-form-item-gi label="Controller模板">
        <n-input
          v-model:value="templateConfigStore.controller"
          placeholder="默认为: templates/controller.java"
        />
      </n-form-item-gi>
    </n-grid>
  </n-form>
</template>

<style lang="css" scoped>
</style>
