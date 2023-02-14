import { FC } from 'react'
import {
  Form,
  Button,
  Radio,
  Select,
  Grid,
  Space,
  Tabs,
  Tag,
  Message,
  Modal,
} from '@arco-design/web-react'
import { useStore } from '@/store'
import { initialStrategyBaseConfig, StrategyBaseConfig } from '@/store/mp/strategy'
import { renderRadio } from '@/utils/renderRadio'
import EntityConfig from './EntityConfig'
import ControllerConfig from './ControllerConfig'
import ServiceConfig from './ServiceConfig'
import MapperConfig from './MapperConfig'
import { BasicTableInfo } from '@/types/type'
import { invoke } from '@tauri-apps/api'
import Editor from '@monaco-editor/react'
import { IconLoading } from '@arco-design/web-react/icon'

const TStrategyConfig: FC = () => {
  const store = useStore()
  const [form] = Form.useForm<StrategyBaseConfig>()
  const [tableForm] = Form.useForm<{include: string[], exclude: string[]}>()
  const [basicTables, setBasicTables] = useState<BasicTableInfo[]>([])

  reaction(() => store.mp.strategyStore.isHydrated, (isHydrated) => {
    if(isHydrated) {
      form.setFieldsValue({...store.mp.strategyStore.strategyBase})
    }
  })

  useEffect(() => {
    setBasicTables(store.mp.basicTableInfos)
  }, [store.mp.basicTableInfos])


  const [visiblePreview, setVisiblePreview] = useState(false)
  const [previewData, setPreviewData] = useState<Record<string, any>>({})
  const code = JSON.stringify(previewData, null, 2)
  const handlePreviewCustomData = async () => {
    try {
      const data = await invoke<Record<string, any>>('generate_preview', {
        config: store.mp.mpConfig,
      })
      setPreviewData(data)
      setVisiblePreview(true)
    } catch (error) {
      Message.error(error as string)
    }
  }

  const handleSelectAll = () => {
    store.mp.strategyStore.setTables({
      include: store.mp.basicTableInfos.map(table => table.name),
      exclude: [],
    })
    tableForm.setFieldValue('include', store.mp.basicTableInfos.map(table => table.name))
  }

  return (
    <>
      <Form<StrategyBaseConfig>
        colon
        form={form}
        layout="vertical"
        initialValues={store.mp.strategyStore.strategyBase}
        onValuesChange={(_, vs) => {
          store.mp.strategyStore.setStrategyBaseConfig(vs)
        }}
      >
        <Form.Item>
          <Space>
            <Button status="warning" onClick={() => {
              form.setFieldsValue(initialStrategyBaseConfig)
              store.mp.strategyStore.resetSelectTable()
              store.mp.strategyStore.setTables({include: [], exclude: []})
              tableForm.setFieldsValue({
                include: store.mp.strategyStore.include,
                exclude: store.mp.strategyStore.exclude,
              })
            }}>
              重置
            </Button>
            <Button type="primary" disabled={!store.mp.isInclude} onClick={handlePreviewCustomData} >
              预览生成的数据
            </Button>
          </Space>
        </Form.Item>

        <Grid.Row gutter={24}>
          <Grid.Col xs={12} md={8}>
            <Form.Item label="开启大写命名" field="isCapitalMode">
              <Radio.Group type="button">
                {renderRadio()}
              </Radio.Group>
            </Form.Item>
          </Grid.Col>
          <Grid.Col xs={12} md={8}>
            <Form.Item label="开启sql过滤" field="enableSqlFilter">
              <Radio.Group type="button">
                {renderRadio()}
              </Radio.Group>
            </Form.Item>
          </Grid.Col>

          <Grid.Col xs={12} md={8}>
            <Form.Item label="开启Schema" field="enableSchema">
              <Radio.Group type="button">
                {renderRadio()}
              </Radio.Group>
            </Form.Item>
          </Grid.Col>
        </Grid.Row>
      </Form>

      <Form
        form={tableForm}
        colon
        layout="vertical"
        initialValues={{
          include: store.mp.strategyStore.include,
          exclude: store.mp.strategyStore.exclude,
        }}
        onValuesChange={(_, vs) => {
          store.mp.strategyStore.setTables(vs)
        }}
      >

        <Form.Item label="包含的表">
          <Grid.Row>
            <Grid.Col span={20}>
              <Form.Item field="include">
                <Select
                  placeholder="请选择"
                  mode="multiple"
                  dragToSort
                  allowClear
                  options={basicTables.map(option => ({
                    label: (
                      <div className="flex justify-between items-center px-3">
                        <span>{option.name}</span>
                        <Tag className="ml-2" color="#00b42a">{option.comment}</Tag>
                      </div>
                    ),
                    value: option.name,
                    extra: {
                      comment: option.comment,
                    },
                  }))}
                  renderFormat={(option) => (
                    <div className="flex items-center">
                      <Tag color="#00b42a" closable>
                        <span>{option?.value}</span>
                        <span className="ml-3">{option?.extra.comment}</span>
                      </Tag>
                    </div>
                  )}
                />
              </Form.Item>
            </Grid.Col>
            <Grid.Col span={4}>
              <Form.Item>
                <Button long type="primary" onClick={handleSelectAll}>全选</Button>
              </Form.Item>
            </Grid.Col>
          </Grid.Row>
        </Form.Item>

        <Form.Item label="排除的表" field="exclude">
          <Select placeholder="请选择" mode="multiple" />
        </Form.Item>
      </Form>

      <Tabs
        type="rounded"
        animation
        lazyload
        destroyOnHide={false}
        size="large"
        defaultActiveTab="EntityConfig"
      >
        <Tabs.TabPane title="Entity配置" key="EntityConfig">
          <EntityConfig />
        </Tabs.TabPane>
        <Tabs.TabPane title="Controller配置" key="ControllerConfig">
          <ControllerConfig />
        </Tabs.TabPane>
        <Tabs.TabPane title="Service配置" key="ServiceConfig">
          <ServiceConfig />
        </Tabs.TabPane>
        <Tabs.TabPane title="Mapper配置" key="MapperConfig">
          <MapperConfig />
        </Tabs.TabPane>
      </Tabs>

      <Modal
        title="预览生成的数据"
        unmountOnExit
        visible={visiblePreview}
        hideCancel
        onOk={() => setVisiblePreview(false)}
        onCancel={() => setVisiblePreview(false)}
        style={{width: '80%', height: '90vh'}}
      >
        <Editor
          language="json"
          height="68vh"
          options={{
            fontSize: 18,
            folding: true,
          }}
          theme="vs-dark"
          defaultValue={code}
          loading={<IconLoading spin />}
        />
      </Modal>
    </>
  )
}

export default observer(TStrategyConfig)
