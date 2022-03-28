<template>
  <t-header>
    <div class="flex items-center justify-between px-6">
      <div class="header-left">
        <t-button theme="default" variant="text" @click="appStore.toggleCollapsed">
          <t-icon name="view-list" />
        </t-button>
      </div>
      <div class="header-right">
        <t-button shape="circle" theme="primary">
          <template #icon>
            <t-icon name="discount" />
          </template>
        </t-button>
        <t-button shape="circle" theme="primary">
          <template #icon>
            <t-icon name="cloud-download" />
          </template>
        </t-button>
      </div>
    </div>
  </t-header>
</template>
<script setup lang="ts">
import { useAppStore } from '@/store/modules/app'

const appStore = useAppStore()
</script>

<style scoped></style>
