import { FC } from 'react'
import {  Layout, Button, Drawer, Radio, Typography, Icon, Divider } from '@arco-design/web-react'
import {
  IconMenuFold,
  IconMenuUnfold,
  IconSettings,
  IconSunFill,
  IconMoonFill,
  IconDesktop,
} from '@arco-design/web-react/icon'
import { observer } from 'mobx-react-lite'
import { useStore } from '@/store'
import Breadcrumb from './Breadcrumb'
import './header.css'

const getTitle = (theme: string) => {
  switch(theme) {
  case 'system':
    return '系统'
  case 'dark':
    return '暗黑'
  default:
    return '亮色'
  }
}

const Header: FC = () => {
  const store = useStore()
  const [visibleSetting, setVisibleSetting] = useState(false)

  return (
    <>
      <Layout.Header
        className="h-[64px] fixed top-0 z-[1001]"
        style={{
          width: `calc(100% - ${store.ui.headerWidth}px)`,
          transition: 'width .2s cubic-bezier(0.34, 0.69, 0.1, 1)',
        }}
      >
        <div
          className="h-full flex justify-between items-center px-5"
          style={{
            backgroundColor: 'var(--color-bg-2)',
            borderBottom: '1px solid var(--color-border)',
            boxSizing: 'border-box',
          }}
        >
          <div className="flex items-center">
            <Button
              type="text"
              className="!text-[var(--color-text-1)]"
              onClick={store.ui.toggleCollapse}
              icon={store.ui.collapse
                ?
                <IconMenuUnfold fontSize={20} />
                :
                <IconMenuFold fontSize={20} />}
            />

            <Breadcrumb />
          </div>

          <Button
            type="text"
            icon={<IconSettings fontSize={20} />}
            className="!text-[var(--color-text-2)]"
            onClick={() => setVisibleSetting(true)}
          />
        </div>
      </Layout.Header>

      <Drawer
        visible={visibleSetting}
        width="30%"
        title="设置"
        onCancel={() => setVisibleSetting(false)}
        footer={null}
      >
        <div>
          <Divider>
            <Typography.Title heading={5}>主题设置</Typography.Title>
          </Divider>
          <Radio.Group
            value={store.ui.theme}
            onChange={(value) => store.ui.changeTheme(value)}
            name='card-radio-group'
            className="w-full !inline-flex items-center justify-between"
          >
            {['light', 'dark', 'system'].map((item) => {
              return (
                <Radio key={item} value={item}>
                  {({ checked }) => {
                    return (<div className="flex flex-col space-y-2">
                      <div
                        className={`custom-radio-card ${checked ? 'custom-radio-card-checked' : ''}`}
                      >
                        <Icon fontSize={54}>
                          {item === 'system'
                            ?
                            <IconDesktop />
                            : item === 'light' ? <IconSunFill />
                              : <IconMoonFill />
                          }
                        </Icon>
                      </div>
                      <Typography.Text className="text-center">
                        {getTitle(item)}
                      </Typography.Text>
                    </div>
                    )
                  }}
                </Radio>
              )
            })}
          </Radio.Group>
        </div>
      </Drawer>
    </>
  )
}

export default observer(Header)
