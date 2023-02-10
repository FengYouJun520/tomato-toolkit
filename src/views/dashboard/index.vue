<script setup lang="ts">
import tomatoIcon from '@/assets/tomato.svg'
import { app } from '@tauri-apps/api'

const version = ref('0.0.0')
const name = ref('tomato')

onMounted(async () => {
  version.value = await app.getVersion()
  name.value = await app.getName()
})
</script>

<template>
  <div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
    <el-image
      :src="tomatoIcon"
      fit="cover"
      style="height: 300px;width: 300px;"
    />

    <el-space>
      <el-tag type="success">
        Project:&nbsp;{{ name }}
      </el-tag>
      <el-tag type="success">
        Version:&nbsp;{{ version }}
      </el-tag>
    </el-space>
  </div>
</template>

<style lang="css" scoped>
</style>
