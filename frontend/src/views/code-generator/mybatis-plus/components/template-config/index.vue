<template>
  <div>
    <t-form :label-width="140" colon label-align="right">
      <t-row :gutter="[24, 24]" mt-6>
        <t-col>
          <t-button theme="warning" @click="templateStore.clearTemplate">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" mt-6>
        <t-col :lg="4" :md="6" :xs="12">
          <t-form-item label="禁用所有模板">
            <t-radio-group :default-value="false" disabled variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :lg="4" :md="6" :xs="12">
          <t-form-item label="禁用模板">
            <t-select :options="disableTemplateOptions" disabled/>
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script lang="ts" setup>
import {useTemplate} from '@/store/modules/mybatis/useTemplate'

const disableTemplateOptions = [
  {
    label: 'entity',
    value: 'ENTITY',
    disabled: false,
  },
  {
    label: 'service',
    value: 'SERVICE',
  },
  {
    label: 'service-impl',
    value: 'SERVICEIMPL',
  },
  {
    label: 'controller',
    value: 'CONTROLLER',
  },
  {
    label: 'mapper',
    value: 'MAPPER',
  },
  {
    label: 'mapper-xml',
    value: 'XML',
  },
]

const templateStore = useTemplate()
</script>

<style lang="scss" scoped></style>
