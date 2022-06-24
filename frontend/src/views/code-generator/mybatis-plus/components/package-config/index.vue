<template>
  <div>
    <t-form label-align="right" :label-width="140" colon :data="packageStore.package">
      <t-row :gutter="[24, 24]" mt-6>
        <t-col>
          <t-button theme="warning" @click="packageStore.clearPackage">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" mt-6>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="父包名">
            <t-input v-model="packageStore.package.parent" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="父模块名">
            <t-input v-model="packageStore.package.moduleName" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="Entity包名">
            <t-input v-model="packageStore.package.entity" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="Service 包名">
            <t-input v-model="packageStore.package.service" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="ServiceImpl包名">
            <t-input v-model="packageStore.package.serviceImpl" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="Mapper包名">
            <t-input v-model="packageStore.package.mapper" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="MapperXML包名">
            <t-input v-model="packageStore.package.mapperXml" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="Controller包名">
            <t-input v-model="packageStore.package.controller" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="自定义文件包名">
            <t-input disabled v-model="packageStore.package.other" />
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script setup lang="ts">
import { usePackage } from '@/store/modules/mybatis/usePackage'

const packageStore = usePackage()
</script>

<style lang="scss" scoped></style>
