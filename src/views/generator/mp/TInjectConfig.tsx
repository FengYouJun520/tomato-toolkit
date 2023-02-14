import {
  Button,
  Form,
  Grid,
  Input,
  Message,
  Modal,
  Radio,
  Space,
  Table,
  TableColumnProps,
  Tag,
  Tooltip,
} from '@arco-design/web-react'
import { FC } from 'react'
import Editor from '@monaco-editor/react'
import * as MonacoEditor from 'monaco-editor/esm/vs/editor/editor.api'
import { IconDelete, IconEdit, IconLoading, IconPlus } from '@arco-design/web-react/icon'
import { dialog, shell } from '@tauri-apps/api'
import { CustomFile } from '@/types/type'
import { useStore } from '@/store'
import { renderRadio } from '@/utils/renderRadio'
import { v4 } from 'uuid'

const initialCustomFile: CustomFile = {
  id: '',
  templatePath: '',
  filePath: '',
  packageName: '',
  fileName: '',
  fileOverride: false,
  addEntityPrefix: false,
}

const InjectConfig: FC = () => {
  const store = useStore()

  const columns: TableColumnProps<CustomFile>[] = [
    {
      title: '模板路径',
      dataIndex: 'templatePath',
      width: 160,
      ellipsis: true,
    },
    {
      title: '输出路径',
      dataIndex: 'filePath',
      width: 160,
      ellipsis: true,
      render: (_, item) => item.filePath ||
       <Tooltip
         content={`${store.mp.outputDir}（默认路径）`}
       >
         <Tag color="#ff7d00">
           {store.mp.outputDir}（默认路径）
         </Tag>
       </Tooltip>,
    },
    {
      title: '包名',
      dataIndex: 'packageName',
    },
    {
      title: '文件名',
      dataIndex: 'fileName',
    },
    {
      title: '是否覆盖',
      width: 100,
      dataIndex: 'fileOverride',
      render: (col, item ) => (
        <Tag color={item.fileOverride ? '#00b42a' : '#f28c18'} >
          {item.fileOverride ? '是' : '否'}
        </Tag>
      ),
    },
    {
      title: 'entity前缀',
      dataIndex: 'addEntityPrefix',
      render: (col, item ) => (
        <Tag color={item.addEntityPrefix ? '#00b42a' : '#f28c18'} >
          {item.addEntityPrefix ? '是' : '否'}
        </Tag>
      ),
    },
    {
      title: '操作',
      render (col, item, _index)  {
        return <Space>
          <Button
            size="mini"
            type="primary"
            icon={<IconEdit />}
            onClick={() => handleEditCustomFile(item)}
          />
          <Button
            size="mini"
            status="danger"
            icon={<IconDelete />}
            onClick={() => handleDelete(item)}
          />
        </Space>
      },
    },
  ]

  const options: MonacoEditor.editor.IStandaloneEditorConstructionOptions = {
    selectOnLineNumbers: true,
    links: false,
    cursorStyle: 'line',
    lineNumbers: 'on',
    contextmenu: false,
    tabSize: 2,
    fontSize: 18,
    showFoldingControls: 'always',
    wordWrap: 'on',
    wrappingIndent: 'indent',
    renderLineHighlight: 'none',
    occurrencesHighlight: false,
    scrollBeyondLastLine: false,
    hideCursorInOverviewRuler: true,
    folding: true,
    colorDecorators: false,
    minimap: {
      enabled: true,
    },
    guides: {
      indentation: true,
      highlightActiveIndentation: true,
      bracketPairs: true,
    },
    scrollbar: {
      useShadows: false,
      verticalScrollbarSize: 10,
      horizontalScrollbarSize: 10,
    },
    wordWrapOverride2: 'off',
  }

  const openTemplateSyntax = async () => {
    await shell.open('https://tera.netlify.app/docs/')
  }

  const [visibleCustomMap, setVisibleCustomMap] = useState(false)
  const [customMap, setCustomMap] = useState('')

  const handleOpenCustomMap = () => {
    setCustomMap(JSON.stringify(store.mp.injectStore.customMap, null, 2))
    setVisibleCustomMap(true)
  }

  const handleOk = () => {
    const value = JSON.parse(customMap)
    store.mp.injectStore.setCustomMap(value)
    setVisibleCustomMap(false)
  }

  const [form] = Form.useForm<CustomFile>()
  const [visibleCustomFile, setVisibleCustomFile] = useState(false)
  const [customFile, setCustomFile] = useState<CustomFile>(initialCustomFile)
  const [isEdit, setIsEdit] = useState(false)
  const modalTitle = isEdit ? '编辑自定义文件' : '创建自定义文件'

  const handleCreateCustomFile = () => {
    setIsEdit(false)
    setVisibleCustomFile(true)
    setCustomFile({...initialCustomFile})
  }

  const handleEditCustomFile = (customFile: CustomFile) => {
    setCustomFile(customFile)
    setIsEdit(true)
    setVisibleCustomFile(true)
  }

  const handleDelete = (item: CustomFile) => {
    Modal.confirm({
      title: '删除自定义模板',
      content: '是否确认删除该自定义模板？',
      okText: '删除',
      okButtonProps: {
        status: 'danger',
      },
      onOk: () => {
        store.mp.injectStore.deleteCustomFile(item.id)
      },
    })
  }

  const handleCustomFileOk = async () => {
    await form.validate()
    // 编辑
    if(isEdit) {
      console.log(customFile)
      store.mp.injectStore.editCustomFile(customFile)
    }else{
      // 创建
      customFile.id = v4()
      store.mp.injectStore.addCustomFile(customFile)
    }
    Message.success('校验通过')
    setVisibleCustomFile(false)
  }

  const selectTemplatePath = async () => {
    const templatePath = await dialog.open({title: '选择模板路径'})
    form.setFieldValue('templatePath', templatePath as string)
  }
  const selectFilePath = async () => {
    const filePath = await dialog.open({
      title: '选择文件路径',
      directory: true,
      defaultPath: store.mp.outputDir,
    })
    form.setFieldValue('filePath', filePath as string)
  }
  return (
    <div className="space-y-5">
      <div className="flex justify-between items-center">
        <Space>
          <Button type="primary" icon={<IconPlus />} onClick={handleCreateCustomFile}>新建</Button>
          <Button type="primary" onClick={handleOpenCustomMap}>
            添加自定义数据
          </Button>
        </Space>

        <Button type="outline" onClick={openTemplateSyntax} >
          查看模板语法
        </Button>
      </div>

      <Table<CustomFile>
        columns={columns}
        data={store.mp.injectStore.customFiles}
      />

      <Modal
        title="添加自定义数据"
        visible={visibleCustomMap}
        unmountOnExit
        onOk={handleOk}
        onCancel={() => setVisibleCustomMap(false)}
        style={{height: '90vh', width: '80%'}}
      >
        <Editor
          height="68vh"
          options={options}
          theme="vs-dark"
          defaultLanguage="json"
          value={customMap}
          onChange={(value) => setCustomMap(value || '')}
          loading={<IconLoading spin />}
        />
      </Modal>

      <Modal
        title={modalTitle}
        visible={visibleCustomFile}
        unmountOnExit
        onOk={handleCustomFileOk}
        onCancel={() => setVisibleCustomFile(false)}
        style={{ width: '80%'}}
      >
        <Form<CustomFile>
          form={form}
          layout="vertical"
          autoComplete="off"
          colon
          initialValues={customFile}
          onValuesChange={(_, vs) => {
            setCustomFile({
              ...customFile,
              ...vs,
            })
          }}
        >
          <Grid.Row gutter={24}>
            <Grid.Col xs={24} md={12}>
              <Form.Item label="模板路径" required>
                <Grid.Row>
                  <Grid.Col span={20}>
                    <Form.Item field="templatePath" rules={[{required: true, message: '模板路径必填'}]}>
                      <Input placeholder="例如：D:\custom\entity.java" />
                    </Form.Item>
                  </Grid.Col>
                  <Grid.Col span={4}>
                    <Button long type="primary" onClick={selectTemplatePath}>选择</Button>
                  </Grid.Col>
                </Grid.Row>
              </Form.Item>
            </Grid.Col>
            <Grid.Col xs={24} md={12}>
              <Form.Item label="输出文件路径">
                <Grid.Row>
                  <Grid.Col span={20}>
                    <Form.Item field="filePath">
                      <Input placeholder="例如：D:\custom，默认为outputDir路径" />
                    </Form.Item>
                  </Grid.Col>
                  <Grid.Col span={4}>
                    <Button long type="primary" onClick={selectFilePath}>选择</Button>
                  </Grid.Col>
                </Grid.Row>
              </Form.Item>
            </Grid.Col>

            <Grid.Col xs={24} md={12}>
              <Form.Item field="packageName" label="自定义包名">
                <Input placeholder="例如：module" />
              </Form.Item>
            </Grid.Col>
            <Grid.Col xs={24} md={12}>
              <Form.Item field="fileName" label="文件名" rules={[{required: true, message: '文件名必填'}]}>
                <Input  placeholder="例如：Entity.java" />
              </Form.Item>
            </Grid.Col>

            <Grid.Col xs={24} md={12}>
              <Form.Item field="fileOverride" label="是否覆盖">
                <Radio.Group type="button">
                  {renderRadio()}
                </Radio.Group>
              </Form.Item>
            </Grid.Col>
            <Grid.Col xs={24} md={12}>
              <Form.Item field="addEntityPrefix" label="是否添加entity前缀">
                <Radio.Group type="button">
                  {renderRadio()}
                </Radio.Group>
              </Form.Item>
            </Grid.Col>
          </Grid.Row>
        </Form>
      </Modal>
    </div>
  )
}

export default observer(InjectConfig)
