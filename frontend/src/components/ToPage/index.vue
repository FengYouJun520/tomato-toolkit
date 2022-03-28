<template>
  <to-card :shadow="false">
    <template #title v-if="title || $slots.title">
      <slot name="title"> {{ title }} </slot>
    </template>
    <template #default>
      <slot name="default" />
    </template>
    <template #footer v-if="footer || $slots.footer">
      <slot name="footer"> {{ footer }} </slot>
    </template>
    <template #extract v-if="$slots.extract">
      <slot name="extract" />
    </template>
    <template #operation v-if="$slots.operation">
      <slot name="operation" />
    </template>
  </to-card>
</template>
<script setup lang="ts">
import ToCard from '../ToCard/index.vue'

interface PageWrapProps {
  title?: string
  footer?: string
  titleClass?: string
  contentClass?: string
  footerClass?: string
  operationClass?: string
  extractClass?: string
}

defineProps<PageWrapProps>()
</script>

<style lang="scss" scoped></style>
