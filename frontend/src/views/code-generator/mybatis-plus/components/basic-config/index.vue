<template>
  <t-form :data="basicStore.dataSource" colon label-align="right">
    <t-row :gutter="[24, 24]">
      <t-col :lg="6" :xs="12">
        <t-form-item label="数据库类型">
          <t-select v-model="basicStore.dataSource.typ" clearable>
            <t-option
              v-for="option in databaseOptions"
              :key="option.value"
              :disabled="option.disabled"
              :value="option.value"
            >
              <div flex gap-6 items-center>
                <component :is="option.icon" h-6 w-6/>
                {{ option.label }}
              </div>
            </t-option>
            <template #valueDisplay="{ value }">
              <div flex gap-3 items-center>
                <component
                  :is="databaseOptions.find((database) => database.value === value)?.icon"
                  h-6
                  w-6
                />
                <t-tag theme="success" variant="light"> {{ value }}</t-tag>
              </div>
            </template>
          </t-select>
        </t-form-item>
      </t-col>
      <t-col :lg="6" :xs="12">
        <t-form-item label="数据库名称">
          <t-input v-model="basicStore.dataSource.database"/>
        </t-form-item>
      </t-col>
      <t-col :lg="6" :xs="12">
        <t-form-item label="Host">
          <t-input v-model="basicStore.dataSource.host"/>
        </t-form-item>
      </t-col>
      <t-col :lg="6" :xs="12">
        <t-form-item label="Port">
          <t-input v-model="basicStore.dataSource.port"/>
        </t-form-item>
      </t-col>
      <t-col :lg="6" :xs="12">
        <t-form-item label="用户名">
          <t-input v-model="basicStore.dataSource.username">
            <template #prefix-icon>
              <t-icon name="user"/>
            </template>
          </t-input>
        </t-form-item>
      </t-col>
      <t-col :lg="6" :xs="12">
        <t-form-item label="密码">
          <t-input v-model="basicStore.dataSource.password" type="password">
            <template #prefix-icon>
              <t-icon name="lock-on"/>
            </template>
          </t-input>
        </t-form-item>
      </t-col>
    </t-row>
    <t-row :gutter="[24, 24]" mt-6>
      <t-col :span="12" class="flex justify-center gap-3">
        <t-button @click="testConnection">测试</t-button>
        <t-button theme="warning" @click="basicStore.clearBasic">重置</t-button>
      </t-col>
    </t-row>
  </t-form>
</template>
<script lang="ts" setup>
import Mysql from '@/assets/mysql.svg'
import Postgresql from '@/assets/postgresql.svg'
import Sqlite from '@/assets/sqlite.svg'
import {useBasic} from '@/store/modules/mybatis/useBasic'
import {useStrategy} from '@/store/modules/mybatis/useStrategy'
import {DatabaseOptions} from '@/types/mybatis-plus'
import {PingDb} from '@/wailsjs/go/main/App'
import {MessagePlugin} from 'tdesign-vue-next'

const databaseOptions = [
  {
    label: 'mysql',
    value: 'mysql',
    icon: Mysql,
  },
  {
    label: 'sqlite',
    value: 'sqlite',
    icon: Sqlite,
    disabled: true,
  },
  {
    label: 'postgres',
    value: 'postgres',
    icon: Postgresql,
    disabled: true,
  },
]

const basicStore = useBasic()
const strategy = useStrategy()

const testConnection = () => {
  PingDb(basicStore.dataSource)
    .then((options: DatabaseOptions[] | Error) => {
      basicStore.setOptions(options as DatabaseOptions[])
      MessagePlugin.success('连接成功')
      basicStore.executeDisable = false
    })
    .catch((err: string) => {
      strategy.clearAddIncludes()
      basicStore.clearOptions()
      MessagePlugin.error(err)
    })
}
</script>

<style lang="scss" scoped></style>
