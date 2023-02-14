import { FC } from 'react'
import { Outlet, useLocation } from 'react-router-dom'
import { BackTop, Layout as ArcoLayout } from '@arco-design/web-react'
import { SwitchTransition, CSSTransition } from 'react-transition-group'
import Header from './Header'
import Aside from './Aside'
import { useStore } from '@/store'

const { Content } = ArcoLayout

const Layout: FC = () => {
  const store = useStore()
  const location = useLocation()
  return (
    <>
      <ArcoLayout hasSider className="h-full w-full">
        <Aside />
        <ArcoLayout
          className="h-full"
          style={{
            paddingLeft: store.ui.collapse ? 48 : store.ui.asideWidth,
            transition: 'padding-left .2s cubic-bezier(0.34, 0.69, 0.1, 1)',
          }}
        >
          <Header />
          <ArcoLayout id="layout-content">
            <Content style={{
              paddingTop: 64,
              backgroundColor: 'var(--color-fill-2)',
            }}>
              <div className="px-5 py-4 h-full">
                <SwitchTransition mode="out-in">
                  <CSSTransition
                    key={location.key}
                    timeout={200}
                    classNames="fade"
                    nodeRef={null}
                  >
                    <Outlet />
                  </CSSTransition>
                </SwitchTransition>
              </div>
            </Content>
          </ArcoLayout>
        </ArcoLayout>
      </ArcoLayout>
      <BackTop
        visibleHeight={128}
        style={{right: 64, bottom: 64}}
      />
    </>
  )
}

export default observer(Layout)
