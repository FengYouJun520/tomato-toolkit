<template>
  <t-header>
    <div class="flex items-center justify-between">
      <div class="ml-4 header-lef">
        <t-button theme="default" shape="square" variant="text" @click="appStore.toggleCollapsed">
          <t-icon size="20" name="view-list" />
        </t-button>
      </div>
      <div class="header-right">
        <to-header-action />
      </div>
    </div>
  </t-header>
</template>
<script setup lang="ts">
import { useAppStore } from '@/store/modules/app'
import ToHeaderAction from '../ToHeaderAction/index.vue'

const appStore = useAppStore()
</script>

<style lang="scss" scoped>
.header-right {
  @apply flex items-center mr-6;
}
</style>
