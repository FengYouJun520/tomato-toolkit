import { FC } from 'react'
import { Layout, Menu, SiderProps } from '@arco-design/web-react'
import { observer } from 'mobx-react-lite'
import { useStore } from '@/store'
import { routes } from '@/router'
import { RouteMenu } from '@/router/type'
import { useLocation, useMatches, useNavigate } from 'react-router-dom'
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


  const generateMenu = (routes: RouteMenu[], parent = '', level = 1) => {
    return routes.map(route => {
      const key = route.index ? parent : level === 1 ? `${parent}${route.path}` : `${parent}/${route.path}`

      return route.children
        ?
        <Menu.SubMenu
          _key={key}
          key={key}
          title={<>
            {route.handle.icon && route.handle.icon}
            {route.handle.title}
          </>}
        >
          {generateMenu(route.children, key, level+1)}
        </Menu.SubMenu>
        :
        <Menu.Item
          _key={key}
          key={key}
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
      width={store.ui.asideWidth}
      collapsed={store.ui.collapse}
      className="h-full fixed"
      style={{
        width: store.ui.collapse ? 48 : store.ui.asideWidth,
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
