<template>
  <div>
    <t-form label-align="right" :label-width="160" colon :data="strategyStore.strategy.controller">
      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col>
          <t-button theme="warning" @click="strategyStore.clearStrategy">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col>
          <t-form-item label="开启驼峰转连字符">
            <t-radio-group
              v-model="strategyStore.strategy.controller.enableHyphenStyle"
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="开启@RestController">
            <t-radio-group
              v-model="strategyStore.strategy.controller.enableRestStyle"
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="父类包名">
            <t-input v-model="strategyStore.strategy.controller.superClass" />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="格式化文件名称">
            <t-input v-model="strategyStore.strategy.controller.formatFileName" />
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script setup lang="ts">
import { useStrategy } from '@/store/modules/mybatis/useStrategy'

const strategyStore = useStrategy()
</script>

<style lang="scss" scoped></style>
