<template>
  <div>
    <t-form :data="packageStore.package" :label-width="140" colon label-align="right">
      <t-row :gutter="[24, 24]" mt-6>
        <t-col>
          <t-button theme="warning" @click="packageStore.clearPackage">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" mt-6>
        <t-col :lg="4" :md="6" :xs="12">
          <t-form-item label="父包名">
            <t-input v-model="packageStore.package.parent"/>
          </t-form-item>
        </t-col>
        <t-col :lg="4" :md="6" :xs="12">
          <t-form-item label="父模块名">
            <t-input v-model="packageStore.package.moduleName"/>
          </t-form-item>
        </t-col>
        <t-col :lg="4" :md="6" :xs="12">
          <t-form-item label="Entity包名">
            <t-input v-model="packageStore.package.entity"/>
          </t-form-item>
        </t-col>
        <t-col :lg="4" :md="6" :xs="12">
          <t-form-item label="Service 包名">
            <t-input v-model="packageStore.package.service"/>
          </t-form-item>
        </t-col>
        <t-col :lg="4" :md="6" :xs="12">
          <t-form-item label="ServiceImpl包名">
            <t-input v-model="packageStore.package.serviceImpl"/>
          </t-form-item>
        </t-col>
        <t-col :lg="4" :md="6" :xs="12">
          <t-form-item label="Mapper包名">
            <t-input v-model="packageStore.package.mapper"/>
          </t-form-item>
        </t-col>
        <t-col :lg="4" :md="6" :xs="12">
          <t-form-item label="MapperXML包名">
            <t-input v-model="packageStore.package.mapperXml"/>
          </t-form-item>
        </t-col>
        <t-col :lg="4" :md="6" :xs="12">
          <t-form-item label="Controller包名">
            <t-input v-model="packageStore.package.controller"/>
          </t-form-item>
        </t-col>
        <t-col :lg="4" :md="6" :xs="12">
          <t-form-item label="自定义文件包名">
            <t-input v-model="packageStore.package.other" disabled/>
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script lang="ts" setup>
import {usePackage} from '@/store/modules/mybatis/usePackage'

const packageStore = usePackage()
</script>

<style lang="scss" scoped></style>
