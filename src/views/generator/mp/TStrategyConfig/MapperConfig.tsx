import { useStore } from '@/store'
import { defaultMapper } from '@/store/mp/strategy'
import { Mapper } from '@/types/type'
import { renderRadio } from '@/utils/renderRadio'
import {
  Button,
  Form, Grid, Input, Radio,
} from '@arco-design/web-react'
import { FC } from 'react'

const MapperConfig: FC = () => {
  const store = useStore()
  const [form] = Form.useForm<Mapper>()

  reaction(() => store.mp.strategyStore.isHydrated, (isHydrated) => {
    if(isHydrated) {
      form.setFieldsValue({...store.mp.strategyStore.mapper})
    }
  })
  return (
    <Form<Mapper>
      colon
      form={form}
      layout="vertical"
      initialValues={store.mp.strategyStore.mapper}
      onValuesChange={(_, vs) => {
        store.mp.strategyStore.setMapperConfig(vs)
      }}
    >
      <Form.Item>
        <Button status="warning" onClick={() => {form.setFieldsValue(defaultMapper)}}>
          重置
        </Button>
      </Form.Item>

      <Grid.Row gutter={24}>
        <Grid.Col xs={24} md={24}>
          <Form.Item label="覆盖文件" field="fileOverride">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>


        <Grid.Col xs={24} md={12}>
          <Form.Item label="设置service接口父类" field="superServiceClass">
            <Input placeholder="示例: com.baomidou.global.BaseService" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="设置service实现类父类" field="superServiceImplClass">
            <Input placeholder="示例: com.baomidou.global.BaseServiceImpl" />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={12}>
          <Form.Item label="格式化service接口文件名称" field="formatServiceFilename">
            <Input placeholder="示例: {}Service, {}表示占位符" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="格式化service实现类文件名称" field="formatServiceImplFilename">
            <Input placeholder="示例: {}ServiceImpl, {}表示占位符" />
          </Form.Item>
        </Grid.Col>
      </Grid.Row>
    </Form>
  )
}

export default observer(MapperConfig)
