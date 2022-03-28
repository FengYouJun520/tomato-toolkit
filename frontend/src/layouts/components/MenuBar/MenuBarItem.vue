<template>
  <template v-for="route in routes" :key="route.path">
    <template v-if="!route.meta?.hidden">
      <!-- submenu -->
      <template v-if="route.children && route.children.length">
        <t-submenu :value="route.path">
          <template v-if="route.meta?.icon" #icon>
            <t-icon :name="route.meta?.icon" />
          </template>
          <template #title>
            {{ route.meta?.title }}
          </template>
          <menu-bar-item :routes="route.children" />
        </t-submenu>
      </template>

      <!-- menuitem -->
      <template v-else>
        <t-menu-item
          :value="route.path"
          :router="route"
          :href="route.meta?.isLink ? route.path : undefined"
        >
          <template v-if="route.meta?.icon" #icon>
            <t-icon :name="route.meta?.icon" />
          </template>
          {{ route.meta?.title }}
        </t-menu-item>
      </template>
    </template>
  </template>
</template>
<script setup lang="ts">
import { RouteRecordRaw } from 'vue-router'

interface MenuBarProps {
  routes: RouteRecordRaw[]
}

defineProps<MenuBarProps>()
</script>

<style lang="scss" scoped></style>
