<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/mp/strategyconfig'
import { Plus, Minus } from '@element-plus/icons-vue'

const globalPrimaryOptions = [
  {
    label: 'AUTO',
    value: 'AUTO',
  },
  {
    label: 'NONE',
    value: 'NONE',
  },
  {
    label: 'INPUT',
    value: 'INPUT',
  },
  {
    label: 'ASSIGN_ID',
    value: 'ASSIGN_ID',
  },
  {
    label: 'ASSIGN_UUID',
    value: 'ASSIGN_UUID',
  },
]

const namingOptions = [
  {
    label: '无操作',
    value: 'NoChange',
  },
  {
    label: '下划线转驼峰',
    value: 'UnderlineToCamel',
  },
]

const tableFillOptions = [
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
]

const strategyconfigStore = useStrategyConfigStore()

const handleReset = () => {
  strategyconfigStore.resetEntity()
}

const handleAddFillField = () => {

}

const handleRemoveFillField = (index: number) => {

}
</script>

<template>
  <el-form
    label-position="top"
    :model="strategyconfigStore.entity"
  >
    <el-form-item label-width="0">
      <el-button type="warning" @click="handleReset">
        重置
      </el-button>
    </el-form-item>

    <el-row :gutter="24">
      <el-col :sm="12" :md="8">
        <el-form-item label="覆盖文件">
          <el-radio-group v-model="strategyconfigStore.entity.fileOverride">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>
      <el-col :sm="12" :md="8">
        <el-form-item label="启动SerialVersionUID">
          <el-radio-group v-model="strategyconfigStore.entity.serialVersionUid">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>
      <el-col :sm="12" :md="8">
        <el-form-item label="开启链式模型">
          <el-radio-group v-model="strategyconfigStore.entity.chainMode">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>

      <el-col :sm="12" :md="8">
        <el-form-item label="生成字段常量">
          <el-radio-group v-model="strategyconfigStore.entity.columnConstant">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>
      <el-col :sm="12" :md="8">
        <el-form-item label="开启lombok模型">
          <el-radio-group v-model="strategyconfigStore.entity.lombok">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>
      <el-col :sm="12" :md="8">
        <el-form-item label="Boolean类型移除is前缀">
          <el-radio-group v-model="strategyconfigStore.entity.booleanColumnRemoveIsPrefix">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>

      <el-col :sm="12" :md="8">
        <el-form-item label="生成实体时生成字段注解">
          <el-radio-group v-model="strategyconfigStore.entity.enableTableFieldAnnotation">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>
      <el-col :sm="12" :md="8">
        <el-form-item label="开启ActiveRecord模型">
          <el-radio-group v-model="strategyconfigStore.entity.activeRecord">
            <el-radio-button :label="true">
              开启
            </el-radio-button>
            <el-radio-button :label="false">
              关闭
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-col>
      <el-col :sm="12" :md="8">
        <el-form-item label="全局主键类型">
          <el-select
            v-model="strategyconfigStore.entity.idType"
            :options="globalPrimaryOptions"
            placeholder="请选择"
          >
            <el-option
              v-for="opt in globalPrimaryOptions"
              :key="opt.value"
              :label="opt.label"
              :value="opt.value"
            >
              {{ opt.value }}
            </el-option>
          </el-select>
        </el-form-item>
      </el-col>
    </el-row>

    <el-row :gutter="24">
      <el-col :md="24">
        <el-form-item label="父类包名">
          <el-input
            v-model="strategyconfigStore.entity.superClass"
            placeholder="示例: com.baomidou.global.BaseEntity"
          />
        </el-form-item>
      </el-col>

      <el-col :md="12">
        <el-form-item label="乐观锁字段名">
          <el-input
            v-model="strategyconfigStore.entity.versionColumnName"
            placeholder="例如：version"
          />
        </el-form-item>
      </el-col>
      <el-col :md="12">
        <el-form-item label="乐观锁属性名">
          <el-input
            v-model="strategyconfigStore.entity.versionPropertyName"
            placeholder="例如：version"
          />
        </el-form-item>
      </el-col>

      <el-col :md="12">
        <el-form-item label="逻辑删除字段名">
          <el-input
            v-model="strategyconfigStore.entity.logicDeleteColumnName"
            placeholder="例如：is_deleted"
          />
        </el-form-item>
      </el-col>
      <el-col :md="12">
        <el-form-item label="逻辑删除属性名	">
          <el-input
            v-model="strategyconfigStore.entity.logicDeletePropertyName"
            placeholder="例如：isDeleted"
          />
        </el-form-item>
      </el-col>
    </el-row>

    <el-form-item label="数据库表映射到实体的命名策略">
      <el-select v-model="strategyconfigStore.entity.naming" class="w-full">
        <el-option
          v-for="opt in namingOptions"
          :key="opt.value"
          :label="opt.label"
          :value="opt.value"
        />
      </el-select>
    </el-form-item>

    <el-form-item label="添加父类公共字段">
      <el-select
        v-model="strategyconfigStore.entity.superEntityColumns"
        class="w-full"
        multiple
        clearable
        filterable
        allow-create
        default-first-option
        placeholder="例如: id, is_deleted, create_time，按回车确定"
      />
    </el-form-item>
    <el-form-item label="添加忽略字段">
      <el-select
        v-model="strategyconfigStore.entity.ignoreColumns"
        class="w-full"
        multiple
        clearable
        filterable
        allow-create
        default-first-option
        placeholder="例如：create_time, update_time，按回车确定"
      />
    </el-form-item>

    <el-form-item
      v-if="strategyconfigStore.entity.tableFillList.length === 0"
      label="添加填充字段"
    >
      <el-button
        text
        bg
        type="primary"
        :icon="Plus"
        class="w-full"
        @click="strategyconfigStore.appendInitTableFill"
      />
    </el-form-item>
    <el-form-item v-else label="添加填充字段">
      <el-row
        v-for="(tableFill, index) in strategyconfigStore.entity.tableFillList"
        :key="tableFill.fieldFill"
        :gutter="24"
        class="w-full mt-2"
      >
        <el-col :span="10">
          <el-form-item>
            <el-input v-model="tableFill.propertyName" />
          </el-form-item>
        </el-col>
        <el-col :span="1" class="flex justify-center">
          <span>-</span>
        </el-col>
        <el-col :span="10">
          <el-form-item>
            <el-select v-model="tableFill.fieldFill" class="w-full">
              <el-option
                v-for="fill in tableFillOptions"
                :key="fill.value"
                :label="fill.label"
                :value="fill.value"
              />
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="3">
          <el-space :size="0">
            <el-button
              type="primary"
              :icon="Plus"
              @click="strategyconfigStore.appendInitTableFill"
            />
            <el-button
              type="danger"
              :icon="Minus"
              @click="strategyconfigStore.removeFillTable(index)"
            />
          </el-space>
        </el-col>
      </el-row>
    </el-form-item>

    <el-form-item label="格式化文件名称">
      <el-input
        v-model="strategyconfigStore.entity.formatFilename"
        placeholder="示例: {}Entity, {}表示占位符"
      />
    </el-form-item>
  </el-form>
</template>

<style lang="css" scoped>
</style>
