<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/mp/strategyconfig'

const strategyconfigStore = useStrategyConfigStore()

const handleReset = () => {
  strategyconfigStore.resetService()
}
</script>

<template>
  <el-form
    label-position="top"
    :model="strategyconfigStore.service"
  >
    <el-form-item label-width="0">
      <el-button type="warning" @click="handleReset">
        重置
      </el-button>
    </el-form-item>
    <el-row :gutter="24">
      <el-col>
        <el-form-item label="覆盖文件">
          <el-radio-group v-model="strategyconfigStore.service.fileOverride">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>

      <el-col :md="12">
        <el-form-item label="设置service接口父类">
          <el-input
            v-model="strategyconfigStore.service.superServiceClass"
            placeholder="示例: com.baomidou.global.BaseService"
          />
        </el-form-item>
      </el-col>
      <el-col :md="12">
        <el-form-item label="设置 service 实现类父类">
          <el-input
            v-model="strategyconfigStore.service.superServiceImplClass"
            placeholder="示例: com.baomidou.global.BaseServiceImpl"
          />
        </el-form-item>
      </el-col>

      <el-col :md="12">
        <el-form-item label="格式化service接口文件名称">
          <el-input
            v-model="strategyconfigStore.service.formatServiceFilename"
            placeholder="示例: {}Service, {}表示占位符"
          />
        </el-form-item>
      </el-col>
      <el-col :md="12">
        <el-form-item label="格式化service实现类文件名称">
          <el-input
            v-model="strategyconfigStore.service.formatServiceImplFilename"
            placeholder="示例: {}ServiceImpl, {}表示占位符"
          />
        </el-form-item>
      </el-col>
    </el-row>
  </el-form>
</template>

<style lang="css" scoped>
</style>
