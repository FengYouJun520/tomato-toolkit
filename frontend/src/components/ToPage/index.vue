<template>
  <to-card :shadow="false">
    <template #title>
      <slot name="title"> {{ title }} </slot>
    </template>
    <template #default>
      <slot name="default" />
    </template>
    <template #footer>
      <slot name="footer"> {{ footer }} </slot>
    </template>
  </to-card>
</template>
<script setup lang="ts">
import ToCard from '../ToCard/index.vue'

interface PageWrapProps {
  title?: string
  footer?: string
}

defineProps<PageWrapProps>()
</script>

<style lang="scss" scoped></style>
