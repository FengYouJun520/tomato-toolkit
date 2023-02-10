<script setup lang="ts">
import { routes } from '@/router'
import { useUiState } from '@/store/ui'
import TMenu from './TMenu.vue'

const uiState = useUiState()

const asideWidth = computed(() => uiState.collapse ? 64 : uiState.asideWidth)
</script>

<template>
  <el-aside :width="`${asideWidth}px`">
    <el-scrollbar style="height: 100%;">
      <el-menu
        class="h-full w-full"
        router
        :collapse="uiState.collapse"
        :default-active="$route.fullPath"
      >
        <TMenu :routes="routes" />
      </el-menu>
    </el-scrollbar>
  </el-aside>
</template>

<style lang="css" scoped>
.el-aside {
  transition: width 0.2s;
  border-right: 1px solid var(--el-border-color);
}

.el-menu {
  border-right: none;
  transition: width 0.2s;
}
</style>
