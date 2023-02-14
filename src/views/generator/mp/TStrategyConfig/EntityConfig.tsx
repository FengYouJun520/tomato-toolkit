import { useStore } from '@/store'
import { defaultEntity } from '@/store/mp/strategy'
import { Entity } from '@/types/type'
import { renderRadio } from '@/utils/renderRadio'
import {
  Button,
  Form, Grid, Input, InputTag, Radio, Select, Space,
} from '@arco-design/web-react'
import { IconDelete, IconPlus } from '@arco-design/web-react/icon'
import { FC } from 'react'

const idTypeOptions = [
  {
    label: 'AUTO',
    value: 'AUTO',
  },
  {
    label: 'NONE',
    value: 'NONE',
  },
  {
    label: 'INPUT',
    value: 'INPUT',
  },
  {
    label: 'ASSIGN_ID',
    value: 'ASSIGN_ID',
  },
  {
    label: 'ASSIGN_UUID',
    value: 'ASSIGN_UUID',
  },
]

const fieldFillOptions = [
  {
    label: 'DEFAULT',
    value: 'DEFAULT',
  },
  {
    label: 'INSERT',
    value: 'INSERT',
  },
  {
    label: 'UPDATE',
    value: 'UPDATE',
  },
  {
    label: 'INSERT_UPDATE',
    value: 'INSERT_UPDATE',
  },
]

const EntityConfig: FC = () => {
  const store = useStore()
  const [form] = Form.useForm<Entity>()

  reaction(() => store.mp.strategyStore.isHydrated, (isHydrated) => {
    if(isHydrated) {
      form.setFieldsValue({...store.mp.strategyStore.entity})
    }
  })

  // 渲染表填充字段
  const renderTableFillList = (
    add: (defaultValue?: any, index?: number) => void,
    remove: (index: number) => void,
    item: {key: number, field: string},
    index: number,
  ) => (
    <Grid.Row  gutter={24}>
      <Grid.Col xs={10} md={10} >
        <Form.Item
          field={item.field + '.propertyName'}
          rules={[{ required: true }]}
          noStyle
        >
          <Input placeholder="例如：create_time, update_time" />
        </Form.Item>
      </Grid.Col>
      <Grid.Col xs={10} md={10}>
        <Form.Item
          field={item.field + '.fieldFill'}
          rules={[{ required: true }]}
          noStyle
        >
          <Select options={fieldFillOptions} />
        </Form.Item>
      </Grid.Col>
      <Grid.Col xs={4} md={4}>
        <Space>
          <Button
            icon={<IconPlus />}
            onClick={() => {add({propertyName: '',fieldFill: 'INSERT'})}}
          />
          <Button
            icon={<IconDelete />}
            shape='circle'
            status='danger'
            onClick={() => remove(index)}
          />
        </Space>
      </Grid.Col>
    </Grid.Row>
  )
  return (
    <Form<Entity>
      colon
      form={form}
      layout="vertical"
      initialValues={store.mp.strategyStore.entity}
      onValuesChange={(_, vs) => {
        store.mp.strategyStore.setEntityConfig(vs)
      }}
    >
      <Form.Item>
        <Button status="warning" onClick={() => {form.setFieldsValue(defaultEntity)}}>
          重置
        </Button>
      </Form.Item>

      <Grid.Row gutter={24}>
        <Grid.Col xs={12} md={8}>
          <Form.Item label="覆盖文件" field="fileOverride">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={12} md={8}>
          <Form.Item label="启动SerialVersionUID" field="serialVersionUid">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={12} md={8}>
          <Form.Item label="开启链式模型" field="chainMode">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={12} md={8}>
          <Form.Item label="生成字段常量" field="columnConstant">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={12} md={8}>
          <Form.Item label="开启lombok模型" field="lombok">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={12} md={8}>
          <Form.Item label="Boolean类型移除is前缀" field="booleanColumnRemoveIsPrefix">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={12} md={8}>
          <Form.Item label="生成实体时生成字段注解" field="enableTableFieldAnnotation">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={12}  md={8}>
          <Form.Item label="开启ActiveRecord模型" field="activeRecord">
            <Radio.Group type="button">
              {renderRadio()}
            </Radio.Group>
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={12}>
          <Form.Item label="全局主键类型" field="idType">
            <Select options={idTypeOptions} />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={12}>
          <Form.Item label="数据库表映射到实体的命名策略" field="naming">
            <Select options={[
              {
                label: '无操作',
                value: 'NoChange',
              },
              {
                label: '下划线转驼峰',
                value: 'UnderlineToCamel',
              },
            ]} />
          </Form.Item>
        </Grid.Col>

        <Grid.Col span={24}>
          <Form.Item label="父类包名" field="superClass">
            <Input placeholder="示例: com.baomidou.global.BaseEntity" />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={12}>
          <Form.Item label="乐观锁字段名" field="versionColumnName">
            <Input placeholder="例如：version" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="乐观锁属性名" field="versionPropertyName">
            <Input placeholder="例如：version" />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={12}>
          <Form.Item label="逻辑删除字段名" field="logicDeleteColumnName">
            <Input placeholder="例如：is_deleted" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="逻辑删除属性名" field="logicDeletePropertyName">
            <Input placeholder="例如：isDeleted" />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={24}>
          <Form.Item label="添加父类公共字段" field="superEntityColumns">
            <InputTag
              allowClear
              placeholder="例如: id, is_deleted, create_time，按回车确定"
            />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={24}>
          <Form.Item label="添加忽略字段" field="ignoreColumns">
            <InputTag
              allowClear
              placeholder="例如：create_time, update_time，按回车确定"
            />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={24}>
          <Form.Item label="格式化文件名称" field="formatFilename">
            <Input placeholder="示例: {}Entity, {}表示占位符"/>
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={24}>
          <Form.Item label="添加填充字段">
            <Form.List field='tableFillList'>
              {(fields, { add, remove  }) => (<>
                {fields.length ? fields.map((item, index) => (
                  <Form.Item key={item.key}>
                    {renderTableFillList(add, remove, item, index)}
                  </Form.Item>
                ))
                  :
                // 如果数据为空
                  <Form.Item>
                    <Button
                      long
                      onClick={() => {
                        add({propertyName: '',fieldFill: 'INSERT'})
                      }}
                    >
                    添加
                    </Button>
                  </Form.Item>
                }
              </>)}
            </Form.List>
          </Form.Item>
        </Grid.Col>
      </Grid.Row>
    </Form>
  )
}

export default observer(EntityConfig)
