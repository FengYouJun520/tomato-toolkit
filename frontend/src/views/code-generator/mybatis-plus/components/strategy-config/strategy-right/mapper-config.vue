<template>
  <div>
    <t-form :data="strategyStore.strategy.mapper" :label-width="160" colon label-align="right">
      <t-row :gutter="[24, 24]" mt-6>
        <t-col>
          <t-button theme="warning" @click="strategyStore.clearStrategy">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" mt-6>
        <t-col>
          <t-form-item label="开启@Mapper注解">
            <t-radio-group
              v-model="strategyStore.strategy.mapper.enableMapperAnnotation"
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="设置通用父类">
            <t-input v-model="strategyStore.strategy.mapper.superClass"/>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="格式化mapper名称">
            <t-input v-model="strategyStore.strategy.mapper.formatMapperFileName"/>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="格式化xml实现类名称">
            <t-input v-model="strategyStore.strategy.mapper.formatXmlFileName"/>
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
