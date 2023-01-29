<script setup lang="ts">
import tomatoIcon from '@/assets/tomato.svg'
import { app } from '@tauri-apps/api'

const version = ref('0.1.0')
const name = ref('tomato')

onMounted(async () => {
  version.value = await app.getVersion()
  name.value = await app.getName()
})
</script>

<template>
  <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
    <n-image
      :width="300"
      :height="300"
      object-fit="cover"
      :src="tomatoIcon"
      preview-disabled
    />

    <n-space justify="center">
      <n-tag type="info">
        Project:&nbsp;{{ name }}
      </n-tag>
      <n-tag type="info">
        Version:&nbsp;{{ version }}
      </n-tag>
    </n-space>
  </div>
</template>

<style lang="css" scoped>
</style>
