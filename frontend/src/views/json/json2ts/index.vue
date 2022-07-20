<template>
  <to-page title="Json转为Typescript">
    <t-form :data="formData" :label-width="140" colon label-align="right">
      <t-row :gutter="[24, 24]">
        <t-col :span="4">
          <t-form-item label="使用前缀 I">
            <t-radio-group v-model="formData.config.prependWithI" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :span="4">
          <t-form-item label="按字母顺序排序">
            <t-radio-group v-model="formData.config.sortAlphabetically" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :span="4">
          <t-form-item label="添加export">
            <t-radio-group v-model="formData.config.addExport" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :span="4">
          <t-form-item label="数组使用Array<T>">
            <t-radio-group v-model="formData.config.useArrayGeneric" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :span="4">
          <t-form-item label="字段为可选">
            <t-radio-group v-model="formData.config.optionalFields" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>

        <t-col :md="6" :xs="12">
          <t-form-item label="类前缀">
            <t-input v-model="formData.config.prefix"/>
          </t-form-item>
        </t-col>

        <t-col :md="6" :xs="12">
          <t-form-item label="根对象名">
            <t-input v-model="formData.config.rootObjectName"/>
          </t-form-item>
        </t-col>
      </t-row>
      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col :lg="6" :xs="12">
          <t-form-item :label-width="0">
            <t-textarea v-model="formData.source" autofocus placeholder="请输入json数据"/>
          </t-form-item>
        </t-col>
        <t-col :lg="6" :xs="12">
          <t-form-item :label-width="0">
            <t-textarea v-model="formData.result"/>
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>

    <template #footer>
      <div flex items-center>
        <span>灵感来源：</span>
        <t-button shape="circle" variant="text" @click="handleInspiration">
          <template #icon>
            <t-icon name="logo-github-filled"/>
          </template>
        </t-button>
      </div>
    </template>
    <template #extract>
      <t-button @click="convert">
        <template #icon>
          <precise-monitor-icon/>
        </template>
      </t-button>
    </template>
  </to-page>
</template>
<script lang="ts" setup>
import ToPage from '@/components/ToPage/index.vue'
import {IJson2TsConfig, Json2Ts} from './json2ts'
import {BrowserOpenURL} from '@/wailsjs/runtime'
import {MessagePlugin} from 'tdesign-vue-next'
import {PreciseMonitorIcon} from 'tdesign-icons-vue-next'

interface Json2TsProps {
  source: string
  result: string
  config: IJson2TsConfig
}

const formData: Json2TsProps = reactive({
  source: '',
  result: '',
  config: {
    prependWithI: true,
    addExport: false,
    sortAlphabetically: false,
    useArrayGeneric: false,
    optionalFields: false,
    prefix: '',
    rootObjectName: 'RootObject',
  },
})

const convert = () => {
  if (formData.source === '') {
    return
  }

  const json2ts = new Json2Ts(formData.config)
  try {
    const obj = JSON.parse(toRaw(formData.source))
    if (obj) {
      const result = json2ts.convert(obj)
      formData.result = result
    }
  } catch (error) {
    MessagePlugin.error({
      content: '请检查JSON格式是否正确',
      duration: 2000,
    })
  }
}

const handleInspiration = () => {
  BrowserOpenURL('https://github.com/beshanoe/json2ts/blob/master/src/utils/json2.ts')
}
</script>

<style lang="scss" scoped>
:deep(.t-textarea__inner) {
  height: 380px;
}
</style>
