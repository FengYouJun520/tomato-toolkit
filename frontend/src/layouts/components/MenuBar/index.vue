<template>
  <t-menu
    :theme="theme"
    style="margin-right: 50px"
    @change="changeHandler"
    :value="$route.path"
    :expanded="expanded"
    :collapsed="appStore.collapse"
    @collapsed="onCollapse"
  >
    <menu-bar-item :routes="routes" />
  </t-menu>
</template>
<script setup lang="ts">
import { routes } from '@/router'
import { useAppStore } from '@/store/modules/app'
import { useThemeStore } from '@/store/modules/theme'
import { MenuValue } from 'tdesign-vue-next'
import { useRoute, useRouter } from 'vue-router'
import MenuBarItem from './MenuBarItem.vue'

const route = useRoute()
const router = useRouter()
const themeStore = useThemeStore()
const appStore = useAppStore()
const theme = computed(() => themeStore.theme)

const expanded = ref<string[]>(route.matched.map((r) => r.path))

const changeHandler = (active: MenuValue) => {
  router.push(active.toString())
}
const onCollapse = (options: { collapsed: boolean }) => {
  appStore.onCollapsed(options.collapsed)
}
</script>

<style lang="scss" scoped></style>
