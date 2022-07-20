<template>
  <to-page title="json格式化">
    <t-form :data="jsonFormatModel" colon label-align="top">
      <t-row :gutter="[24, 24]">
        <t-col :md="6" :xs="12">
          <t-form-item label="格式化宽度">
            <t-select v-model="jsonFormatModel.space">
              <t-option :value="2" label="2"/>
              <t-option :value="4" label="4"/>
            </t-select>
          </t-form-item>
        </t-col>
      </t-row>
      <t-form-item label="json字符串">
        <t-textarea v-model="jsonFormatModel.value" :autosize="{ minRows: 20, maxRows: 50 }"/>
      </t-form-item>
    </t-form>

    <template #extract>
      <t-button @click="formatJson">
        <template #icon>
          <precise-monitor-icon/>
        </template>
      </t-button>
    </template>
  </to-page>
</template>
<script lang="ts" setup>
import ToPage from '@/components/ToPage/index.vue'
import {PreciseMonitorIcon} from 'tdesign-icons-vue-next'

const jsonFormatModel = reactive({
  space: 2,
  value: '',
})

const formatJson = () => {
  if (!jsonFormatModel.value) {
    return
  }

  const parseJson = JSON.parse(jsonFormatModel.value)
  jsonFormatModel.value = JSON.stringify(parseJson, null, jsonFormatModel.space)
}
</script>

<style lang="scss" scoped></style>
