<script setup lang="ts">
import { useTemplateConfigStore } from '@/store/modules/mp/templateconfig'


const templateConfigStore = useTemplateConfigStore()

const handleReset = () => {
  templateConfigStore.$reset()
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
      <n-form-item-gi label="禁用所有模板">
        <n-radio-group v-model:value="templateConfigStore.disable">
          <n-radio-button :value="true">
            开启
          </n-radio-button>
          <n-radio-button :value="false">
            关闭
          </n-radio-button>
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="Controller模板">
        <n-input v-model:value="templateConfigStore.controller" />
      </n-form-item-gi>

      <n-form-item-gi label="Entity模板">
        <n-input v-model:value="templateConfigStore.entity" />
      </n-form-item-gi>
      <n-form-item-gi label="EntityKotlin模板">
        <n-input v-model:value="templateConfigStore.entityKt" />
      </n-form-item-gi>


      <n-form-item-gi label="Mapper模板">
        <n-input v-model:value="templateConfigStore.mapper" />
      </n-form-item-gi>
      <n-form-item-gi label="Xml模板">
        <n-input v-model:value="templateConfigStore.xml" />
      </n-form-item-gi>

      <n-form-item-gi label="Service模板">
        <n-input v-model:value="templateConfigStore.service" />
      </n-form-item-gi>
      <n-form-item-gi label="ServiceImpl模板">
        <n-input v-model:value="templateConfigStore.serviceImpl" />
      </n-form-item-gi>
    </n-grid>
  </n-form>
</template>

<style lang="css" scoped>
</style>
