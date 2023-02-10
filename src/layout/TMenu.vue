<script setup lang="ts">
import { useUiState } from '@/store/ui'
import { RouteRecordRaw } from 'vue-router'

interface MenuProps {
  routes: RouteRecordRaw[]
}

defineProps<MenuProps>()

const uiState = useUiState()
</script>

<template>
  <template v-for="route in routes" :key="route.path">
    <el-sub-menu
      v-if="route.children && route.children.length"
      :index="route.path"
    >
      <template #title>
        <el-icon v-if="route.meta?.icon">
          <component :is="route.meta?.icon" />
        </el-icon>
        <span>
          {{ route.meta?.title }}
        </span>
      </template>
      <t-menu :routes="route.children" />
    </el-sub-menu>

    <el-menu-item
      v-else
      :index="route.path"
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
