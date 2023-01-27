<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/modules/mp/strategyconfig'
import { NTag, SelectRenderTag } from 'naive-ui'


const strategyconfigStore = useStrategyConfigStore()

const handleReset = () => {
  strategyconfigStore.resetEntity()
}

const renderTag: SelectRenderTag = ({ option, handleClose }) => h(
  NTag,
  { type: 'info', bordered: false, closable: true, onClose: handleClose },
  { default: () => option.label }
)

const onCreate = () =>({
  name: '',
  value: 'INSERT',
})
</script>

<template>
  <n-form
    label-align="right"
    label-placement="left"
    :label-width="210"
  >
    <n-form-item>
      <n-button type="warning" @click="handleReset">
        重置
      </n-button>
    </n-form-item>

    <n-grid cols="1 m:2" :x-gap="24" responsive="screen">
      <n-form-item-gi label="禁用SerialVersionUID">
        <n-radio-group v-model:value="strategyconfigStore.entity.disableSerialVersionUid">
          <n-radio-button :value="true" label="开启" />
          <n-radio-button :value="false" label="关闭" />
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="开启链式模型">
        <n-radio-group v-model:value="strategyconfigStore.entity.chainMode">
          <n-radio-button :value="true" label="开启" />
          <n-radio-button :value="false" label="关闭" />
        </n-radio-group>
      </n-form-item-gi>

      <n-form-item-gi label="生成字段常量">
        <n-radio-group v-model:value="strategyconfigStore.entity.columnContant">
          <n-radio-button :value="true" label="开启" />
          <n-radio-button :value="false" label="关闭" />
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="开启lombok模型">
        <n-radio-group v-model:value="strategyconfigStore.entity.lombok">
          <n-radio-button :value="true" label="开启" />
          <n-radio-button :value="false" label="关闭" />
        </n-radio-group>
      </n-form-item-gi>

      <n-form-item-gi label="Boolean类型移除is前缀">
        <n-radio-group v-model:value="strategyconfigStore.entity.booleanColumnRemoveIsPrefix">
          <n-radio-button :value="true" label="开启" />
          <n-radio-button :value="false" label="关闭" />
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="生成实体时生成字段注解">
        <n-radio-group v-model:value="strategyconfigStore.entity.enableTableFieldAnnotation">
          <n-radio-button :value="true" label="开启" />
          <n-radio-button :value="false" label="关闭" />
        </n-radio-group>
      </n-form-item-gi>

      <n-form-item-gi label="开启ActiveRecord模型">
        <n-radio-group v-model:value="strategyconfigStore.entity.activeRecord">
          <n-radio-button :value="true" label="开启" />
          <n-radio-button :value="false" label="关闭" />
        </n-radio-group>
      </n-form-item-gi>
      <n-form-item-gi label="全局主键类型">
        <n-select
          v-model:value="strategyconfigStore.entity.idType"
          :options="[
            {
              label: 'AUTO',
              value: 'auto',
            },
            {
              label: 'NONE',
              value: 'none',
            },
            {
              label: 'INPUT',
              value: 'input',
            },
            {
              label: 'ASSIGN_ID',
              value: 'assignId',
            },
            {
              label: 'ASSIGN_UUID',
              value: 'assignUuid',
            },
          ]"
        />
      </n-form-item-gi>
    </n-grid>

    <n-form-item label="父类包名">
      <n-input v-model:value="strategyconfigStore.entity.superClass" />
    </n-form-item>
    <n-form-item label="乐观锁字段名(数据库)">
      <n-input v-model:value="strategyconfigStore.entity.versionColumnName" />
    </n-form-item>
    <n-form-item label="乐观锁属性名(实体)">
      <n-input v-model:value="strategyconfigStore.entity.versionPropertyName" />
    </n-form-item>

    <n-form-item label="逻辑删除字段名(数据库)">
      <n-input v-model:value="strategyconfigStore.entity.logicDeleteColumnName" />
    </n-form-item>
    <n-form-item label="逻辑删除属性名(实体)	">
      <n-input v-model:value="strategyconfigStore.entity.logicDeletePropertyName" />
    </n-form-item>

    <n-form-item label="数据库表映射到实体的命名策略">
      <n-select v-model:value="strategyconfigStore.entity.naming" :options="[
        {
          label: '无操作',
          value: 'noChange',
        },
        {
          label: '下划线转驼峰',
          value: 'underlineToCamel',
        },
      ]" />
    </n-form-item>

    <n-form-item label="添加父类公共字段">
      <n-select
        v-model:value="strategyconfigStore.entity.superEntityColumns"
        :render-tag="renderTag"
        filterable
        multiple
        clearable
        tag
        placeholder="输入，按回车确认"
        :show-arrow="false"
        :show="false"
      />
    </n-form-item>
    <n-form-item label="添加忽略字段">
      <n-select
        v-model:value="strategyconfigStore.entity.ignoreColumns"
        :render-tag="renderTag"
        filterable
        multiple
        clearable
        tag
        placeholder="输入，按回车确认"
        :show-arrow="false"
        :show="false"
      />
    </n-form-item>

    <n-form-item label="添加填充字段">
      <n-dynamic-input
        v-model:value="strategyconfigStore.entity.tableFillList"
        item-style="margin-bottom: 0;"
        :on-create="onCreate"
        #="{ index, value }"
      >
        <div class="flex w-full">
          <!--
          通常，path 的变化会导致 form-item 验证内容或规则的改变，所以 naive-ui 会清理掉
          表项已有的验证信息。但是这个例子是个特殊情况，我们明确的知道，path 的改变不会导致
          form-item 验证内容和规则的变化，所以就 ignore-path-change
        -->
          <n-form-item
            ignore-path-change
            :show-label="false"
            :path="`dynamicInputValue[${index}].name`"
          >
            <n-input
              v-model:value="strategyconfigStore.entity.tableFillList[index].key"
              placeholder="e.g: updateTime"
              @keydown.enter.prevent
            />
          <!--
            由于在 input 元素里按回车会导致 form 里面的 button 被点击，所以阻止了默认行为
          -->
          </n-form-item>
          <div style="height: 34px; line-height: 34px; margin: 0 8px">
            -
          </div>
          <n-form-item
            ignore-path-change
            :show-label="false"
            class="w-full"
            :path="`dynamicInputValue[${index}].value`"
          >
            <n-select
              v-model:value="strategyconfigStore.entity.tableFillList[index].value"
              :options="[
                {
                  label: 'DEFAULT',
                  value: 'DEFAULT',
                },
                {
                  label: 'INSERT',
                  value: 'INSERT',
                },
                {
                  label: 'UPDATE',
                  value: 'UPDATE',
                },
                {
                  label: 'INSERT_UPDATE',
                  value: 'INSERT_UPDATE',
                },
              ]"
              @keydown.enter.prevent
            />
          </n-form-item>
        </div>
      </n-dynamic-input>
    </n-form-item>

    <n-form-item label="格式化文件名称">
      <n-input v-model:value="strategyconfigStore.entity.formatFilename" />
    </n-form-item>
  </n-form>
</template>

<style lang="css" scoped>
</style>
