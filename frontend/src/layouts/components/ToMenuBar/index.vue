<template>
  <t-menu
    v-model:expanded="expanded"
    :collapsed="appStore.collapse"
    :default-value="$route.path"
    :theme="theme"
    @change="handleChange"
  >
    <template #logo>
      <div v-if="!appStore.collapse" class="flex items-center overflow-hidden">
        <div class="ml-4 overflow-hidden text-2xl">番茄工具集</div>
      </div>
    </template>

    <to-menu-item :routes="routes"/>

    <template #operations>
      <t-icon class="t-menu__operations-icon" name="view-list" @click="appStore.toggleCollapsed"/>
    </template>
  </t-menu>
</template>
<script lang="ts" setup>
import {routes} from '@/router'
import {useAppStore} from '@/store/modules/app'
import {useThemeStore} from '@/store/modules/theme'
import {MenuValue} from 'tdesign-vue-next'
import {useRoute, useRouter} from 'vue-router'
import ToMenuItem from './ToMenuItem.vue'

const route = useRoute()
const router = useRouter()
const themeStore = useThemeStore()
const appStore = useAppStore()
const theme = computed(() => themeStore.getTheme)

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
