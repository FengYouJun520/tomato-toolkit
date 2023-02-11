<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/mp/strategyconfig'
import EntityConfig from './EntityConfig.vue'
import ControllerConfig from './ControllerConfig.vue'
import ServiceConfig from './ServiceConfig.vue'
import MapperConfig from './MapperConfig.vue'
import { MpConfig } from '@/types/type'
import { useGlobalConfigStore } from '@/store/mp/globalconfig'
import { usePackageConfigStore } from '@/store/mp/packageconfig'
import { useTemplateConfigStore } from '@/store/mp/templateconfig'
import { useDatasourceStore } from '@/store/mp/datasource'
import { clipboard, invoke } from '@tauri-apps/api'
import MonacoEditor from 'monaco-editor-vue3'
import editor from 'monaco-editor/esm/vs/editor/editor.api'
import { useInjectConfigStore } from '@/store/mp/injectConfig'
import { useBasicTableInfosStore } from '@/store/mp/basicTable'
import { ElMessage } from 'element-plus'

const active = ref('entity-config')
const datasourceConfigStore = useDatasourceStore()
const globalConfigStore = useGlobalConfigStore()
const packageConfigStore = usePackageConfigStore()
const templateConfigStore = useTemplateConfigStore()
const strategyConfigStore = useStrategyConfigStore()
const injectConfigStore = useInjectConfigStore()
const basicTableInfos = useBasicTableInfosStore()

const editorOptions: editor.editor.IStandaloneEditorConstructionOptions = {
  fontSize: 18,
  folding: true,
}

const includes = ref<string[]>([])
const excludes = ref<string[]>([])

watch(includes, () => {
  strategyConfigStore.include = includes.value
  excludes.value = excludes.value.filter(exclude => !includes.value.includes(exclude))
})

watch(excludes, () => {
  strategyConfigStore.exclude = excludes.value
})


const allSelect = () => {
  includes.value = basicTableInfos.basicTables.map(table => table.name)
}

const handleReset = () => {
  strategyConfigStore.$reset()
}

const options = computed(() => basicTableInfos.basicTables.map(table => ({
  label: table.name,
  value: table.name,
  comment: table.comment,
})))

const excludeOptions = computed(() => {
  const result = basicTableInfos.basicTables.filter(table => includes.value.every(value => value !== table.name))
  return result.map(table=>({
    label: table.name,
    value: table.name,
    comment: table.comment,
  }))
})

const showPreview = ref(false)
const contextData = ref('')

const handlePreview = async () => {
  try {
    const config: MpConfig = {
      datasource: datasourceConfigStore.$state,
      global: globalConfigStore.$state,
      package: packageConfigStore.$state,
      template: templateConfigStore.$state,
      strategy: strategyConfigStore.$state,
      injection: injectConfigStore.$state,
    }
    const data = await invoke<Record<string, any>>('generate_preview', { config })
    contextData.value = JSON.stringify(data, null, 2)
    showPreview.value = true
  } catch (error) {
    ElMessage.error(error as string)
    showPreview.value = false
  }
}

const copyContextData = async () => {
  try {
    clipboard.writeText(JSON.stringify(contextData.value, null, 2))
  } catch (error) {
    ElMessage.error(error as string)
  }
}
</script>

<template>
  <div>
    <el-form
      :label-width="120"
      :model="strategyConfigStore.$state"
    >
      <el-form-item label-width="0">
        <el-space>
          <el-button type="warning" @click="handleReset">
            重置
          </el-button>

          <el-button
            type="primary"
            :disabled="!includes || !includes.length"
            @click="handlePreview"
          >
            预览生成的数据
          </el-button>
        </el-space>
      </el-form-item>

      <el-row :gutter="24">
        <el-col :md="12">
          <el-form-item label="开启大写命名">
            <el-radio-group v-model="strategyConfigStore.isCapitalMode">
              <el-radio-button :label="true">
                开启
              </el-radio-button>
              <el-radio-button :label="false">
                关闭
              </el-radio-button>
            </el-radio-group>
          </el-form-item>
        </el-col>
        <el-col :md="12">
          <el-form-item label="开启sql过滤">
            <el-radio-group v-model="strategyConfigStore.enableSqlFilter">
              <el-radio-button :label="true">
                开启
              </el-radio-button>
              <el-radio-button :label="false">
                关闭
              </el-radio-button>
            </el-radio-group>
          </el-form-item>
        </el-col>

        <el-col :md="12">
          <el-form-item label="开启Schema">
            <el-radio-group v-model="strategyConfigStore.enableSchema">
              <el-radio-button :label="true">
                开启
              </el-radio-button>
              <el-radio-button :label="false">
                关闭
              </el-radio-button>
            </el-radio-group>
          </el-form-item>
        </el-col>
      </el-row>

      <el-row>
        <el-col :sm="18" :md="20">
          <el-form-item label="包含的表">
            <el-select
              v-model="includes"
              multiple
              clearable
              class="w-full"
              placeholder="请选择"
            >
              <el-option
                v-for="opt in options"
                :key="opt.value"
                :label="opt.label"
                :value="opt.value"
              >
                {{ opt.label }}
              </el-option>
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :sm="6" :md="4">
          <el-button
            type="primary"
            :disabled="basicTableInfos.isEmpty"
            @click="allSelect"
          >
            全选
          </el-button>
        </el-col>
      </el-row>

      <el-form-item label="排除的表">
        <el-select
          v-model="excludes"
          multiple
          clearable
          class="w-full"
          placeholder="请选择"
        >
          <el-option
            v-for="opt in excludeOptions"
            :key="opt.value"
            :label="opt.label"
            :value="opt.value"
          >
            {{ opt.label }}
          </el-option>
        </el-select>
      </el-form-item>
    </el-form>

    <el-tabs v-model="active" type="border-card">
      <el-tab-pane label="Entity配置" name="entity-config">
        <EntityConfig />
      </el-tab-pane>
      <el-tab-pane label="Controller配置" name="controller-config">
        <ControllerConfig />
      </el-tab-pane>
      <el-tab-pane label="Service配置" name="service-config">
        <ServiceConfig />
      </el-tab-pane>
      <el-tab-pane label="Mapper配置" name="mapper-config">
        <MapperConfig />
      </el-tab-pane>
    </el-tabs>

    <el-dialog
      v-model="showPreview"
      title="查看生成预览的数据"
      style="width: 80%;"
    >
      <MonacoEditor
        v-model:value="contextData"
        height="100%"
        width="100%"
        theme="vs-dark"
        language="json"
        :options="editorOptions"
      />
      <template #footer>
        <el-button type="primary" @click="copyContextData">
          复制
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style lang="css" scoped>
</style>
