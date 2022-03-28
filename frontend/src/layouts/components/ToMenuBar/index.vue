<template>
  <t-menu
    :theme="theme"
    :value="$route.path"
    :expanded="expanded"
    :collapsed="appStore.collapse"
    @change="handleChange"
  >
    <template #logo>
      <img
        class="ml-2"
        width="136"
        src="https://www.tencent.com/img/index/menu_logo_hover.png"
        alt="logo"
      />
    </template>

    <to-menu-item :routes="routes" />

    <template #operations>
      <t-icon class="t-menu__operations-icon" name="view-list" @click="appStore.toggleCollapsed" />
    </template>
  </t-menu>
</template>
<script setup lang="ts">
import { routes } from '@/router'
import { useAppStore } from '@/store/modules/app'
import { useThemeStore } from '@/store/modules/theme'
import { MenuValue } from 'tdesign-vue-next'
import { useRoute, useRouter } from 'vue-router'
import ToMenuItem from './ToMenuItem.vue'

const route = useRoute()
const router = useRouter()
const themeStore = useThemeStore()
const appStore = useAppStore()
const theme = computed(() => themeStore.theme)

const expanded = ref<string[]>(route.matched.map((r) => r.path))
const handleChange = (active: MenuValue) => {
  router.push(active.toString())
}
</script>

<style lang="scss" scoped>
.t-default-menu {
  @apply fixed top-0 bottom-0 z-50 min-h-full duration-200 ease-linear;
}
</style>
