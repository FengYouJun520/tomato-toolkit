import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import Layout from '@/layout/index.vue'
import { HomeOutline, TerminalOutline, CodeDownload, Apps } from '@vicons/ionicons5'

export const routes: RouteRecordRaw[] = [
  {
    name: 'Root',
    path: '/',
    redirect: '/dashboard',
    component: Layout,
    meta: {
      title: '首页',
      icon: HomeOutline,
    },
    children: [
      {
        name: 'Dashboard',
        path: 'dashboard',
        component: () => import('@/views/dashboard/index.vue'),
        meta: {
          title: '首页',
          icon: Apps,
        },
      },
    ],
  },
  {
    name: 'Generator',
    path: '/generator',
    redirect: '/generator/mp',
    component: Layout,
    meta: {
      title: '代码生成器',
      icon: CodeDownload,
    },
    children: [
      {
        name: 'MyBatisPlusGenerator',
        path: 'mp',
        component: () => import('@/views/generator/mp.vue'),
        meta: {
          title: 'MyBatisPlus生成器',
          icon: TerminalOutline,
        },
      },
    ],
  },
]

const router = createRouter({
  routes,
  history: createWebHistory(),
})

export default router
