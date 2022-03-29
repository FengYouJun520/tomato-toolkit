<template>
  <div>
    <t-form label-align="right" :label-width="160" colon>
      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col>
          <t-button theme="warning">重置</t-button>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col :xs="12" :md="6">
          <t-form-item label="禁用SerialVersionUID">
            <t-radio-group :default-value="true" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6">
          <t-form-item label="使用链式模式">
            <t-radio-group :default-value="false" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6">
          <t-form-item label="使用lombok">
            <t-radio-group :default-value="false" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6">
          <t-form-item label="使用record">
            <t-radio-group :default-value="false" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6">
          <t-form-item label="使用静态常量字段">
            <t-radio-group :default-value="false" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
        <t-col :xs="12" :md="6">
          <t-form-item label="移除Boolean类型is前缀">
            <t-radio-group :default-value="false" variant="primary-filled">
              <t-radio-button :value="true">开启</t-radio-button>
              <t-radio-button :value="false">关闭</t-radio-button>
            </t-radio-group>
          </t-form-item>
        </t-col>
      </t-row>

      <t-row :gutter="[24, 24]" class="mt-6">
        <t-col>
          <t-form-item label="父类包名">
            <t-input />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="乐观锁属性名">
            <t-input />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="乐观锁字段名">
            <t-input />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="逻辑删除属性名">
            <t-input />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="逻辑删除字段名">
            <t-input />
          </t-form-item>
        </t-col>

        <t-col>
          <t-form-item label="实体命名策略">
            <t-select :options="namingOptions" default-value="underline_to_camel" />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="实体命名策略">
            <t-tag-input v-model="tags" excess-tags-display-type="break-line" clearable />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="自动填充字段">
            <div v-if="fillFields && fillFields.length" class="flex flex-col gap-6">
              <t-input-group separate v-for="fillField in fillFields" :key="fillField.key">
                <t-row>
                  <t-col :span="4">
                    <t-input v-model="fillField.value" style="width: 140px" />
                  </t-col>
                  <t-col :span="1" class="text-center">
                    <span class="text-center align-middle">-</span>
                  </t-col>
                  <t-col :span="5">
                    <t-select
                      v-model="fillField.fill"
                      :options="fillOptions"
                      style="width: 160px"
                    />
                  </t-col>
                  <t-col :span="2">
                    <div class="flex gap-1 ml-3">
                      <t-button @click="addFillField">+</t-button>
                      <t-button @click="removeFillField(fillField)">-</t-button>
                    </div>
                  </t-col>
                </t-row>
              </t-input-group>
            </div>

            <t-button v-else block @click="addFillField">+</t-button>
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="全局主键类型">
            <t-select :options="assignIds" />
          </t-form-item>
        </t-col>
        <t-col>
          <t-form-item label="类格式">
            <t-input default-value="%s" />
          </t-form-item>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script setup lang="ts">
import { nanoid } from 'nanoid'
import { assignIds, FillInput, fillOptions } from './data'

const namingOptions = [
  {
    label: '下划线转驼峰',
    value: 'underline_to_camel',
  },
]

const tags = ref<string[]>([])
const fillFields = ref<FillInput[]>([])

const addFillField = () => {
  fillFields.value.push({
    key: nanoid(),
    value: '',
    fill: '',
  })
}

const removeFillField = (fillInput: FillInput) => {
  const index = fillFields.value.findIndex((fill) => fill.key === fillInput.key)
  if (index !== -1) {
    fillFields.value.splice(index, 1)
  }
}
</script>

<style lang="scss" scoped></style>
