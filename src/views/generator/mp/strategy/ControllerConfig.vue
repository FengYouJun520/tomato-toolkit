<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/mp/strategyconfig'

const strategyconfigStore = useStrategyConfigStore()

const handleReset = () => {
  strategyconfigStore.resetController()
}
</script>

<template>
  <el-form
    label-position="top"
    :model="strategyconfigStore.controller"
  >
    <el-form-item label-width="0">
      <n-button type="warning" @click="handleReset">
        重置
      </n-button>
    </el-form-item>

    <el-row :gutter="24">
      <el-col :sm="12" :md="8">
        <el-form-item label="覆盖文件">
          <el-radio-group v-model="strategyconfigStore.controller.fileOverride">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>

      <el-col :sm="12" :md="8">
        <el-form-item label="开启驼峰转连字符">
          <el-radio-group v-model="strategyconfigStore.controller.hyphenStyle">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>
      <el-col :sm="12" :md="8">
        <el-form-item label="开启生成@RestController">
          <el-radio-group v-model="strategyconfigStore.controller.restStyle">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>
    </el-row>

    <el-row :gutter="24">
      <el-col :md="12">
        <el-form-item label="父类包名">
          <el-input
            v-model="strategyconfigStore.controller.superClass"
            placeholder="示例: com.baomidou.global.BaseController"
          />
        </el-form-item>
      </el-col>

      <el-col :md="12">
        <el-form-item label="格式化文件名称">
          <el-input
            v-model="strategyconfigStore.controller.formatFilename"
            placeholder="示例: {}Controller, {}表示占位符"
          />
        </el-form-item>
      </el-col>
    </el-row>
  </el-form>
</template>

<style lang="css" scoped>
</style>
