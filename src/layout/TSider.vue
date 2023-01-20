<script setup lang="ts">
import { NIcon, NLayoutSider, NMenu } from 'naive-ui'

import { Component } from 'vue'
import { RouteRecordRaw, RouterLink } from 'vue-router'
import router, { routes } from '@/router'
import { MenuMixedOption, MenuOption } from 'naive-ui/es/menu/src/interface'
import { useUiState } from '@/store/ui'

const message = useMessage()
const uiState = useUiState()

function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const generateMenuOptions = (routes: RouteRecordRaw[]): MenuMixedOption[] => routes.map(route => {
  const menu: MenuMixedOption = {
    label: () => route.children && route.children.length > 0
      ? route.meta?.title
      : h(
        RouterLink,
        {
          to: { ...route },
        },
        { default: () => route.meta?.title }
      ),
    icon: renderIcon(route.meta?.icon),
    key: router.resolve(route).path,
    children: route.children && route.children.length > 0 ? generateMenuOptions(route.children) : undefined,
  }

  return menu
})

const menuOptions: MenuMixedOption[] = generateMenuOptions(routes)
console.log(routes)


const handleUpdateValue = (key: string, item: MenuOption) =>{
  message.info(`[onUpdate:value]: ${JSON.stringify(key)}`)
  message.info(`[onUpdate:value]: ${JSON.stringify(item)}`)
}
</script>

<template>
  <NLayoutSider
    :native-scrollbar="false"
    collapse-mode="width"
    :collapsed="uiState.isCollapse"
  >
    <NMenu
      :collapsed="uiState.isCollapse"
      :default-value="$route.path"
      :options="menuOptions"
      @update:value="handleUpdateValue"
    />
  </NLayoutSider>
</template>

<style lang="css" scoped>
</style>
