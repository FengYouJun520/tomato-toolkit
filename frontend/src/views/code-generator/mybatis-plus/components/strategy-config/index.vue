<template>
  <t-row :gutter="24">
    <t-col :lg="6" :xs="12">
      <strategy-left/>
    </t-col>
    <t-col :lg="6" :xs="12">
      <strategy-right/>
    </t-col>
  </t-row>
</template>
<script lang="ts" setup>
import StrategyLeft from './strategy-left.vue'
import StrategyRight from './strategy-right/index.vue'</script>

<style lang="scss" scoped></style>
