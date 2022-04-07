<template>
  <to-page title="json格式化">
    <t-form label-align="top" colon :data="jsonFormatModel">
      <t-row :gutter="[24, 24]">
        <t-col :xs="12" :md="6">
          <t-form-item label="格式化宽度">
            <t-select v-model="jsonFormatModel.space">
              <t-option label="2" :value="2" />
              <t-option label="4" :value="4" />
            </t-select>
          </t-form-item>
        </t-col>
      </t-row>
      <t-form-item label="json字符串">
        <t-textarea v-model="jsonFormatModel.value" :autosize="{ minRows: 20, maxRows: 50 }" />
      </t-form-item>
    </t-form>

    <template #extract>
      <t-button @click="formatJson">
        <template #icon>
          <precise-monitor-icon />
        </template>
      </t-button>
    </template>
  </to-page>
</template>
<script setup lang="ts">
import ToPage from '@/components/ToPage/index.vue'
import { PreciseMonitorIcon } from 'tdesign-icons-vue-next'

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
