<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/mp/strategyconfig'

const strategyconfigStore = useStrategyConfigStore()

const handleReset = () => {
  strategyconfigStore.resetService()
}
</script>

<template>
  <n-form
    label-align="right"
    label-placement="left"
    :label-width="210"
  >
    <n-grid cols="1 m:2" :x-gap="24" responsive="screen">
      <n-form-item-gi>
        <n-space class="y-0">
          <n-button type="warning" @click="handleReset">
            重置
          </n-button>
        </n-space>
      </n-form-item-gi>
      <n-form-item-gi label="覆盖文件">
        <n-radio-group v-model:value="strategyconfigStore.service.fileOverride">
          <n-radio-button label="开启" :value="true" />
          <n-radio-button label="关闭" :value="false" />
        </n-radio-group>
      </n-form-item-gi>
    </n-grid>

    <n-form-item label="设置service接口父类">
      <n-input
        v-model:value="strategyconfigStore.service.superServiceClass"
        placeholder="示例: com.baomidou.global.BaseService"
      />
    </n-form-item>
    <n-form-item label="设置 service 实现类父类">
      <n-input
        v-model:value="strategyconfigStore.service.superServiceImplClass"
        placeholder="示例: com.baomidou.global.BaseServiceImpl"
      />
    </n-form-item>
    <n-form-item label="格式化service接口文件名称">
      <n-input
        v-model:value="strategyconfigStore.service.formatServiceFilename"
        placeholder="示例: {}Service, {}表示占位符"
      />
    </n-form-item>
    <n-form-item label="格式化service实现类文件名称">
      <n-input
        v-model:value="strategyconfigStore.service.formatServiceImplFilename"
        placeholder="示例: {}ServiceImpl, {}表示占位符"
      />
    </n-form-item>
  </n-form>
</template>

<style lang="css" scoped>
</style>
