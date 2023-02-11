<script setup lang="ts">
import { useInjectConfigStore } from '@/store/mp/injectConfig'
import { useGlobalConfigStore } from '@/store/mp/globalconfig'
import { CustomFile } from '@/types/type'
import { dialog, shell } from '@tauri-apps/api'
import { Edit, Delete } from '@element-plus/icons-vue'
import { v4 } from 'uuid'
import { FormInstance } from 'element-plus'
import Editor from '@guolao/vue-monaco-editor'

const initialModel: CustomFile = {
  id: '',
  fileName: '',
  fileOverride: false,
  templatePath: '',
  filePath: '',
  packageName: '',
  addEntityPrefix: true,
}

const injectConfigStore = useInjectConfigStore()
const globalConfigStore = useGlobalConfigStore()

/// 自定义文件操作
const data = computed(() => injectConfigStore.getCustomFiles)

const title = ref('')
const showModal = ref(false)
const isEdit = ref(false)
const formRef = ref<FormInstance>()
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

const handleEditClick = (row: CustomFile) => {
  showModal.value = true
  isEdit.value = true
  title.value = '编辑自定义文件'
  Object.assign(model, row)
}

const handleCloseModal = () => {
  Object.assign(model, { ...initialModel })
  formRef.value?.clearValidate()
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

const handleConfirm = async (formEl?: FormInstance) => {
  if (!formEl) {
    return
  }

  await formEl.validate((valid, fields) => {
    if (!valid) {
      ElMessage.error('校验失败')
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
    ElMessage.success(unref(isEdit) ? '编辑成功' : '添加成功')
    showModal.value = false
  })
}

/// 自定义数据操作
const customMap = ref('')
const showCustomMap = ref(false)

const handleAddCustomDataClick = () => {
  showCustomMap.value = true
  customMap.value = JSON.stringify(injectConfigStore.customMap, null, 2)
}

const handleSaveCustomMap = () => {
  try {
    injectConfigStore.addCustomData(JSON.parse(unref(customMap)))
    ElMessage.success('添加自定义数据成功')
  } catch (error) {
    ElMessage.error('添加自定义数据失败，请检查格式是否正确')
  }
  showCustomMap.value = false
}
</script>

<template>
  <div class="space-y-5">
    <div class="flex justify-between items-center">
      <el-space>
        <el-button type="primary" @click="handleNewClick()">
          新建
        </el-button>
        <el-button type="primary" @click="handleAddCustomDataClick">
          添加数据
        </el-button>
      </el-space>

      <div>
        <el-button text bg @click="handleViewTemplateSyntax">
          查看模板语法
        </el-button>
      </div>
    </div>

    <el-table
      :data="data"
      stripe
      border
      :max-height="300"
    >
      <el-table-column
        label="模板路径"
        prop="templatePath"
        show-overflow-tooltip
      />

      <el-table-column
        label="输出路径"
        prop="filePath"
        show-overflow-tooltip
      >
        <template #default="scope">
          <span v-if="scope.row.filePath">{{ scope.row.filePath }}</span>
          <el-tag type="warning">
            {{ `${globalConfigStore.outputDir}（未填写，使用outputDir）` }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column
        label="包名"
        prop="packageName"
        show-overflow-tooltip
      />
      <el-table-column
        label="文件名"
        prop="fileName"
        show-overflow-tooltip
      />
      <el-table-column
        label="是否覆盖"
        prop="fileOverride"
      >
        <template #default="scope">
          <el-tag :type="scope.row.fileOverride ? 'success' : 'info'">
            {{ scope.row.fileOverride ? '是' : '否' }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column
        label="entity前缀"
        prop="addEntityPrefix"
      >
        <template #default="scope">
          <el-tag :type="scope.row.addEntityPrefix ? 'success' : 'info'">
            {{ scope.row.addEntityPrefix ? '添加' : '去除' }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column
        label="操作"
        fixed="right"
      >
        <template #default="scope">
          <el-space size="small" wrap>
            <el-button
              size="small"
              text
              type="primary"
              @click="handleEditClick(scope.row)"
            >
              <template #icon>
                <el-icon>
                  <Edit />
                </el-icon>
              </template>
            </el-button>
            <el-popconfirm
              title="是否删除该模板？"
              confirm-button-text="确定"
              cancel-button-text="取消"
              @confirm="injectConfigStore.removeCustomFile(scope.row.id)"
            >
              <template #reference>
                <el-button size="small" text type="danger">
                  <template #icon>
                    <el-icon>
                      <Delete />
                    </el-icon>
                  </template>
                </el-button>
              </template>
            </el-popconfirm>
          </el-space>
        </template>
      </el-table-column>
    </el-table>
  </div>

  <el-dialog
    v-model="showModal"
    :title="title"
    width="80%"
    @closed="handleCloseModal"
  >
    <el-form
      ref="formRef"
      label-position="top"
      :model="model"
      status-icon
    >
      <el-row :gutter="24">
        <el-col :md="12">
          <el-form-item
            label="模板路径"
            prop="templatePath"
            :rules="{
              required: true,
              message: '模板路径必填',
              trigger: 'blur',
            }"
          >
            <el-input
              v-model="model.templatePath"
              placeholder="例如：D:\custom\entity.java"
            >
              <template #append>
                <el-button type="primary" @click="selectTemplatePath">
                  选择
                </el-button>
              </template>
            </el-input>
          </el-form-item>
        </el-col>
        <el-col :md="12">
          <el-form-item label="输出路径" prop="filePath">
            <el-input
              v-model="model.filePath"
              placeholder="例如：D:\custom"
            >
              <template #append>
                <el-button type="primary" @click="selectFilePath">
                  选择
                </el-button>
              </template>
            </el-input>
          </el-form-item>
        </el-col>

        <el-col :md="12">
          <el-form-item label="包名" prop="packageName">
            <el-input
              v-model="model.packageName"
              placeholder="例如：com.dto"
            />
          </el-form-item>
        </el-col>
        <el-col :md="12">
          <el-form-item
            label="文件名"
            prop="fileName"
            :rules="{
              required: true,
              message: '文件名必填',
              trigger: 'blur',
            }"
          >
            <el-input
              v-model="model.fileName"
              placeholder="例如：Entity.java"
            />
          </el-form-item>
        </el-col>

        <el-col :md="12">
          <el-form-item label="是否覆盖" prop="fileOverride">
            <el-radio-group v-model="model.fileOverride">
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
          <el-form-item label="是否添加entity前缀" prop="addEntityPrefix">
            <el-radio-group v-model="model.addEntityPrefix">
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
    </el-form>

    <template #footer>
      <el-button text bg @click="showModal = false">
        取消
      </el-button>
      <el-button type="primary" @click="handleConfirm(formRef)">
        确定
      </el-button>
    </template>
  </el-dialog>

  <el-dialog
    v-model="showCustomMap"
    title="添加自定义数据"
    width="80%"
    style="height: 85vh;"
    top="20px"
    destroy-on-close
  >
    <Editor
      v-model:value="customMap"
      theme="vs-dark"
      height="62vh"
      language="json"
      :options="{
        fontSize: 18,
        tabSize: 2,
      }"
    />
    <template #footer>
      <el-button type="primary" @click="handleSaveCustomMap">
        确定
      </el-button>
    </template>
  </el-dialog>
</template>

<style lang="css" scoped>
</style>
