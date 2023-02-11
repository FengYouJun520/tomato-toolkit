<script setup lang="ts">
import { useTemplateConfigStore } from '@/store/mp/templateconfig'
import { shell, path } from '@tauri-apps/api'

const message = useMessage()
const templateConfigStore = useTemplateConfigStore()

const handleReset = () => {
  templateConfigStore.$reset()
}

const handleOpenTempelate = async () => {
  try {
    const resourcePath = await path.resolveResource('templates')
    shell.open(resourcePath)
  } catch (error) {
    message.error(error as string)
  }
}
</script>

<template>
  <el-form
    label-position="top"
    :model="templateConfigStore.$state"
  >
    <el-form-item label-width="0">
      <el-space>
        <el-button type="warning" @click="handleReset">
          重置
        </el-button>
        <el-button type="primary" @click="handleOpenTempelate">
          打开模板文件目录
        </el-button>
      </el-space>
    </el-form-item>

    <el-row :gutter="24">
      <el-col :md="12">
        <el-form-item label="Entity模板">
          <el-input
            v-model="templateConfigStore.entity"
            placeholder="默认为: templates/entity.java"
          />
        </el-form-item>
      </el-col>
      <el-col :md="12">
        <el-form-item label="EntityKotlin模板">
          <el-input
            v-model="templateConfigStore.entityKt"
            placeholder="默认为: templates/entity.kt.java"
          />
        </el-form-item>
      </el-col>

      <el-col :md="12">
        <el-form-item label="Mapper模板">
          <el-input
            v-model="templateConfigStore.mapper"
            placeholder="默认为: templates/mapper.java"
          />
        </el-form-item>
      </el-col>
      <el-col :md="12">
        <el-form-item label="Xml模板">
          <el-input
            v-model="templateConfigStore.xml"
            placeholder="默认为: templates/mapper.xml"
          />
        </el-form-item>
      </el-col>

      <el-col :md="12">
        <el-form-item label="Service模板">
          <el-input
            v-model="templateConfigStore.service"
            placeholder="默认为: templates/service.java"
          />
        </el-form-item>
      </el-col>
      <el-col :md="12">
        <el-form-item label="ServiceImpl模板">
          <el-input
            v-model="templateConfigStore.serviceImpl"
            placeholder="默认为: templates/servicerImpl.java"
          />
        </el-form-item>
      </el-col>

      <el-col :md="12">
        <el-form-item label="Controller模板">
          <el-input
            v-model="templateConfigStore.controller"
            placeholder="默认为: templates/controller.java"
          />
        </el-form-item>
      </el-col>
    </el-row>
  </el-form>
</template>

<style lang="css" scoped>
</style>
