<script setup lang="ts">
import { useGlobalConfigStore } from '@/store/modules/mp/globalconfig'


const globalConfig = useGlobalConfigStore()
</script>

<template>
  <n-form label-placement="left" :label-width="120" label-align="right">
    <n-form-item label="作者">
      <n-input v-model:value="globalConfig.author" />
    </n-form-item>
    <n-grid :cols="2">
      <n-form-item-gi label="打开输出目录">
        <n-radio-group v-model:value="globalConfig.open">
          <n-radio-button :value="true">
            开启
          </n-radio-button>
          <n-radio-button :value="false">
            关闭
          </n-radio-button>
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="开启kotlin模式">
        <n-radio-group v-model:value="globalConfig.kotlin">
          <n-radio-button :value="true">
            开启
          </n-radio-button>
          <n-radio-button :value="false">
            关闭
          </n-radio-button>
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="开启swagger">
        <n-radio-group v-model:value="globalConfig.swagger" :disabled="globalConfig.springdoc">
          <n-radio-button :value="true">
            开启
          </n-radio-button>
          <n-radio-button :value="false">
            关闭
          </n-radio-button>
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="开启springdoc">
        <n-radio-group v-model:value="globalConfig.springdoc">
          <n-radio-button :value="true">
            开启
          </n-radio-button>
          <n-radio-button :value="false">
            关闭
          </n-radio-button>
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="时间策略">
        <n-select v-model:value="globalConfig.dateType" :options="[
          {
            label: 'ONLY_DATE',
            value: 'ONLY_DATE',
          },
          {
            label: 'SQL_PACK',
            value: 'SQL_PACK',
          },
          {
            label: 'TIME_PACK',
            value: 'TIME_PACK',
          },
        ]" />
      </n-form-item-gi>
      <n-form-item-gi label="注释日期格式">
        <n-input v-model:value="globalConfig.commentDate" />
      </n-form-item-gi>
    </n-grid>

    <n-form-item label="输出目录">
      <n-input v-model:value="globalConfig.outputDir" />
    </n-form-item>
  </n-form>
</template>

<style lang="css" scoped>
</style>
