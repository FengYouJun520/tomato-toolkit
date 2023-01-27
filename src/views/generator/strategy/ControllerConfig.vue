<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/modules/mp/strategyconfig'

const strategyconfigStore = useStrategyConfigStore()

const handleReset = () => {
  strategyconfigStore.resetController()
}
</script>

<template>
  <n-form
    label-align="right"
    label-placement="left"
    :label-width="210"
  >
    <n-form-item>
      <n-button type="warning" @click="handleReset">
        重置
      </n-button>
    </n-form-item>

    <n-grid cols="1 m:2" responsive="screen" :x-gap="24">
      <n-form-item-gi label="开启驼峰转连字符">
        <n-radio-group v-model:value="strategyconfigStore.controller.hyphenStyle">
          <n-radio-button label="开启" :value="true" />
          <n-radio-button label="关闭" :value="false" />
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="开启生成@RestController">
        <n-radio-group v-model:value="strategyconfigStore.controller.restStyle">
          <n-radio-button label="开启" :value="true" />
          <n-radio-button label="关闭" :value="false" />
        </n-radio-group>
      </n-form-item-gi>
    </n-grid>

    <n-form-item label="父类包名">
      <n-input
        v-model:value="strategyconfigStore.controller.superClass"
        placeholder="示例: com.baomidou.global.BaseController"
      />
    </n-form-item>
    <n-form-item label="格式化文件名称">
      <n-input
        v-model:value="strategyconfigStore.controller.formatFilename"
        placeholder="示例: {}Controller, {}表示占位符"
      />
    </n-form-item>
  </n-form>
</template>

<style lang="css" scoped>
</style>
