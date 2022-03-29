<template>
  <div>
    <t-form label-align="right" :label-width="140" colon>
      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col>
          <t-button theme="warning">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="父包名">
            <t-input default-value="com.baomidou" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="父模块名">
            <t-input />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="Entity包名">
            <t-input default-value="entity" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="Service 包名">
            <t-input default-value="service" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="ServiceImpl包名">
            <t-input default-value="service.impl" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="Mapper包名">
            <t-input default-value="mapper" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="MapperXML包名">
            <t-input default-value="mapper.xml" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="Controller包名">
            <t-input default-value="controller" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="自定义文件包名">
            <t-input disabled />
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script setup lang="ts"></script>

<style lang="scss" scoped></style>
