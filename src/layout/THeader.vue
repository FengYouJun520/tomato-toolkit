<script setup lang="ts">
import { useUiState } from '@/store/ui'
import { Expand, Fold } from '@element-plus/icons-vue'
const uiState = useUiState()
</script>

<template>
  <el-header height="64px">
    <div class="h-full flex justify-between items-center">
      <el-button text @click="uiState.toggleCollapse">
        <template #icon>
          <el-icon>
            <Expand v-if="uiState.isCollapse" />
            <Fold v-else />
          </el-icon>
        </template>
      </el-button>
    </div>
  </el-header>
</template>

<style lang="css" scoped>
.el-header {
  border-bottom: 1px solid var(--el-border-color);
}
</style>
