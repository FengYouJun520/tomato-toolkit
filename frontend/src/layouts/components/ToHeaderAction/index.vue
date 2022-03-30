<template>
  <div>
    <t-button variant="text" shape="square" @click="visible = true" class="w-12 h-16">
      <template #icon>
        <t-icon size="20" name="setting" />
      </template>
    </t-button>

    <t-drawer v-model:visible="visible" header="页面设置" :footer="false">
      <t-form>
        <div class="setting-title">主题设置</div>
        <t-radio-group class="gap-x-3" :value="themeStore.theme" @change="handleThemeChange">
          <div v-for="mode in ThemeModes" :key="mode.type" class="radio-setting">
            <div class="flex flex-col items-center">
              <t-radio-button :key="mode.type" :value="mode.type">
                <component :is="getModeIcon(mode.type)" class="w-20 h-10" />
              </t-radio-button>
              <p>{{ mode.label }}</p>
            </div>
          </div>
        </t-radio-group>
      </t-form>
    </t-drawer>
  </div>
</template>
<script setup lang="ts">
import AutoIcon from '@/assets/auto.svg'
import DarkIcon from '@/assets/moon.svg'
import LightIcon from '@/assets/sun.svg'
import { ThemeType, useThemeStore } from '@/store/modules/theme'
import { RadioValue } from 'tdesign-vue-next'

const ThemeModes = [
  {
    type: 'light',
    label: '明亮',
  },
  {
    type: 'dark',
    label: '暗黑',
  },
  {
    type: 'auto',
    label: '跟随系统',
  },
]

const visible = ref(false)
const themeStore = useThemeStore()

const getModeIcon = (type: string) => {
  if (type === 'light') {
    return LightIcon
  }
  if (type === 'dark') {
    return DarkIcon
  }
  if (type === 'auto') {
    return AutoIcon
  }
}

const handleThemeChange = (theme: RadioValue) => {
  console.log(theme)

  themeStore.toggleTheme(theme.toString() as ThemeType)
}
</script>

<style lang="scss" scoped>
:deep(.t-drawer__content-wrapper) {
  width: 400px !important;
}

.setting-title {
  @apply my-6 inline-block w-full text-base;
  color: var(--td-text-color-primary);
}

.radio-setting {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 16px;

  p {
    color: var(--td-text-color-primary);
  }

  .t-radio-button {
    display: inline-flex;
    max-height: 78px;
    padding: 8px;
    border-radius: 2px;
    border: 2px solid var(--td-gray-color-5);

    > :deep(.t-radio-button__label) {
      display: inline-flex;
      background-color: var(--td-gray-color-5);

      svg {
        padding: 8px;
      }
    }
  }

  &:nth-child(2) :deep(.t-radio-button__label) {
    background-color: var(--td-gray-color-11);

    svg {
      fill: var(--td-gray-color-4);
    }
  }

  .t-is-checked {
    border: 2px solid var(--td-brand-color) !important;
  }
  .t-form__controls-content {
    justify-content: end;
  }
}
</style>
