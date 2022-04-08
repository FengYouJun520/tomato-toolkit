<template>
  <t-row :gutter="24">
    <t-col :xs="12" :lg="6">
      <strategy-left />
    </t-col>
    <t-col :xs="12" :lg="6">
      <strategy-right />
    </t-col>
  </t-row>
</template>
<script setup lang="ts">
import StrategyLeft from './strategy-left.vue'
import StrategyRight from './strategy-right/index.vue'
</script>

<style lang="scss" scoped></style>
