<script setup lang="ts">
import { useStrategyConfigStore } from '@/store/modules/mp/strategyconfig'
import { NTag, SelectOption, SelectRenderLabel, SelectRenderTag, NModal, NCard } from 'naive-ui'
import { useTables } from '../useTables'
import EntityConfig from './EntityConfig.vue'
import ControllerConfig from './ControllerConfig.vue'
import ServiceConfig from './ServiceConfig.vue'
import MapperConfig from './MapperConfig.vue'
import { MpConfig } from '@/types/type'
import { useGlobalConfigStore } from '@/store/modules/mp/globalconfig'
import { usePackageConfigStore } from '@/store/modules/mp/packageconfig'
import { useTemplateConfigStore } from '@/store/modules/mp/templateconfig'
import { useDatasourceStore } from '@/store/modules/mp/datasource'
import { clipboard, invoke } from '@tauri-apps/api'
import VueJsonPretty from 'vue-json-pretty'
import { useInjectConfigStore } from '@/store/modules/mp/injectConfig'

const message = useMessage()
const datasourceConfigStore = useDatasourceStore()
const globalConfigStore = useGlobalConfigStore()
const packageConfigStore = usePackageConfigStore()
const templateConfigStore = useTemplateConfigStore()
const strategyConfigStore = useStrategyConfigStore()
const injectConfigStore = useInjectConfigStore()
const tablesContext = useTables()

const includes = ref<string[]>([])
const excludes = ref<string[]>([])

const disabledAllSelect = computed(() => !tablesContext.tables.value.length)

watch(includes, () => {
  strategyConfigStore.include = includes.value
  excludes.value = excludes.value.filter(exclude => !includes.value.includes(exclude))
})

watch(excludes, () => {
  strategyConfigStore.exclude = excludes.value
})


const allSelect = () => {
  includes.value = tablesContext.tables.value.map(table => table.name)
}

const handleReset = () => {
  strategyConfigStore.$reset()
}

const options = computed<SelectOption[]>(() => tablesContext.tables.value.map(table => ({
  label: table.name,
  value: table.name,
  comment: table.comment,
})))

const excludeOptions = computed<SelectOption[]>(() => {
  const result = tablesContext.tables.value.filter(table => includes.value.every(value => value !== table.name))
  return result.map(table=>({
    label: table.name,
    value: table.name,
    comment: table.comment,
  }))
})

const renderLabel: SelectRenderLabel = option => h(
  NTag,
  { type: 'info', bordered: false },
  {
    default: () => h(
      'div',
      { class: 'flex justify-between items-center space-x-2' },
      {
        default: () => [
          h(
            'span',
            null,
            { default: () => option.label }
          ),
          h(
            'span',
            null,
            { default: () => option.comment }
          ),
        ],
      }
    ),
  }
)

const renderTag: SelectRenderTag = ({ option, handleClose }) => h(
  NTag,
  { type: 'info', bordered: false, closable: true, onClose: handleClose },
  { default: () => option.label }
)

const showPreview = ref(false)
const modelRef = ref<InstanceType<typeof NCard>|null>(null)
const contextData = ref<Record<string, any>>({})
const { height } = useElementSize(modelRef)

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
    contextData.value = data
    showPreview.value = true
  } catch (error) {
    message.error(error as string)
    showPreview.value = false
  }
}

const copyContextData = async () => {
  try {
    clipboard.writeText(JSON.stringify(contextData.value, null, 2))
  } catch (error) {
    message.error(error as string)
  }
}
</script>

<template>
  <div>
    <n-form
      label-align="right"
      label-placement="left"
      :label-width="120"
    >
      <n-form-item>
        <n-space>
          <n-button type="warning" @click="handleReset">
            重置
          </n-button>

          <n-button
            type="info"
            :disabled="!includes || !includes.length"
            @click="handlePreview"
          >
            预览生成的数据
          </n-button>
        </n-space>
      </n-form-item>

      <n-grid cols="1 m:2" :x-gap="24" responsive="screen">
        <n-form-item-gi label="开启大写命名">
          <n-radio-group v-model:value="strategyConfigStore.isCapitalMode">
            <n-radio-button :value="true">
              开启
            </n-radio-button>
            <n-radio-button :value="false">
              关闭
            </n-radio-button>
          </n-radio-group>
        </n-form-item-gi>

        <n-form-item-gi label="开启sql过滤">
          <n-radio-group v-model:value="strategyConfigStore.enableSqlFilter">
            <n-radio-button :value="true">
              开启
            </n-radio-button>
            <n-radio-button :value="false">
              关闭
            </n-radio-button>
          </n-radio-group>
        </n-form-item-gi>
        <n-form-item-gi label="开启Schema">
          <n-radio-group v-model:value="strategyConfigStore.enableSchema">
            <n-radio-button :value="true">
              开启
            </n-radio-button>
            <n-radio-button :value="false">
              关闭
            </n-radio-button>
          </n-radio-group>
        </n-form-item-gi>
      </n-grid>

      <n-form-item label="包含的表">
        <n-select
          v-model:value="includes"
          multiple
          :options="options"
          clearable
          :render-tag="renderTag"
          :render-label="renderLabel"
        />
        <n-button type="info" :disabled="disabledAllSelect" @click="allSelect">
          全选
        </n-button>
      </n-form-item>
      <n-form-item label="排除的表">
        <n-select
          v-model:value="excludes"
          :options="excludeOptions"
          multiple
          clearable
          :render-tag="renderTag"
          :render-label="renderLabel"
        />
      </n-form-item>
    </n-form>

    <n-tabs default-value="entity-config" type="card">
      <n-tab-pane tab="Entity配置" name="entity-config">
        <EntityConfig />
      </n-tab-pane>
      <n-tab-pane tab="Controller配置" name="controller-config">
        <ControllerConfig />
      </n-tab-pane>
      <n-tab-pane tab="Service配置" name="service-config">
        <ServiceConfig />
      </n-tab-pane>
      <n-tab-pane tab="Mapper配置" name="mapper-config">
        <MapperConfig />
      </n-tab-pane>
    </n-tabs>

    <NModal
      v-model:show="showPreview"
      title="查看生成预览的数据"
      style="width: 70%;height: 90vh;"
    >
      <NCard ref="modelRef">
        <VueJsonPretty
          :data="contextData"
          :height="height - 100"
          virtual
          show-icon
          show-line
          show-line-number
        />
        <template #footer>
          <n-button type="info" @click="copyContextData">
            复制
          </n-button>
        </template>
      </NCard>
    </NModal>
  </div>
</template>

<style lang="css" scoped>
</style>
