<template>
  <div :class="classes" class="to-card">
    <div class="to-card__inner">
      <div v-if="$slots.title || title" :class="[titleClass]" class="to-card__title">
        <slot name="title">
          {{ title }}
        </slot>
      </div>
      <div :class="contentClass" class="to-card__content">
        <slot name="default"/>
      </div>
      <div v-if="$slots.footer || title" :class="footerClass" class="to-card__footer">
        <slot name="footer">
          {{ footer }}
        </slot>
      </div>
      <div v-if="$slots.extract" :class="extractClass" class="to-card__extract">
        <slot name="extract"/>
      </div>
      <div v-if="$slots.operation" :class="operationClass" class="to-card-operation">
        <slot name="operation"/>
      </div>
    </div>
  </div>
</template>
<script lang="ts" setup>
interface CardProps {
  trigger?: 'hover' | 'always'
  shadow?: boolean
  title?: string
  footer?: string
  titleClass?: string
  contentClass?: string
  footerClass?: string
  operationClass?: string
  extractClass?: string
}

const props = withDefaults(defineProps<CardProps>(), {
  trigger: 'always',
  shadow: true,
})

const classes = reactive({
  'shadow-lg': props.trigger === 'always' && props.shadow,
  'hover:shadow-lg hover:duration-200': props.trigger === 'hover' && props.shadow,
})
</script>

<style lang="scss" scoped>
.to-card {
  @apply w-full h-full rounded;
  color: var(--td-text-color-secondary);
  background-color: white;

  &__inner {
    @apply p-3 w-full h-full relative flex flex-col gap-6;
  }

  &__title {
    @apply flex items-center text-2xl pb-3;
    border-bottom: 1px solid var(--td-gray-color-2);
  }

  &__content {
    flex: 1;
  }

  &__footer {
  }

  &__extract {
    @apply absolute right-3 top-3 flex items-center;
  }

  &-operation {
  }
}

html[theme-mode='dark'] {
  .to-card {
    background-color: var(--td-bg-color-container) !important;
  }

  .to-card__title {
    border-bottom-color: var(--td-gray-color-10);
  }
}
</style>
