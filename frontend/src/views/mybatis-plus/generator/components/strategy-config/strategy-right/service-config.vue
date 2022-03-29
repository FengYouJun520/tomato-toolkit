<template>
  <div>
    <t-form label-align="right" :label-width="180" colon>
      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col>
          <t-button theme="warning">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col>
          <t-form-item label="格式化service接口名称">
            <t-input default-value="%sService" />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="格式化service实现类名称">
            <t-input default-value="%sServiceImpl" />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="service父类接口">
            <t-input />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="service实现类父类接口">
            <t-input />
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script setup lang="ts"></script>

<style lang="scss" scoped></style>
