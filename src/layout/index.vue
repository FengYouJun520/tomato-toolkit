<script setup lang="ts">
import THeader from './THeader.vue'
import TSider from './TSider.vue'
</script>

<template>
  <n-layout has-sider position="absolute">
    <TSider />
    <n-layout>
      <THeader />
      <n-layout-content
        class="mt-16"
        position="absolute"
        :native-scrollbar="false">
        <router-view />
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<style lang="css" scoped>
</style>
