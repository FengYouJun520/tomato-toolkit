<script setup lang="ts">
import { RouteRecordRaw } from 'vue-router'

interface MenuProps {
  routes: RouteRecordRaw[]
  parent?: string
}

defineProps<MenuProps>()

// 获取菜单路径
const getPath = (route: RouteRecordRaw, parent?: string) => {
  if (parent) {
    if (parent === '/') {
      return `/${route.path}`
    } else {
      return `${parent}/${route.path}`
    }
  } else {
    return route.path
  }
}
</script>

<template>
  <template v-for="route in routes" :key="route.path">
    <el-sub-menu
      v-if="route.children && route.children.length"
      :index="getPath(route, parent)"
    >
      <template #title>
        <el-icon v-if="route.meta?.icon">
          <component :is="route.meta?.icon" />
        </el-icon>
        <span>
          {{ route.meta?.title }}
        </span>
      </template>
      <t-menu :routes="route.children" :parent="route.path" />
    </el-sub-menu>

    <el-menu-item
      v-else
      :index="getPath(route, parent)"
      :route="route"
      :title="route.meta?.title"
    >
      <template #title>
        <el-icon v-if="route.meta?.icon">
          <component :is="route.meta?.icon" />
        </el-icon>
        <span>
          {{ route.meta?.title }}
        </span>
      </template>
    </el-menu-item>
  </template>
</template>

<style lang="css" scoped>

</style>
