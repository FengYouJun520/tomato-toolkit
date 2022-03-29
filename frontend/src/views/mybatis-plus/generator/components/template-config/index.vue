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
          <t-form-item label="禁用所有模板">
            <t-radio-group disabled variant="primary-filled" :default-value="false">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="禁用模板">
            <t-select disabled></t-select>
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script setup lang="ts"></script>

<style lang="scss" scoped></style>
