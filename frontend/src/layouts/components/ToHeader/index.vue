<template>
  <t-header>
    <div class="flex items-center justify-between">
      <div class="ml-4 header-lef">
        <t-button shape="square" theme="default" variant="text" @click="appStore.toggleCollapsed">
          <t-icon name="view-list" size="20"/>
        </t-button>
      </div>
      <div class="header-right">
        <to-header-action/>
      </div>
    </div>
  </t-header>
</template>
<script lang="ts" setup>
import {useAppStore} from '@/store/modules/app'
import ToHeaderAction from '../ToHeaderAction/index.vue'

const appStore = useAppStore()
</script>

<style lang="scss" scoped>
.header-right {
  @apply flex items-center mr-6;
}
</style>
