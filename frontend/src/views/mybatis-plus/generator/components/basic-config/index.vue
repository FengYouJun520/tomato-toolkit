<template>
  <t-form label-align="right" colon>
    <t-row :gutter="[24, 24]">
      <t-col :xs="12" :lg="6">
        <t-form-item label="数据库类型">
          <t-select>
            <t-option v-for="option in databaseOptions" :key="option.value" :value="option.value">
              <div class="flex items-center gap-6">
                <component :is="option.icon" class="w-6 h-6" />
                {{ option.label }}
              </div>
            </t-option>
            <template #valueDisplay="{ value, label, icon }">
              <t-tag>
                {{ value }}
              </t-tag>
            </template>
          </t-select>
        </t-form-item>
      </t-col>
      <t-col :xs="12" :lg="6">
        <t-form-item label="数据库名称">
          <t-input />
        </t-form-item>
      </t-col>
      <t-col :xs="12" :lg="6">
        <t-form-item label="Host">
          <t-input />
        </t-form-item>
      </t-col>
      <t-col :xs="12" :lg="6">
        <t-form-item label="Port">
          <t-input />
        </t-form-item>
      </t-col>
      <t-col :xs="12" :lg="6">
        <t-form-item label="用户名">
          <t-input>
            <template #prefix-icon>
              <t-icon name="user" />
            </template>
          </t-input>
        </t-form-item>
      </t-col>
      <t-col :xs="12" :lg="6">
        <t-form-item label="密码">
          <t-input type="password">
            <template #prefix-icon>
              <t-icon name="lock-on" />
            </template>
          </t-input>
        </t-form-item>
      </t-col>
    </t-row>
    <t-row :gutter="[24, 24]" class="mt-6">
      <t-col :span="12" class="flex justify-center gap-3">
        <t-button>测试</t-button>
        <t-button theme="warning">重置</t-button>
      </t-col>
    </t-row>
  </t-form>
</template>
<script setup lang="ts">
import Mysql from '@/assets/mysql.svg'
import Postgresql from '@/assets/postgresql.svg'
import Sqlite from '@/assets/sqlite.svg'
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
  },
  {
    label: 'postgresql',
    value: 'postgresql',
    icon: Postgresql,
  },
]
</script>

<style lang="scss" scoped></style>
