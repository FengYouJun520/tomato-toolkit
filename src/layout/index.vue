<script setup lang="ts">
import THeader from './THeader.vue'
import TSider from './TSider.vue'
</script>

<template>
  <el-container class="h-full">
    <TSider />
    <el-container direction="vertical">
      <THeader />
      <el-main class="relative">
        <el-scrollbar>
          <router-view />
        </el-scrollbar>
      </el-main>
    </el-container>
  </el-container>
</template>

<style lang="css" scoped>
.el-main {
  background-color: var(--el-bg-color-page);
}

.el-scrollbar {
  height: 100%;
}
</style>
