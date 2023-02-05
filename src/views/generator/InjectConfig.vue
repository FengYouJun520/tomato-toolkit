<script setup lang="ts">
import { useInjectConfigStore } from '@/store/modules/injectConfig'
import { useGlobalConfigStore } from '@/store/modules/mp/globalconfig'
import { CustomFile } from '@/types/type'
import { dialog } from '@tauri-apps/api'
import {
  DataTableColumns,
  FormRules,
  NButton,
  NForm,
  NIcon,
  NPopconfirm,
  NSpace,
  NTag,
} from 'naive-ui'
import { EditOutlined, DeleteOutlined } from '@vicons/antd'

import { v4 } from 'uuid'

const initialModel: CustomFile = {
  id: '',
  fileName: '',
  fileOverride: false,
  templatePath: '',
  filePath: '',
  packageName: '',
}

const message = useMessage()
const injectConfigStore = useInjectConfigStore()
const globalConfigStore = useGlobalConfigStore()

const data = computed(() => injectConfigStore.getCustomFiles)
const columns: DataTableColumns<CustomFile> = [
  {
    title: '模板路径',
    key: 'templatePath',
  },
  {
    title: '输出路径',
    key: 'filePath',
    render: row => h(
      NTag,
      {
        type: row.filePath ? 'success' : 'warning',
      },
      {
        default: () => row.filePath || `${globalConfigStore.outputDir}（未填，默认outputDir）`,
      }
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
    render: row => h(
      NTag, { type: row.fileOverride ? 'success' : 'error' },
      { default: () => row.fileOverride ? '是' : '否' }
    ),
  },
  {
    key: 'actions',
    title: '操作',
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
    defaultPath: unref(outputDir),
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
</script>

<template>
  <div class="space-y-5">
    <NSpace>
      <NButton type="primary" @click="handleNewClick">
        新建
      </NButton>
    </NSpace>

    <n-data-table
      :columns="columns"
      :data="data"
      :bordered="false"
    />
  </div>

  <n-modal
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
      </n-grid>
    </NForm>
  </n-modal>
</template>

<style lang="css" scoped>
</style>
