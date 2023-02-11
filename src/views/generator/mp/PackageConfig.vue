<script setup lang="ts">
import { usePackageConfigStore } from '@/store/mp/packageconfig'

const packageConfigStore = usePackageConfigStore()

const handleReset = () => {
  packageConfigStore.$reset()
}
</script>

<template>
  <el-form
    label-position="top"
    :model="packageConfigStore.$state"
  >
    <el-form-item label-width="0">
      <el-button type="warning" @click="handleReset">
        重置
      </el-button>
    </el-form-item>

    <el-row :gutter="24">
      <el-col :sm="12">
        <el-form-item label="父包名">
          <el-input v-model="packageConfigStore.parent" />
        </el-form-item>
      </el-col>
      <el-col :sm="12">
        <el-form-item label="父模块名">
          <el-input v-model="packageConfigStore.moduleName" />
        </el-form-item>
      </el-col>

      <el-col :sm="12">
        <el-form-item label="Entity包名">
          <el-input v-model="packageConfigStore.entity" />
        </el-form-item>
      </el-col>
      <el-col :sm="12">
        <el-form-item label="Controller名">
          <el-input v-model="packageConfigStore.controller" />
        </el-form-item>
      </el-col>

      <el-col :sm="12">
        <el-form-item label="Service包名">
          <el-input v-model="packageConfigStore.service" />
        </el-form-item>
      </el-col>
      <el-col :sm="12">
        <el-form-item label="ServiceImpl名">
          <el-input v-model="packageConfigStore.serviceImpl" />
        </el-form-item>
      </el-col>

      <el-col :sm="12">
        <el-form-item label="Mapper包名">
          <el-input v-model="packageConfigStore.mapper" />
        </el-form-item>
      </el-col>
      <el-col :sm="12">
        <el-form-item label="MapperXml包名">
          <el-input v-model="packageConfigStore.serviceImpl" />
        </el-form-item>
      </el-col>
    </el-row>
  </el-form>
</template>

<style lang="css" scoped>
</style>
