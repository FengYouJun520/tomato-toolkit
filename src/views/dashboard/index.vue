<script setup lang="ts">
import tomatoIcon from '@/assets/tomato.svg'
import { app } from '@tauri-apps/api'
import PageLayout from '@/components/PageLayout/index.vue'

const version = ref('0.0.0')
const name = ref('tomato')

onMounted(async () => {
  version.value = await app.getVersion()
  name.value = await app.getName()
})
</script>

<template>
  <PageLayout>
    <div class="flex h-full flex-col space-y-5 items-center justify-center">
      <el-image
        :src="tomatoIcon"
        fit="cover"
        style="height: 300px;width: 300px;"
      />

      <el-space>
        <el-tag type="success" size="large">
          Project:&nbsp;{{ name }}
        </el-tag>
        <el-tag type="success" size="large">
          Version:&nbsp;{{ version }}
        </el-tag>
      </el-space>
    </div>
  </PageLayout>
</template>

<style lang="css" scoped>
</style>
