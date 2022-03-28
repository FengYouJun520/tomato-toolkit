<template>
  <t-aside :width="appStore.collapse ? '64px' : '232px'">
    <div class="h-full">
      <to-menu-bar />
    </div>
  </t-aside>
</template>
<script setup lang="ts">
import { useAppStore } from '@/store/modules/app'
import ToMenuBar from '../ToMenuBar/index.vue'

const appStore = useAppStore()
</script>

<style scoped>
.t-aside {
  @apply duration-200 ease-linear;
}
</style>
