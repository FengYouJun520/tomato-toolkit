
import {
  createBrowserRouter,
  RouteObject,
} from 'react-router-dom'
import {
  IconHome,
  IconDashboard,
  IconCodeSandbox,
  IconCodeBlock,
} from '@arco-design/web-react/icon'
import Layout from '@/layout'
import Dashboard from '@/views/dashboard'
import MpGenerator from '@/views/generator/mp'
import { RouteMenu } from './type'

export const Home: RouteMenu = {
  path: '/',
  element: <Layout />,
  handle: {
    title: '首页',
    icon: <IconHome />,
    submenu: true,
  },
}

export const routes: RouteMenu[] = [
  {
    ...Home,
    children: [
      {
        index: true,
        element: <Dashboard />,
        handle: {
          title: 'Dashboard',
          icon: <IconDashboard />,
        },
      },
    ],
  },
  {
    path: '/generator',
    element: <Layout />,
    handle: {
      title: '代码生成器',
      icon: <IconCodeSandbox />,
      submenu: true,
    },
    children: [
      {
        path: 'mp',
        handle: {
          title: 'MyBatisPlus生成器',
          icon: <IconCodeBlock />,
        },
        element: <MpGenerator />,
      },
    ],
  },
]

const router = createBrowserRouter(routes.map(route => ({...route} as RouteObject)))

export default router
