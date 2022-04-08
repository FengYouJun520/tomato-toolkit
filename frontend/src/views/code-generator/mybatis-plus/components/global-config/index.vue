<template>
  <div>
    <t-form label-align="right" :label-width="140" colon :data="globalStore.global">
      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col>
          <t-button theme="warning" @click="globalStore.clearGlobal">重置</t-button>
        </t-col>
      </t-row>
      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col :span="12">
          <t-form-item label="作者名称">
            <t-input v-model="globalStore.global.author" />
          </t-form-item>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="覆盖已生成文件">
            <t-radio-group v-model="globalStore.global.fileOverride" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="禁止打开输出目录">
            <t-radio-group v-model="globalStore.global.disableOpenDir" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="开启kotlin模式">
            <t-radio-group
              disabled
              v-model="globalStore.global.enableKotlin"
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="开启swagger">
            <t-radio-group v-model="globalStore.global.enableSwagger" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="时间策略">
            <t-select v-model="globalStore.global.dateType" :options="timeStrategyOptions" />
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6" :lg="4">
          <t-form-item label="注释日期">
            <t-input disabled v-model="globalStore.global.commentDate" />
          </t-form-item>
        </t-col>
      </t-row>
      <t-row class="mt-6">
        <t-col>
          <t-form-item label="指定输出目录">
            <t-input v-model="globalStore.global.outputDir" />
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script setup lang="ts">
import { useGlobal } from '@/store/modules/mybatis/useGlobal'

const timeStrategyOptions = [
  {
    label: 'ONLY_TIME',
    value: 'ONLY_TIME',
    disabled: true,
  },
  {
    label: 'TIME_PACK',
    value: 'TIME_PACK',
  },
]

const globalStore = useGlobal()
</script>

<style lang="scss" scoped></style>
