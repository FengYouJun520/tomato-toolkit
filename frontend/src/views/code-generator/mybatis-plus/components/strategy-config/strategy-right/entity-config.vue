<template>
  <div>
    <t-form :data="strategyStore.strategy.entity" :label-width="160" colon label-align="right">
      <t-row :gutter="[24, 24]" mt-6>
        <t-col>
          <t-button theme="warning" @click="strategyStore.clearStrategy">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" mt-6>
        <t-col :md="6" :xs="12">
          <t-form-item label="禁用SerialVersionUID">
            <t-radio-group
              v-model="strategyStore.strategy.entity.disableSerialVersionUID"
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :md="6" :xs="12">
          <t-form-item label="使用链式模式">
            <t-radio-group
              v-model="strategyStore.strategy.entity.enableChainModel"
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :md="6" :xs="12">
          <t-form-item label="使用lombok">
            <t-radio-group
              v-model="strategyStore.strategy.entity.enableLombok"
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :md="6" :xs="12">
          <t-form-item label="使用record">
            <t-radio-group
              v-model="strategyStore.strategy.entity.enableActiveRecord"
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :md="6" :xs="12">
          <t-form-item label="使用静态常量字段">
            <t-radio-group
              v-model="strategyStore.strategy.entity.enableColumnConstant"
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :md="6" :xs="12">
          <t-form-item label="移除Boolean类型is前缀">
            <t-radio-group
              v-model="strategyStore.strategy.entity.enableRemoveIsPrefix"
              variant="primary-filled"
            >
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col>
          <t-form-item label="父类包名">
            <t-input v-model="strategyStore.strategy.entity.superClass"/>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="乐观锁属性名">
            <t-input v-model="strategyStore.strategy.entity.versionPropertyName"/>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="乐观锁字段名">
            <t-input v-model="strategyStore.strategy.entity.versionColumnName"/>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="逻辑删除属性名">
            <t-input v-model="strategyStore.strategy.entity.logicDeletePropertyName"/>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="逻辑删除字段名">
            <t-input v-model="strategyStore.strategy.entity.logicDeleteColumnName"/>
          </t-form-item>
        </t-col>

        <t-col>
          <t-form-item label="实体命名策略">
            <t-select v-model="strategyStore.strategy.entity.naming" :options="namingOptions"/>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="添加忽略字段">
            <t-tag-input
              v-model="strategyStore.strategy.entity.addIgnoreColumns"
              clearable
              excess-tags-display-type="break-line"
            />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="自动填充字段">
            <div v-if="strategyStore.addTableFillsIsExist" class="flex flex-col gap-6">
              <t-input-group
                v-for="fillField in strategyStore.strategy.entity.addTableFills"
                :key="fillField.key"
                separate
              >
                <t-row>
                  <t-col :span="4">
                    <t-input v-model="fillField.name" style="width: 140px"/>
                  </t-col>
                  <t-col :span="1" class="text-center">
                    <span class="text-center align-middle">-</span>
                  </t-col>
                  <t-col :span="5">
                    <t-select
                      v-model="fillField.value"
                      :options="fillOptions"
                      style="width: 160px"
                    />
                  </t-col>
                  <t-col :span="2">
                    <div class="flex gap-1 ml-3">
                      <t-button @click="strategyStore.addFillField">+</t-button>
                      <t-button @click="strategyStore.removeFillField(fillField)">-</t-button>
                    </div>
                  </t-col>
                </t-row>
              </t-input-group>
            </div>

            <t-button v-else block @click="strategyStore.addFillField">+</t-button>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="全局主键类型">
            <t-select v-model="strategyStore.strategy.entity.idType" :options="assignIds"/>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="类文件格式">
            <t-input v-model="strategyStore.strategy.entity.formatFileName"/>
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script lang="ts" setup>
import {useStrategy} from '@/store/modules/mybatis/useStrategy'
import {assignIds, fillOptions} from './data'

const strategyStore = useStrategy()

const namingOptions = [
  {
    label: '下划线转驼峰',
    value: 'underline_to_camel',
  },
]
</script>

<style lang="scss" scoped></style>
