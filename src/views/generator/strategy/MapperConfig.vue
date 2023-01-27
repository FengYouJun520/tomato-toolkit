<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/modules/mp/strategyconfig'

const strategyconfigStore = useStrategyConfigStore()

const handleReset = () => {
  strategyconfigStore.resetMapper()
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
      <n-form-item-gi label="开启@Mapper注解">
        <n-radio-group v-model:value="strategyconfigStore.mapper.mapperAnnotation">
          <n-radio-button label="开启" :value="true" />
          <n-radio-button label="关闭" :value="false" />
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="启用@BaseResultMap生成">
        <n-radio-group v-model:value="strategyconfigStore.mapper.baseResultMap">
          <n-radio-button label="开启" :value="true" />
          <n-radio-button label="关闭" :value="false" />
        </n-radio-group>
      </n-form-item-gi>

      <n-form-item-gi label="启用BaseColumnList">
        <n-radio-group v-model:value="strategyconfigStore.mapper.baseColumnList">
          <n-radio-button label="开启" :value="true" />
          <n-radio-button label="关闭" :value="false" />
        </n-radio-group>
      </n-form-item-gi>
    </n-grid>

    <n-form-item label="父类包名">
      <n-input
        v-model:value="strategyconfigStore.mapper.superClass"
        placeholder="示例: com.baomidou.global.BaseMapper"
      />
    </n-form-item>
    <n-form-item label="格式化mapper文件名称">
      <n-input
        v-model:value="strategyconfigStore.mapper.formatMapperFilename"
        placeholder="示例: {}Mapper, {}表示占位符"
      />
    </n-form-item>
    <n-form-item label="格式化xml实现类文件名称">
      <n-input
        v-model:value="strategyconfigStore.mapper.formatXmlFilename"
        placeholder="示例: {}Mapper, {}表示占位符"
      />
    </n-form-item>
  </n-form>
</template>

<style lang="css" scoped>
</style>
