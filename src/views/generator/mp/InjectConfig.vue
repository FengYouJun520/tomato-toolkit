<script setup lang="ts">
import { useInjectConfigStore } from '@/store/mp/injectConfig'
import { useGlobalConfigStore } from '@/store/mp/globalconfig'
import { CustomFile } from '@/types/type'
import { dialog, shell } from '@tauri-apps/api'
import {
  DataTableColumns,
  FormRules,
  NButton,
  NForm,
  NIcon,
  NModal,
  NPopconfirm,
  NSpace,
  NTag,
} from 'naive-ui'
import { EditOutlined, DeleteOutlined } from '@vicons/antd'
import { v4 } from 'uuid'
import MonacoEditor from 'monaco-editor-vue3'

const initialModel: CustomFile = {
  id: '',
  fileName: '',
  fileOverride: false,
  templatePath: '',
  filePath: '',
  packageName: '',
  addEntityPrefix: true,
}

const message = useMessage()
const injectConfigStore = useInjectConfigStore()
const globalConfigStore = useGlobalConfigStore()

const data = computed(() => injectConfigStore.getCustomFiles)
const columns: DataTableColumns<CustomFile> = [
  {
    title: '模板路径',
    key: 'templatePath',
    ellipsis: {
      tooltip: true,
    },
  },
  {
    title: '输出路径',
    key: 'filePath',
    ellipsis: {
      tooltip: true,
    },
    render: row => h(
      NTag,
      {
        type: row.filePath ? 'success' : 'warning',
      },
      () => row.filePath || `${globalConfigStore.outputDir}（未填，默为输出目录）`
    ),
  },
  {
    title: '包名',
    key: 'packageName',
  },
  {
    title: '文件名',
    key: 'fileName',
  },
  {
    title: '是否覆盖',
    key: 'fileOverride',
    width: 80,
    render: row => h(
      NTag, { type: row.fileOverride ? 'success' : 'warning' },
      () => row.fileOverride ? '是' : '否'
    ),
  },
  {
    title: 'entity前缀',
    key: 'addEntityPrefix',
    width: 100,
    render: row => h(
      NTag, { type: row.addEntityPrefix ? 'success' : 'warning' },
      () => row.addEntityPrefix ? '添加' : '去除'
    ),
  },
  {
    key: 'actions',
    title: '操作',
    fixed: 'right',
    render: row => h(
      NSpace,
      () => [
        h(
          NButton,
          {
            type: 'info',
            size: 'tiny',
            onClick: () => {
              Object.assign(model, { ...row })
              handleEditClick()
            },
          },
          {
            icon: () => h(
              NIcon,
              () => h(EditOutlined)
            ),
          }
        ),
        h(
          NPopconfirm,
          {
            onPositiveClick: () => {
              injectConfigStore.removeCustomFile(row.id)
            },
          },
          {
            default: () => `是否删除${row.fileName}？`,
            trigger: () => h(
              NButton,
              { type: 'error', size: 'tiny' },
              {
                icon: () => h(
                  NIcon,
                  () => h(DeleteOutlined)
                ),
              }
            ),
          }
        ),
      ]
    ),
  },
]

const rules: FormRules = {
  templatePath: {
    required: true,
    message: '模板路径必填',
  },
  fileName: {
    required: true,
    message: '文件名必填',
  },
}
const title = ref('')
const showModal = ref(false)
const isEdit = ref(false)
const formRef = ref<InstanceType<typeof NForm>|null>(null)
let model: CustomFile = reactive({ ...initialModel })
const outputDir = computed(() => globalConfigStore.outputDir)

const handleViewTemplateSyntax = async () => {
  shell.open('https://tera.netlify.app/docs/')
}

const handleNewClick = () => {
  showModal.value = true
  isEdit.value = false
  title.value = '新建自定义文件'
}

const handleEditClick = () => {
  showModal.value = true
  isEdit.value = true
  title.value = '编辑自定义文件'
}


const resetModel = () => {
  Object.assign(model, { ...initialModel })
}

const handleCloseModal = () => {
  resetModel()
}

const selectTemplatePath = async () => {
  const templatePath = await dialog.open({
    title: '选择模板路径',
    defaultPath: model.templatePath || unref(outputDir),
  })

  model.templatePath = templatePath as string
}

