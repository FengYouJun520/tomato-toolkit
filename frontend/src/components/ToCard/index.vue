<template>
  <div class="to-card" :class="classes">
    <div class="to-card__inner">
      <div class="to-card__title" :class="[titleClass]" v-if="$slots.title || title">
        <slot name="title">
          {{ title }}
        </slot>
      </div>
      <div class="to-card__content" :class="contentClass">
        <slot name="default" />
      </div>
      <div class="to-card__footer" :class="footerClass" v-if="$slots.footer || title">
        <slot name="footer">
          {{ footer }}
        </slot>
      </div>
      <div class="to-card__extract" :class="extractClass" v-if="$slots.extract">
        <slot name="extract" />
      </div>
      <div class="to-card-operation" :class="operationClass" v-if="$slots.operation">
        <slot name="operation" />
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
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
