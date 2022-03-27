<template>
  <div>
    <t-button theme="primary">primary</t-button>
    <t-alert theme="success" message="这是一条成功的消息提示" />
    <t-button @click="themeStore.toggleTheme">切换主题</t-button>
  </div>
</template>
<script setup lang="ts">
import { useThemeStore } from '@/store/modules/theme'

const themeStore = useThemeStore()
</script>

<style lang="scss" scoped></style>
