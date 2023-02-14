import { FC } from 'react'
import {
  Form,
  Input,
  Button,
  Grid,
  Space,
  Message,
} from '@arco-design/web-react'
import { TemplateConfig } from '@/types/type'
import { useStore } from '@/store'
import { initialTemplateConfig } from '@/store/mp/templateConfig'
import { path, shell } from '@tauri-apps/api'

const TTemplateConfig: FC = () => {
  const store = useStore()
  const [form] = Form.useForm<TemplateConfig>()

  reaction(() => store.mp.templateStore.isHydrated, (isHydrated) => {
    if(isHydrated) {
      form.setFieldsValue({...store.mp.templateStore.template})
    }
  })

  const openTemplateDirectory = async () => {
    try {
      const resource = await path.resolveResource('templates')
      await shell.open(resource)
    } catch (error) {
      Message.error(error as string)
    }
  }

  return (
    <Form<TemplateConfig>
      colon
      form={form}
      layout="vertical"
      initialValues={store.mp.templateStore.template}
      onValuesChange={(v) => {
        store.mp.templateStore.setTemplateConfig(v)
      }}
    >
      <Form.Item>
        <Space>
          <Button status="warning" onClick={() => {
            form.setFieldsValue(initialTemplateConfig)
          }}>
            重置
          </Button>
          <Button type="primary" onClick={openTemplateDirectory}>
            打开模板文件目录
          </Button>
        </Space>
      </Form.Item>

      <Grid.Row gutter={24}>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="Entity模板" field="entity">
            <Input placeholder="默认为: templates/entity.java" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="EntityKotlin模板" field="entityKt">
            <Input placeholder="默认为: templates/entity.kt.java" />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={12}>
          <Form.Item label="Mapper模板" field="mapper">
            <Input placeholder="默认为: templates/mapper.java" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="Xml模板" field="xml">
            <Input placeholder="默认为: templates/mapper.xml" />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={12}>
          <Form.Item label="Service模板" field="service">
            <Input placeholder="默认为: templates/service.java" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="ServiceImpl模板" field="serviceImpl">
            <Input placeholder="默认为: templates/servicerImpl.java" />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={12}>
          <Form.Item label="Controller模板" field="controller">
            <Input placeholder="默认为: templates/controller.java" />
          </Form.Item>
        </Grid.Col>
      </Grid.Row>
    </Form>
  )
}

export default observer(TTemplateConfig)
