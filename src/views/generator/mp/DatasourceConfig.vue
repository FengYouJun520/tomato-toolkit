<script setup lang="ts">
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

const basicTableInfos = useBasicTableInfosStore()
const datasourceConfigStore = useDatasourceStore()

const options = [
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
  <el-form
    :label-width="120"
    :model="datasourceConfigStore.$state"
  >
    <el-form-item label="数据库类型">
      <el-select
        v-model="datasourceConfigStore.type"
        class="w-full"
      >
        <el-option
          v-for="option in options"
          :key="option.value"
          :label="option.value"
          :value="option.value"
          :disabled="option.disabled"
        >
          <div class="flex items-center space-x-5">
            <img :src="getIcon(option.value)">
            <el-tag type="success">
              {{ option.label }}
            </el-tag>
          </div>
        </el-option>
      </el-select>
    </el-form-item>
    <el-form-item label="数据库名称">
      <el-input v-model="datasourceConfigStore.database" />
    </el-form-item>
    <el-form-item label="Host">
      <el-input v-model="datasourceConfigStore.host" />
    </el-form-item>
    <el-form-item label="Port">
      <el-input-number v-model="datasourceConfigStore.port" :min="0" :max="65565" />
    </el-form-item>
    <el-form-item label="用户名">
      <el-input v-model="datasourceConfigStore.username">
        <template #prefix>
          <img :src="userIcon" alt="user">
        </template>
      </el-input>
    </el-form-item>
    <el-form-item label="密码">
      <el-input
        v-model="datasourceConfigStore.password"
        type="password"
        show-password-on="click"
      >
        <template #prefix>
          <img :src="lockIcon" alt="lock">
        </template>
      </el-input>
    </el-form-item>
    <div class="flex justify-center items-center space-x-2">
      <el-button
        type="primary"
        :loading="testLoading"
        @click="handleTestConnection"
      >
        <template #icon>
          <img :src="rocketIcon" alt="rocket">
        </template>
        测试
      </el-button>
      <el-button type="warning" @click="handleReset">
        重置
      </el-button>
    </div>
  </el-form>
</template>

<style lang="css" scoped>
</style>
