<template>
  <div>
    <t-form :data="strategyStore.strategy" :label-width="120" colon label-align="right">
      <t-row :gutter="[24, 24]" mt-6>
        <t-col>
          <t-button theme="warning" @click="strategyStore.clearStrategy">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" mt-6>
        <t-col :md="6" :xs="12">
          <t-form-item label="开启大写命名">
            <t-radio-group
              v-model="strategyStore.strategy.enableCapitalMode"
              disabled
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :md="6" :xs="12">
          <t-form-item label="开启跳过视图">
            <t-radio-group
              v-model="strategyStore.strategy.enableSkipView"
              disabled
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :md="6" :xs="12">
          <t-form-item label="禁用sql过滤">
            <t-radio-group
              v-model="strategyStore.strategy.disableSqlFilter"
              disabled
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :md="6" :xs="12">
          <t-form-item label="启用schema">
            <t-radio-group
              v-model="strategyStore.strategy.enableSchema"
              disabled
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>

        <t-col :span="12">
          <t-form-item label="要生成的表列表">
            <t-select v-model="strategyStore.strategy.addIncludes" clearable multiple>
              <t-option
                v-for="option in basicStore.tablesOptions"
                :key="option.value"
                :label="option.label"
                :value="option.value"
              >
                <t-row :gutter="24">
                  <t-col :span="4">{{ option.value }}</t-col>
                  <t-col :span="8">
                    <t-tag v-if="option.comment" theme="success" variant="light">
                      {{ option.comment }}
                    </t-tag>
                  </t-col>
                </t-row>
              </t-option>
            </t-select>
          </t-form-item>
        </t-col>
      </t-row>
      <t-row class="mt-6">
        <t-col>
          <t-button :disabled="basicStore.tableIsEmpty" @click="selectAll"> 全选</t-button>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script lang="ts" setup>
import {useBasic} from '@/store/modules/mybatis/useBasic'
import {useStrategy} from '@/store/modules/mybatis/useStrategy'

const basicStore = useBasic()
const strategyStore = useStrategy()

const selectAll = () => {
  strategyStore.setAddIncludes()
}
</script>

<style lang="scss" scoped></style>