const selectFilePath = async () => {
  const filePath = await dialog.open({
    directory: true,
    title: '选择文件路径',
  })

  model.filePath = filePath as string
}


const handleConfirm = () => {
  formRef.value?.validate(errors => {
    if (errors) {
      return
    }

    // 编辑
    if (unref(isEdit)) {
      injectConfigStore.editCustomFile(model)
    } else {
      model.id = v4()
      injectConfigStore.addCustomFile(model)
    }

    // 添加
    message.success(unref(isEdit) ? '编辑成功' : '添加成功')
    showModal.value = false
  })
}

const customMap = ref('')
const showCustomMap = ref(false)

const handleAddCustomDataClick = () => {
  showCustomMap.value = true
  customMap.value = JSON.stringify(injectConfigStore.customMap, null, 2)
}

const handleSaveCustomMap = () => {
  injectConfigStore.addCustomData(JSON.parse(unref(customMap)))
  showCustomMap.value = false
}
</script>

<template>
  <div class="space-y-5">
    <div class="flex justify-between items-center">
      <NSpace>
        <NButton type="primary" @click="handleNewClick">
          新建
        </NButton>
        <NButton type="primary" @click="handleAddCustomDataClick">
          添加数据
        </NButton>
      </NSpace>

      <div>
        <NButton type="success" ghost @click="handleViewTemplateSyntax">
          查看模板语法
        </NButton>
      </div>
    </div>

    <n-data-table
      striped
      :single-line="false"
      :columns="columns"
      :data="data"
      :max-height="250"
    />
  </div>

  <NModal
    :show="showModal"
    :title="title"
    style="width: 70%;"
    preset="dialog"
    positive-text="确定"
    negative-text="取消"
    @positive-click="handleConfirm"
    @negative-click="showModal = false"
    @esc="showModal = false"
    @close="showModal = false"
    @mask-click="showModal = false"
    @after-leave="handleCloseModal"
  >
    <NForm
      ref="formRef"
      label-placement="top"
      :model="model"
      :rules="rules"
    >
      <n-grid cols="1 m:2" :x-gap="24" responsive="screen">
        <n-form-item-gi label="模板路径" path="templatePath">
          <n-input
            v-model:value="model.templatePath"
            placeholder="例如：D:\custom\entity.java"
          />
          <NButton type="info" @click="selectTemplatePath">
            选择
          </NButton>
        </n-form-item-gi>
        <n-form-item-gi label="输出文件路径" path="filePath">
          <n-input
            v-model:value="model.filePath"
            placeholder="例如：D:\custom，默认为outputDir路径"
          />
          <NButton type="info" @click="selectFilePath">
            选择
          </NButton>
        </n-form-item-gi>

        <n-form-item-gi label="自定义包名" path="packageName">
          <n-input
            v-model:value="model.packageName"
            placeholder="例如：module"
          />
        </n-form-item-gi>
        <n-form-item-gi label="文件名" path="fileName">
          <n-input
            v-model:value="model.fileName"
            placeholder="例如：Entity.java"
          />
        </n-form-item-gi>

        <n-form-item-gi label="是否覆盖" path="fileOverride">
          <n-radio-group v-model:value="model.fileOverride">
            <n-radio-button label="开启" :value="true" />
            <n-radio-button label="关闭" :value="false" />
          </n-radio-group>
        </n-form-item-gi>
        <n-form-item-gi label="是否添加entity前缀" path="addEntityPrefix">
          <n-radio-group v-model:value="model.addEntityPrefix">
            <n-radio-button label="开启" :value="true" />
            <n-radio-button label="关闭" :value="false" />
          </n-radio-group>
        </n-form-item-gi>
      </n-grid>
    </NForm>
  </NModal>

  <NModal
    v-model:show="showCustomMap"
    style="width: 80%;height: 90vh;"
  >
    <NCard title="添加自定义数据">
      <MonacoEditor
        v-model:value="customMap"
        height="100%"
        theme="vs-dark"
        language="json"
        :options="{
          fontSize: 18,
        }"
      />
      <template #footer>
        <NButton type="info" class="flex content-end" @click="handleSaveCustomMap">
          确定
        </NButton>
      </template>
    </NCard>
  </NModal>
</template>

<style lang="css" scoped>
.monaco-editor {
  height: calc(90vh - 120px);
  width: 100%
}
</style>
