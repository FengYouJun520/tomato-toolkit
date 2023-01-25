<script setup lang="ts">
import { useUiState } from '@/store/ui'
import { MenuFoldOutlined, MenuUnfoldOutlined } from '@vicons/antd'

const uiState = useUiState()
</script>

<template>
  <n-layout-header position="absolute" class="h-16 px-5">
    <div class="h-full flex justify-between items-center">
      <n-button quaternary @click="uiState.toggleCollapse">
        <template #icon>
          <n-icon>
            <MenuFoldOutlined v-if="!uiState.isCollapse" />
            <MenuUnfoldOutlined v-else />
          </n-icon>
        </template>
      </n-button>
    </div>
  </n-layout-header>
</template>

<style lang="css" scoped>
</style>
