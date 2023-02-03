<script setup lang="ts">
import { useInjectConfigStore } from '@/store/modules/injectConfig'


const injectConfigStore = useInjectConfigStore()

const handleReset = () => {
  injectConfigStore.$reset()
}
</script>

<template>
  <n-form
    label-align="right"
    label-placement="left"
    :label-width="210"
  >
    <n-grid cols="1 m:2" responsive="screen" :x-gap="24">
      <n-form-item-gi>
        <n-space class="y-0">
          <n-button type="warning" @click="handleReset">
            清空
          </n-button>
        </n-space>
      </n-form-item-gi>
    </n-grid>
  </n-form>
</template>

<style lang="css" scoped>
</style>
