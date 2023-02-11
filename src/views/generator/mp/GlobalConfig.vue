<script setup lang="ts">
import { useGlobalConfigStore } from '@/store/mp/globalconfig'
import { dialog } from '@tauri-apps/api'

const dateTypeoptions = [
  {
    label: 'ONLY_DATE',
    value: 'ONLY_DATE',
  },
  {
    label: 'SQL_PACK',
    value: 'SQL_PACK',
  },
  {
    label: 'TIME_PACK',
    value: 'TIME_PACK',
  },
]

const globalConfigStore = useGlobalConfigStore()

const handleReset = () => {
  globalConfigStore.$reset()
}

const selectDirectory = async () => {
  const outputDir = await dialog.open({
    directory: true,
    title: '选择输出目录',
  })

  if (outputDir) {
    globalConfigStore.outputDir = outputDir as string
  }
}
</script>

<template>
  <el-form
    :model="globalConfigStore.$state"
    :label-width="120"
  >
    <el-form-item label-width="0">
      <el-button type="warning" @click="handleReset">
        重置
      </el-button>
    </el-form-item>

    <el-form-item label="作者">
      <el-input v-model="globalConfigStore.author" />
    </el-form-item>

    <el-row :gutter="24">
      <el-col :xs="24" :md="12">
        <el-form-item label="打开输出目录">
          <el-radio-group v-model="globalConfigStore.open">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>
      <el-col :xs="24" :md="12">
        <el-form-item label="开启swagger">
          <el-radio-group v-model="globalConfigStore.swagger">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>

      <el-col :xs="24" :md="12">
        <el-form-item label="开启springdoc">
          <el-radio-group v-model="globalConfigStore.springdoc">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>
      <el-col :xs="24" :md="12">
        <el-form-item label="开启kotlin模式">
          <el-radio-group v-model="globalConfigStore.kotlin">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>

      <el-col :xs="24" :md="12">
        <el-form-item label="注释日期格式">
          <el-input
            v-model="globalConfigStore.commentDate"
            placeholder="示例：%Y-%m-%d %H:%M:%S"
          />
        </el-form-item>
      </el-col>
      <el-col :xs="24" :md="12">
        <el-form-item label="时间策略">
          <el-select v-model="globalConfigStore.dateType">
            <el-option
              v-for="option in dateTypeoptions"
              :key="option.value"
              :label="option.label"
              :value="option.value"
            />
          </el-select>
        </el-form-item>
      </el-col>
    </el-row>

    <el-row>
      <el-col :sm="18" :md="20">
        <el-form-item label="输出目录">
          <el-input v-model="globalConfigStore.outputDir" />
        </el-form-item>
      </el-col>

      <el-col :sm="6" :md="4">
        <el-button type="primary" @click="selectDirectory">
          选择目录
        </el-button>
      </el-col>
    </el-row>
  </el-form>
</template>

<style lang="css" scoped>
</style>
