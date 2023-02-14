import {
  Button,
  Card,
  Image,
  Message,
  Tabs,
  Typography,
} from '@arco-design/web-react'
import { FC } from 'react'
import mybatisIcon from '@/assets/mybatis.svg'
import DataSourceConfig from './DataSourceConfig'
import TGlobalConfig from './TGlobalConfig'
import TPackageConfig from './TPackageConfig'
import TemplateConfig from './TemplateConfig'
import InjectConfig from './TInjectConfig'
import TStrategyConfig from './TStrategyConfig'
import { useStore } from '@/store'
import { invoke } from '@tauri-apps/api'

const MyBatisPlusGenerator: FC = () => {
  const store = useStore()

  const generateMyBatisPlus = async () => {
    try {
      await invoke('mp_codegen', {config: store.mp.mpConfig})
      Message.success('代码生成成功')
    } catch (error) {
      Message.error(error as string)
    }
  }

  return (
    <Card
      className="h-full p-3"
      cover={<>
        <div className="flex items-center space-x-3 px-4">
          <Image
            src={mybatisIcon}
            width={64}
            preview={false}
          />
          <Typography.Title heading={4}>MyBatisPlus代码生成器</Typography.Title>
        </div>
      </>}
    >
      <DataSourceConfig />

      <Tabs
        type="rounded"
        animation
        lazyload
        destroyOnHide={false}
        size="large"
        defaultActiveTab="GlobalConfig"
      >
        <Tabs.TabPane title="全局配置" key="GlobalConfig">
          <TGlobalConfig />
        </Tabs.TabPane>
        <Tabs.TabPane title="包配置" key="PackageConfig">
          <TPackageConfig />
        </Tabs.TabPane>
        <Tabs.TabPane title="模板配置" key="TemplateConfig">
          <TemplateConfig />
        </Tabs.TabPane>
        <Tabs.TabPane title="注入配置" key="InjectConfig">
          <InjectConfig />
        </Tabs.TabPane>
        <Tabs.TabPane title="策略配置" key="StrategyConfig">
          <TStrategyConfig />
        </Tabs.TabPane>
      </Tabs>

      <Button
        long
        type="primary"
        size="large"
        className="mt5"
        disabled={!store.mp.isInclude}
        onClick={generateMyBatisPlus}
      >
        生成
      </Button>
    </Card>
  )
}

export default observer(MyBatisPlusGenerator)
