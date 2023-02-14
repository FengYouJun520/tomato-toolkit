import { FC } from 'react'
import {
  Form,
  Input,
  Select,
  Radio,
  Button,
  Grid,
  Message,
} from '@arco-design/web-react'
import { GlobalConfig } from '@/types/type'
import { renderRadio } from '@/utils/renderRadio'
import { useStore } from '@/store'
import { initialGlobalConfig } from '@/store/mp/global'
import { dialog } from '@tauri-apps/api'

const dateTypeOptions = [
  {
    label: 'ONLY_DATE',
    value: 'ONLY_DATE',
  },
  {
    label: 'SQL_PACK',
    value: 'SQL_PACK',
  },
  {
    label: 'TIME_PACK',
    value: 'TIME_PACK',
  },
]

const TGlobalConfig: FC = () => {
  const store = useStore()
  const [form] = Form.useForm<GlobalConfig>()

  const selectDirectory = async () => {
    try {
      const outputDir = await dialog.open({
        title: '选择输出目录',
        directory: true,
        defaultPath: store.mp.outputDir,
      })

      if(outputDir) {
        form.setFieldValue('outputDir', outputDir as string)
      }
    } catch (error) {
      Message.error(error as string)
    }
  }

  reaction(() => store.mp.globalStore.isHydrated, (isHydrated) => {
    if(isHydrated) {
      form.setFieldsValue({...store.mp.globalStore.global})
    }
  })

  return (
    <Form<GlobalConfig>
      colon
      form={form}
      layout="vertical"
      initialValues={store.mp.globalStore.global}
      onValuesChange={(v) => {
        store.mp.globalStore.setGlobalConfig(v)
      }}
    >
      <Form.Item>
        <Button status="warning" onClick={() => {
          form.setFieldsValue(initialGlobalConfig)
        }}>
          重置
        </Button>
      </Form.Item>

      <Form.Item label="作者" field="author">
        <Input placeholder="请输入" />
      </Form.Item>

      <Grid.Row gutter={24}>
        <Grid.Col xs={12} md={12}>
          <Form.Item label="打开输出目录"  field="open">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={12} md={12}>
          <Form.Item label="开启swagger"  field="swagger">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={12} md={12}>
          <Form.Item label="开启springdoc"  field="springdoc">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={12} md={12}>
          <Form.Item label="开启kotlin模式" field="kotlin">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={12} md={12}>
          <Form.Item label="注释日期格式" field="commentDate">
            <Input  />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={12} md={12}>
          <Form.Item label="时间策略" field="dateType">
            <Select options={dateTypeOptions} />
          </Form.Item>
        </Grid.Col>
      </Grid.Row>

      <Form.Item itemRef="outputDir" label="输出目录">
        <Grid.Row>
          <Grid.Col xs={18} md={20}>
            <Form.Item field="outputDir">
              <Input />
            </Form.Item>
          </Grid.Col>
          <Grid.Col xs={6} md={4}>
            <Form.Item>
              <Button
                long
                type="primary"
                onClick={selectDirectory}
              >
                选择目录
              </Button>
            </Form.Item>
          </Grid.Col>
        </Grid.Row>
      </Form.Item>
    </Form>
  )
}

export default observer(TGlobalConfig)
