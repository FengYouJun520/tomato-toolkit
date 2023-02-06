<template>
  <template v-for="route in menus" :key="route.path">
    <template v-if="!route.meta?.hidden">
      <!-- submenu -->
      <template v-if="route.children && route.children.length">
        <t-submenu :value="route.path">
          <template v-if="route.meta?.icon" #icon>
            <t-icon :name="route.meta?.icon"/>
          </template>
          <template #title>
            {{ route.meta?.title }}
          </template>
          <to-menu-item :routes="route.children"/>
        </t-submenu>
      </template>

      <!-- menuitem -->
      <template v-else>
        <t-menu-item
          :href="route.meta?.isLink ? route.path : undefined"
          :router="route"
          :value="route.path"
        >
          <template v-if="route.meta?.icon" #icon>
            <t-icon :name="route.meta?.icon"/>
          </template>
          {{ route.meta?.title }}
        </t-menu-item>
      </template>
    </template>
  </template>
</template>
<script lang="ts" setup>
import {RouteRecordRaw} from 'vue-router'

interface MenuBarProps {
  routes: RouteRecordRaw[]
}

const props = defineProps<MenuBarProps>()

const getMenus = (routes: RouteRecordRaw[], parent: string = ''): RouteRecordRaw[] => {
  const res: RouteRecordRaw[] = []
  routes.forEach((route) => {
    let path = parent + '/' + route.path

    path = path.endsWith('/') ? path.slice(0, path.length - 1) : path
    path = path.replace(/\/{2,}/, '/')

    const finalRoute = {...route, path: path}
    res.push(finalRoute)

    if (route.children && route.children.length) {
      finalRoute.children = getMenus(route.children, parent + route.path)
    }
  })
  return res
}

const menus = getMenus(props.routes)
</script>

<style lang="scss" scoped></style>
