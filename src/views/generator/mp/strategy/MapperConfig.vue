<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/mp/strategyconfig'

const strategyconfigStore = useStrategyConfigStore()

const handleReset = () => {
  strategyconfigStore.resetMapper()
}
</script>

<template>
  <el-form
    label-position="top"
    :model="strategyconfigStore.mapper"
  >
    <el-form-item label-width="0">
      <el-button type="warning" @click="handleReset">
        重置
      </el-button>
    </el-form-item>

    <el-row :gutter="24">
      <el-col :sm="12">
        <el-form-item label="覆盖文件">
          <el-radio-group v-model="strategyconfigStore.mapper.fileOverride">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>
      <el-col :sm="12">
        <el-form-item label="开启@Mapper注解">
          <el-radio-group v-model="strategyconfigStore.mapper.mapperAnnotation">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>

      <el-col :sm="12">
        <el-form-item label="启用BaseResultMap生成">
          <el-radio-group v-model="strategyconfigStore.mapper.baseResultMap">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>
      <el-col :sm="12">
        <el-form-item label="启用BaseColumnList">
          <el-radio-group v-model="strategyconfigStore.mapper.baseColumnList">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>

      <el-col :md="24">
        <el-form-item label="父类包名">
          <el-input
            v-model="strategyconfigStore.mapper.superClass"
            placeholder="默认: com.baomidou.mybatisplus.core.mapper.BaseMapper"
          />
        </el-form-item>
      </el-col>
      <el-col :md="12">
        <el-form-item label="格式化mapper文件名称">
          <el-input
            v-model="strategyconfigStore.mapper.formatMapperFilename"
            placeholder="示例: {}Mapper, {}表示占位符"
          />
        </el-form-item>
      </el-col>
      <el-col :md="12">
        <el-form-item label="格式化xml实现类文件名称">
          <el-input
            v-model="strategyconfigStore.mapper.formatXmlFilename"
            placeholder="示例: {}Mapper, {}表示占位符"
          />
        </el-form-item>
      </el-col>
    </el-row>
  </el-form>
</template>

<style lang="css" scoped>
</style>
