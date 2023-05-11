
import {
  createBrowserRouter,
  RouteObject,
} from 'react-router-dom'
import {
  IconHome,
  IconDashboard,
  IconCodeSandbox,
  IconCodeBlock,
  IconLock,
} from '@arco-design/web-react/icon'
import Layout from '@/layout'
import Dashboard from '@/views/dashboard'
import MpGenerator from '@/views/generator/mp'
import { RouteMenu } from './type'
import EnDecoder from '@/views/endecoer'

export const Home: RouteMenu = {
  path: '/',
  element: <Layout />,
  handle: {
    name: 'Home',
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
          name: 'Dashboard',
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
      name: 'Generator',
      title: '代码生成器',
      icon: <IconCodeSandbox />,
      submenu: true,
    },
    children: [
      {
        path: 'mp',
        handle: {
          name: 'MyBatisPlusGenerator',
          title: 'MyBatisPlus生成器',
          icon: <IconCodeBlock />,
        },
        element: <MpGenerator />,
      },
    ],
  },
  {
    path: '/endecoder',
    element: <Layout />,
    handle: {
      name: 'Endecoder',
      title: '编解码',
      icon: <IconLock />,
      submenu: true,
    },
    children: [
      {
        path: 'en',
        element: <EnDecoder />,
        handle: {
          name: 'Encoder',
          title: '编码',
          icon: <IconLock />,
        },
      },
    ],
  },
]

const router = createBrowserRouter(routes.map(route => ({...route} as RouteObject)))

export default router
