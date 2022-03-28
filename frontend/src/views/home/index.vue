<template>
  <to-page title="首页" extractClass="gap-3" operationClass="justify-end">
    <t-button theme="primary">primary</t-button>
    <t-alert theme="success" message="这是一条成功的消息提示" />
    <t-button @click="themeStore.toggleTheme">切换主题</t-button>
    <t-button theme="success">success</t-button>
    <t-button theme="warning">warning</t-button>
    <t-button theme="danger">danger</t-button>

    <template #extract>
      <t-button theme="warning">warning</t-button>
      <t-button theme="danger">danger</t-button>
    </template>
    <template #footer>
      <span>footer</span>
    </template>
    <template #operation>
      <t-button theme="warning">warning</t-button>
      <t-button theme="danger">danger</t-button>
    </template>
  </to-page>
</template>
<script setup lang="ts">
import ToPage from '@/components/ToPage/index.vue'
import { useThemeStore } from '@/store/modules/theme'

const themeStore = useThemeStore()
</script>

<style lang="scss" scoped></style>
