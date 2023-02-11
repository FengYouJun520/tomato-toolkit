<script setup lang="ts">
import { NTag, SelectOption, SelectRenderLabel, SelectRenderTag } from 'naive-ui'
import mysqlIcon from '@/assets/mysql.svg'
import sqliteIcon from '@/assets/sqlite.svg'
import mssqlIcon from '@/assets/mssql.svg'
import postgresIcon from '@/assets/postgres.svg'
import userIcon from '@/assets/user.png'
import lockIcon from '@/assets/lock.png'
import { invoke } from '@tauri-apps/api'
import rocketIcon from '@/assets/rocket.svg'
import { useDatasourceStore } from '@/store/mp/datasource'
import { useBasicTableInfosStore } from '@/store/mp/basicTable'
import { BasicTableInfo } from '@/types/type'
import { ElMessage } from 'element-plus'

const basicTableInfos = useBasicTableInfosStore()
const message = useMessage()
const datasourceConfigStore = useDatasourceStore()

const options: SelectOption[] = [
  {
    label: 'mysql',
    value: 'mysql',
  },
  {
    label: 'sqlite（大致做完了，但不一定正确）',
    value: 'sqlite',
  },
  {
    label: 'sqlserver',
    value: 'sqlserver',
    disabled: true,
  },
  {
    label: 'postgressql',
    value: 'postgressql',
    disabled: true,
  },
]

const getIcon = (value: string) => {
  switch (value) {
  case 'mysql':
    return mysqlIcon
  case 'sqlite':
    return sqliteIcon
  case 'sqlserver':
    return mssqlIcon
  case 'postgressql':
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
      { type: 'info', bordered: false, disabled: !!option.disabled },
      { default: () => option.label }
    ),
  ] }
)

const testLoading = ref(false)
const handleTestConnection = async () => {
  testLoading.value = true
  try {
    const tables = await invoke<BasicTableInfo[]>('test_connection', { config: datasourceConfigStore.$state })
    basicTableInfos.setBasicTables(tables)
    ElMessage.success('测试连接成功，请到策略配置选择要生成的表')
  } catch (error) {
    ElMessage.error(error as string)
  }
  testLoading.value = false
}

const handleReset = () => {
  datasourceConfigStore.$reset()
}
</script>

<template>
  <n-form
    label-placement="left"
    :label-width="100"
    label-align="left"
  >
    <n-form-item label="数据库类型">
      <n-select
        v-model:value="datasourceConfigStore.type"
        :options="options"
        :render-tag="renderTag"
        :render-label="renderLabel"
      />
    </n-form-item>
    <n-form-item label="数据库名称">
      <n-input v-model:value="datasourceConfigStore.database" />
    </n-form-item>
    <n-form-item label="Host">
      <n-input v-model:value="datasourceConfigStore.host" />
    </n-form-item>
    <n-form-item label="Port">
      <n-input-number v-model:value="datasourceConfigStore.port" :min="0" :max="65565" />
    </n-form-item>
    <n-form-item label="用户名">
      <n-input v-model:value="datasourceConfigStore.username">
        <template #prefix>
          <img :src="userIcon" alt="user">
        </template>
      </n-input>
    </n-form-item>
    <n-form-item label="密码">
      <n-input
        v-model:value="datasourceConfigStore.password"
        type="password"
        show-password-on="click"
      >
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
      <n-button type="warning" @click="handleReset">
        重置
      </n-button>
    </div>
  </n-form>
</template>

<style lang="css" scoped>
</style>
