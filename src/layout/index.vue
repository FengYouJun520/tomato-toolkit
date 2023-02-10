<script setup lang="ts">
import THeader from './THeader.vue'
import TSider from './TSider.vue'
</script>

<template>
  <el-container class="h-full">
    <TSider />
    <el-container direction="vertical">
      <THeader />
      <el-main :style="{ backgroundColor: 'var(--el-bg-color-page)' }">
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<style lang="css" scoped>
</style>
