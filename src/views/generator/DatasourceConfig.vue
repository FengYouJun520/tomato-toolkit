<script setup lang="ts">
import { NTag, SelectOption, SelectRenderLabel, SelectRenderTag } from 'naive-ui'
import mysqlIcon from '@/assets/mysql.svg'
import sqliteIcon from '@/assets/sqlite.svg'
import mssqlIcon from '@/assets/mssql.svg'
import postgresIcon from '@/assets/postgres.svg'
import userIcon from '@/assets/user.png'
import lockIcon from '@/assets/lock.png'
import { invoke } from '@tauri-apps/api'
import { DatasourceConfig } from '@/types/type'
import rocketIcon from '@/assets/rocket.svg'
import { useDatesource } from '@/store/modules/mp/datasource'

const message = useMessage()
const datasourceConfig = useDatesource()

const options: SelectOption[] = [
  {
    label: 'mysql',
    value: 'mysql',
  },
  {
    label: 'sqlite',
    value: 'sqlite',
  },
  {
    label: 'mssql',
    value: 'mssql',
  },
  {
    label: 'postgres',
    value: 'postgres',
  },
]

const getIcon = (value: string) => {
  switch (value) {
  case 'mysql':
    return mysqlIcon
  case 'sqlite':
    return sqliteIcon
  case 'mssql':
    return mssqlIcon
  case 'postgres':
    return postgresIcon
  default:
    return mysqlIcon
  }
}

const renderTag: SelectRenderTag = ({ option }) => h(
  'div',
  { class: 'flex items-center space-x-2' },
  { default: () => [
    h('img', { src: getIcon(option.value as string) }),
    h(
      NTag,
      { type: 'info', bordered: false },
      { default: () => option.label }
    ),
  ] }
)

const renderLabel: SelectRenderLabel = option => h(
  'div',
  { class: 'flex items-center space-x-2' },
  { default: () => [
    h('img', { src: getIcon(option.value as string) }),
    h(
      NTag,
      { type: 'info', bordered: false },
      { default: () => option.label }
    ),
  ] }
)

const testLoading = ref(false)
const handleTestConnection = async () => {
  testLoading.value = true
  try {
    await invoke('test_connection', { config: datasourceConfig.$state })
    message.success('测试连接成功')
  } catch (error) {
    message.error(error as string)
  }
  testLoading.value = false
}
</script>

<template>
  <n-form-item label="数据库类型">
    <n-select
      v-model:value="datasourceConfig.type"
      :options="options"
      :render-tag="renderTag"
      :render-label="renderLabel"
    />
  </n-form-item>
  <n-form-item label="数据库名称">
    <n-input v-model:value="datasourceConfig.database" />
  </n-form-item>
  <n-form-item label="Host">
    <n-input v-model:value="datasourceConfig.host" />
  </n-form-item>
  <n-form-item label="Port">
    <n-input-number v-model:value="datasourceConfig.port" :min="0" :max="65565" />
  </n-form-item>
  <n-form-item label="用户名">
    <n-input v-model:value="datasourceConfig.username">
      <template #prefix>
        <img :src="userIcon" alt="user">
      </template>
    </n-input>
  </n-form-item>
  <n-form-item label="密码">
    <n-input v-model:value="datasourceConfig.password" type="password">
      <template #prefix>
        <img :src="lockIcon" alt="lock">
      </template>
    </n-input>
  </n-form-item>
  <div class="flex justify-center items-center space-x-2">
    <n-button
      type="primary"
      :loading="testLoading"
      @click="handleTestConnection"
    >
      <template #icon>
        <img :src="rocketIcon" alt="rocket">
      </template>
      测试
    </n-button>
    <n-button type="error">
      重置
    </n-button>
  </div>
</template>

<style lang="css" scoped>
</style>
