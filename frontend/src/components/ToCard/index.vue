<template>
  <div class="to-card" :class="classes">
    <div class="to-card__inner">
      <div class="to-card__title" v-if="$slots.title || title">
        <slot name="title">
          {{ title }}
        </slot>
      </div>
      <div class="to-card__content">
        <slot name="default" />
      </div>
      <div class="to-card__footer" v-if="$slots.footer || title">
        <slot name="footer">
          {{ footer }}
        </slot>
      </div>
      <div class="to-card-extract-left" v-if="$slots.extractLeft">
        <slot name="extract-left" />
      </div>
      <div class="to-card-extract-right" v-if="$slots.extractRight">
        <slot name="extract-right" />
      </div>
      <div class="to-card-operation" v-if="$slots.operation">
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
}

const props = withDefaults(defineProps<CardProps>(), {
  trigger: 'always',
  shadow: true,
})

const classes = reactive({
  'shadow-lg': props.trigger === 'always' && props.shadow,
  'hover:shadow-lg': props.trigger === 'hover' && props.shadow,
})
</script>

<style lang="scss" scoped>
.to-card {
  @apply w-full h-full duration-200 rounded;
  color: var(--td-text-color-secondary);
  background-color: white;

  &__inner {
    @apply p-3 w-full h-full relative flex flex-col gap-3;
  }

  &__title {
    @apply flex items-center text-2xl pb-3;
    border-bottom: 1px solid var(--td-gray-color-2);
  }

  &__content {
    flex: 1;
  }

  &__footer {
    @apply flex items-center;
  }

  &-extract-left {
  }

  &-extract-right {
  }

  &-operation {
  }
}

html[theme-mode='dark'] {
  .to-card {
    background-color: var(--td-bg-color-secondarycontainer) !important;
  }

  .to-card__title {
    border-bottom-color: var(--td-gray-color-10);
  }
}
</style>
