<template>
  <div>
    <t-form :data="strategyStore.strategy.service" :label-width="180" colon label-align="right">
      <t-row :gutter="[24, 24]" mt-6>
        <t-col>
          <t-button theme="warning" @click="strategyStore.clearStrategy">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" mt-6>
        <t-col>
          <t-form-item label="格式化service接口名称">
            <t-input v-model="strategyStore.strategy.service.formatServiceFileName"/>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="格式化service实现类名称">
            <t-input v-model="strategyStore.strategy.service.formatServiceImplFileName"/>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="service父类接口">
            <t-input v-model="strategyStore.strategy.service.superServiceClass"/>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="service实现类父类接口">
            <t-input v-model="strategyStore.strategy.service.superServiceImplClass"/>
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script lang="ts" setup>
import {useStrategy} from '@/store/modules/mybatis/useStrategy'

const strategyStore = useStrategy()
</script>

<style lang="scss" scoped></style>
