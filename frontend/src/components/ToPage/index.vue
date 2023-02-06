<template>
  <to-card :shadow="false">
    <template v-if="title || $slots.title" #title>
      <slot name="title"> {{ title }}</slot>
    </template>
    <template #default>
      <slot name="default"/>
    </template>
    <template v-if="footer || $slots.footer" #footer>
      <slot name="footer"> {{ footer }}</slot>
    </template>
    <template v-if="$slots.extract" #extract>
      <slot name="extract"/>
    </template>
    <template v-if="$slots.operation" #operation>
      <slot name="operation"/>
    </template>
  </to-card>
</template>
<script lang="ts" setup>
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
