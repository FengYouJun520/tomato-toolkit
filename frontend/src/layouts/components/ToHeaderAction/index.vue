<template>
  <div class="mr-4">
    <t-drawer v-model:visible="visible" header="页面设置" :confirm-btn="null" :cancel-btn="null">
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
    <t-button variant="text" shape="square" @click="visible = true">
      <template #icon>
        <t-icon size="20" name="setting" />
      </template>
    </t-button>
  </div>
</template>
<script setup lang="ts">
import DarkIcon from '@/assets/moon.svg'
import LightIcon from '@/assets/sun.svg'
import { useThemeStore } from '@/store/modules/theme'

const ThemeModes = [
  {
    type: 'light',
    label: '明亮',
  },
  {
    type: 'dark',
    label: '暗黑',
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
}

const handleThemeChange = () => {
  themeStore.toggleTheme()
}
</script>

<style lang="scss" scoped>
:deep(.t-drawer__content-wrapper) {
  width: 400px !important;
}

.setting-title {
  @apply my-6 inline-block w-full text-base;
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
    border: 2px solid #e3e6eb;

    > :deep(.t-radio-button__label) {
      display: inline-flex;
      background-color: var(--td-gray-color-5);

      svg {
        padding: 8px;
      }
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
