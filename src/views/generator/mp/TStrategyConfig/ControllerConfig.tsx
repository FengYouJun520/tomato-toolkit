import { useStore } from '@/store'
import { defaultController } from '@/store/mp/strategyConfig'
import { Controller } from '@/types/type'
import { renderRadio } from '@/utils/renderRadio'
import {
  Button,
  Form, Grid, Input, Radio,
} from '@arco-design/web-react'
import { FC } from 'react'

const ControllerConfig: FC = () => {
  const store = useStore()
  const [form] = Form.useForm<Controller>()

  reaction(() => store.mp.strategyStore.isHydrated, (isHydrated) => {
    if(isHydrated) {
      form.setFieldsValue({...store.mp.strategyStore.controller})
    }
  })
  return (
    <Form
      colon
      form={form}
      layout="vertical"
      initialValues={store.mp.strategyStore.controller}
      onValuesChange={(_, vs) => {
        store.mp.strategyStore.setControllerConfig(vs)
      }}
    >
      <Form.Item>
        <Button status="warning" onClick={() => {form.setFieldsValue(defaultController)}}>
          重置
        </Button>
      </Form.Item>

      <Grid.Row gutter={24}>
        <Grid.Col xs={24} md={8}>
          <Form.Item label="覆盖文件" field="fileOverride">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={8}>
          <Form.Item label="开启驼峰转连字符" field="hyphenStyle">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={8}>
          <Form.Item label="开启生成@RestController" field="restStyle">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={12}>
          <Form.Item label="父类包名" field="superClass">
            <Input placeholder="示例: com.baomidou.global.BaseController" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="格式化文件名称" field="formatFilename">
            <Input placeholder="示例: {}Controller, {}表示占位符" />
          </Form.Item>
        </Grid.Col>
      </Grid.Row>
    </Form>
  )
}

export default observer(ControllerConfig)
