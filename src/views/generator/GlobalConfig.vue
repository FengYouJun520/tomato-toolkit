<script setup lang="ts">
import { useGlobalConfigStore } from '@/store/modules/mp/globalconfig'


const globalConfigStore = useGlobalConfigStore()

const handleReset = () => {
  globalConfigStore.$reset()
}
</script>

<template>
  <n-form label-placement="left" :label-width="120" label-align="right">
    <n-form-item>
      <n-button type="warning" @click="handleReset">
        重置
      </n-button>
    </n-form-item>

    <n-form-item label="作者">
      <n-input v-model:value="globalConfigStore.author" />
    </n-form-item>

    <n-grid cols="1 m:2" :x-gap="24" responsive="screen">
      <n-form-item-gi label="打开输出目录">
        <n-radio-group v-model:value="globalConfigStore.open">
          <n-radio-button :value="true">
            开启
          </n-radio-button>
          <n-radio-button :value="false">
            关闭
          </n-radio-button>
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="覆盖已生成文件">
        <n-radio-group v-model:value="globalConfigStore.fileOverride">
          <n-radio-button :value="true">
            开启
          </n-radio-button>
          <n-radio-button :value="false">
            关闭
          </n-radio-button>
        </n-radio-group>
      </n-form-item-gi>

      <n-form-item-gi label="开启swagger">
        <n-radio-group v-model:value="globalConfigStore.swagger" :disabled="globalConfigStore.springdoc">
          <n-radio-button :value="true">
            开启
          </n-radio-button>
          <n-radio-button :value="false">
            关闭
          </n-radio-button>
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="开启springdoc">
        <n-radio-group v-model:value="globalConfigStore.springdoc">
          <n-radio-button :value="true">
            开启
          </n-radio-button>
          <n-radio-button :value="false">
            关闭
          </n-radio-button>
        </n-radio-group>
      </n-form-item-gi>

      <n-form-item-gi label="开启kotlin模式">
        <n-radio-group v-model:value="globalConfigStore.kotlin">
          <n-radio-button :value="true">
            开启
          </n-radio-button>
          <n-radio-button :value="false">
            关闭
          </n-radio-button>
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="时间策略">
        <n-select v-model:value="globalConfigStore.dateType" :options="[
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
    </n-grid>

    <n-form-item label="注释日期格式">
      <n-input v-model:value="globalConfigStore.commentDate" />
    </n-form-item>
    <n-form-item label="输出目录">
      <n-input v-model:value="globalConfigStore.outputDir" />
    </n-form-item>
  </n-form>
</template>

<style lang="css" scoped>
</style>
