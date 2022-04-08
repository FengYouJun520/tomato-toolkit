<template>
  <div>
    <t-tabs v-model="tabsValue">
      <t-tab-panel label="Entity配置" value="entity-config">
        <entity-config />
      </t-tab-panel>
      <t-tab-panel label="Controller配置" value="controller-config">
        <controller-config />
      </t-tab-panel>
      <t-tab-panel label="Service配置" value="service-config">
        <service-config />
      </t-tab-panel>
      <t-tab-panel label="Mapper配置" value="mapper-config">
        <mapper-config />
      </t-tab-panel>
    </t-tabs>
  </div>
</template>
<script setup lang="ts">
import ControllerConfig from './controller-config.vue'
import EntityConfig from './entity-config.vue'
import MapperConfig from './mapper-config.vue'
import ServiceConfig from './service-config.vue'
const tabsValue = ref('entity-config')
</script>

<style lang="scss" scoped></style>
