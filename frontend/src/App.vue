<script setup lang="ts">
import { useThemeStore } from '@/store/modules/theme'

const themeStore = useThemeStore()
watchEffect(() => {
  if (themeStore.getTheme === 'dark') {
    document.documentElement.setAttribute('theme-mode', 'dark')
  } else {
    document.documentElement.removeAttribute('theme-mode')
  }
})
</script>

<template>
  <t-config-provider>
    <router-view />
  </t-config-provider>
</template>

<style></style>
