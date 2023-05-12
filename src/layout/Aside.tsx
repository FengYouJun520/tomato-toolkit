import { FC } from 'react'
import { Layout, Menu, SiderProps } from '@arco-design/web-react'
import { observer } from 'mobx-react-lite'
import { useStore } from '@/store'
import { routes } from '@/router'
import { RouteMenu } from '@/router/type'
import { resolvePath, useLocation, useMatches, useNavigate } from 'react-router-dom'
import './aside.css'

export type AsideProps = SiderProps

const Aside: FC<AsideProps> = (props) => {
  const store = useStore()
  const location = useLocation()
  const navigate = useNavigate()
  const matches = useMatches()

  const [active, setActive] = useState('/')
  useEffect(() => {
    setActive(location.pathname)
  }, [location])


  const generateMenu = (routes: RouteMenu[], parent = '') => {
    return routes.map(route => {
      const { pathname } = resolvePath({pathname: route.path}, parent)

      return route.children
        ?
        <Menu.SubMenu
          _key={pathname}
          key={pathname}
          title={<>
            {route.handle.icon && route.handle.icon}
            {route.handle.title}
          </>}
        >
          {generateMenu(route.children, pathname)}
        </Menu.SubMenu>
        :
        <Menu.Item
          _key={pathname}
          key={pathname}
        >
          {route.handle.icon && route.handle.icon}
          {route.handle.title}
        </Menu.Item>
    })
  }

  const handleClickMenuItem = (key: string) => {
    navigate(key)
  }

  return (
    <Layout.Sider
      {...props}
      collapsed={store.ui.collapse}
      className="h-full fixed"
      style={{
        width: store.ui.headerWidth,
      }}
    >
      <Menu
        className="w-full h-full"
        onClickMenuItem={handleClickMenuItem}
        defaultOpenKeys={matches.map(match => match.pathname)}
        selectedKeys={[active]}
        tabIndex={1002}
      >
        {generateMenu(routes)}
      </Menu>
    </Layout.Sider>
  )
}

export default observer(Aside)
