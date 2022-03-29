<template>
  <div>
    <t-form label-align="right" :label-width="120" colon>
      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col>
          <t-button theme="warning">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col :xs="12" :md="6">
          <t-form-item label="开启大写命名">
            <t-radio-group disabled :default-value="false" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6">
          <t-form-item label="开启跳过视图">
            <t-radio-group disabled :default-value="false" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6">
          <t-form-item label="禁用sql过滤">
            <t-radio-group disabled :default-value="true" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6">
          <t-form-item label="启用schema">
            <t-radio-group disabled :default-value="false" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>

        <t-col :span="12">
          <t-form-item label="要生成的表列表">
            <t-select multiple clearable>
              <t-option
                v-for="option in tableList"
                :key="option.value"
                :label="option.label"
                :value="option.value"
              >
                <t-row :gutter="24">
                  <t-col :span="4">{{ option.value }}</t-col>
                  <t-col :span="8">{{ option.label }}</t-col>
                </t-row>
              </t-option>
            </t-select>
          </t-form-item>
        </t-col>
      </t-row>
      <t-row class="mt-6">
        <t-col>
          <t-button>全选</t-button>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script setup lang="ts">
const tableList = [
  {
    label: '文章表',
    value: 'sys_article',
    comment: '文章表',
  },
  {
    label: '分类表',
    value: 'sys_category',
    comment: '分类表',
  },
  {
    label: '标签表',
    value: 'sys_tag',
    comment: '标签表',
  },
  {
    label: '用户表',
    value: 'sys_user',
    comment: '用户表',
  },
]
</script>

<style lang="scss" scoped></style>
