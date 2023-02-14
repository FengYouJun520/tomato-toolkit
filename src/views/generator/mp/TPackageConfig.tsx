import { FC } from 'react'
import {
  Form,
  Input,
  Button,
  Grid,
} from '@arco-design/web-react'
import { PackageConfig } from '@/types/type'
import { useStore } from '@/store'
import { initialPackageConfig } from '@/store/mp/package'

const TPackageConfig: FC = () => {
  const store = useStore()
  const [form] = Form.useForm<PackageConfig>()

  reaction(() => store.mp.packageStore.isHydrated, (isHydrated) => {
    if(isHydrated) {
      form.setFieldsValue({...store.mp.packageStore.package})
    }
  })

  return (
    <Form<PackageConfig>
      colon
      form={form}
      layout="vertical"
      initialValues={store.mp.packageStore.package}
      onValuesChange={(v) => {
        store.mp.packageStore.setPackageConfig(v)
      }}
    >
      <Form.Item>
        <Button status="warning" onClick={() => {
          form.setFieldsValue(initialPackageConfig)
        }}>
          重置
        </Button>
      </Form.Item>

      <Grid.Row gutter={24}>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="父包名" field="parent">
            <Input placeholder="请输入" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="父模块包名" field="moduleName">
            <Input placeholder="请输入" />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={12}>
          <Form.Item label="Entity包名" field="entity">
            <Input placeholder="请输入" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="Controller包名" field="controller">
            <Input placeholder="请输入" />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={12}>
          <Form.Item label="Service包名" field="service">
            <Input placeholder="请输入" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="ServiceImpl包名" field="serviceImpl">
            <Input placeholder="请输入" />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={12}>
          <Form.Item label="Mapper包名" field="mapper">
            <Input placeholder="请输入" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="MapperXml包名" field="xml">
            <Input placeholder="请输入" />
          </Form.Item>
        </Grid.Col>
      </Grid.Row>
    </Form>
  )
}

export default observer(TPackageConfig)
