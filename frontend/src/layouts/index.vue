<template>
  <t-layout class="w-full h-full">
    <to-aside/>
    <t-layout class="flex-1 overflow-x-hidden">
      <to-header/>
      <to-content/>
      <to-footer/>
    </t-layout>
  </t-layout>
</template>
<script lang="ts" setup>
import ToAside from './components/ToAside/index.vue'
import ToContent from './components/ToContent/index.vue'
import ToFooter from './components/ToFooter/index.vue'
import ToHeader from './components/ToHeader/index.vue'</script>

<style scoped></style>
