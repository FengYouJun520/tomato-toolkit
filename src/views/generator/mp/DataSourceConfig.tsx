
import {
  Form,
  Image,
  Input,
  InputNumber,
  Select,
  Button,
  Space,
  Grid,
  Message,
} from '@arco-design/web-react'
import mysqlIcon from '@/assets/mysql.svg'
import sqliteIcon from '@/assets/sqlite.svg'
import mssqlIcon from '@/assets/mssql.svg'
import postgresIcon from '@/assets/postgres.svg'
import { LabeledValue, OptionInfo } from '@arco-design/web-react/es/Select/interface'
import { useStore } from '@/store'
import { BasicTableInfo, DatasourceConfig } from '@/types/type'
import { IconLock, IconUser, IconSend } from '@arco-design/web-react/icon'
import { initDataSourceConfig } from '@/store/mp/datasource'
import { FC } from 'react'
import { invoke } from '@tauri-apps/api'

const getIcon = (value: string) => {
  switch (value) {
  case 'mysql':
    return mysqlIcon
  case 'sqlite':
    return sqliteIcon
  case 'sqlserver':
    return mssqlIcon
  case 'postgressql':
    return postgresIcon
  default:
    return mysqlIcon
  }
}

const options = [
  {
    label: (
      <div className="flex items-center space-x-3">
        <Image src={mysqlIcon} width={30} preview={false} />
        <span>mysql</span>
      </div>
    ),
    value: 'mysql',
  },
  {
    label: (
      <div className="flex items-center space-x-3">
        <Image src={sqliteIcon} width={30} preview={false} />
        <span>sqlite（大致做完了，但不一定正确</span>
      </div>
    ),
    value: 'sqlite',
  },
  {
    label: (
      <div className="flex items-center space-x-3">
        <Image src={mssqlIcon} width={30} preview={false} />
        <span>sqlserver</span>
      </div>
    ),
    value: 'sqlserver',
    disabled: true,
  },
  {
    label: (
      <div className="flex items-center space-x-3">
        <Image src={postgresIcon} width={30} preview={false} />
        <span>postgressql</span>
      </div>
    ),
    value: 'postgressql',
    disabled: true,
  },
]

const DataSourceConfig: FC = () => {
  const store = useStore()
  const [form] = Form.useForm <DatasourceConfig>()

  reaction(() => store.mp.dataSourceStore.isHydrated, (isHydrated) => {
    if(isHydrated) {
      form.setFieldsValue({...store.mp.dataSourceStore.dataSource})
    }
  })

  const renderFormat = (option: OptionInfo | null, value: string | number | LabeledValue) => {
    return (
      <div className="flex items-center space-x-3">
        <Image src={getIcon(value as string)} width={30} preview={false} />
        {option ? option.value : value as string}
      </div>
    )
  }

  const handleTestConnection = async () => {
    try {
      const basicTableInfos =  await invoke<BasicTableInfo[]>('test_connection',{
        config: store.mp.dataSourceStore.dataSource,
      })

      store.mp.setBasicTableInfos(basicTableInfos)
      Message.success('测试成功，请到策略配置选择要生成的表')
    } catch (error) {
      Message.error(error as string)
    }
  }

  return (
    <Form<DatasourceConfig>
      form={form}
      colon
      layout="vertical"
      initialValues={store.mp.dataSourceStore.dataSource}
      onValuesChange={(v) => {
        store.mp.dataSourceStore.setDataSourceConfig(v)
      }}
    >

      <Grid.Row gutter={24}>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="数据库类型" field="type">
            <Select
              placeholder="请选择"
              options={options}
              renderFormat={renderFormat}
            />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="数据库名称" field="database">
            <Input placeholder="请输入数据库名称" />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={12}>
          <Form.Item label="地址" field="host">
            <Input placeholder="请输入地址" />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="端口号" field="port">
            <InputNumber placeholder="请输入端口号" min={0} max={65535} />
          </Form.Item>
        </Grid.Col>

        <Grid.Col xs={24} md={12}>
          <Form.Item label="用户名" field="username">
            <Input placeholder="请输入用户名" prefix={<IconUser />} />
          </Form.Item>
        </Grid.Col>
        <Grid.Col xs={24} md={12}>
          <Form.Item label="密码" field="password">
            <Input.Password placeholder="请输入密码" prefix={<IconLock />} />
          </Form.Item>
        </Grid.Col>
      </Grid.Row>

      <Form.Item wrapperCol={{ offset: 8 }}>
        <Space>
          <Button
            type='primary'
            htmlType='submit'
            icon={<IconSend />}
            onClick={handleTestConnection}
          >
              测试
          </Button>
          <Button
            status="danger"
            onClick={() => {
              form.setFieldsValue(initDataSourceConfig)
            }}
          >
              重置
          </Button>
        </Space>
      </Form.Item>
    </Form>
  )
}

export default observer(DataSourceConfig)
