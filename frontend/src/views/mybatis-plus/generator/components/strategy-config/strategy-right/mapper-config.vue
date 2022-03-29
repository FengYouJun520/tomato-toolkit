<template>
  <div>
    <t-form label-align="right" :label-width="160" colon>
      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col>
          <t-button theme="warning">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col>
          <t-form-item label="开启@Mapper注解">
            <t-radio-group :default-value="false" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="设置通用父类">
            <t-input />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="格式化mapper名称">
            <t-input default-value="%sMapper" />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="格式化xml实现类名称">
            <t-input default-value="%sMapper" />
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script setup lang="ts"></script>

<style lang="scss" scoped></style>
