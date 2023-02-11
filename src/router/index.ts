import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import Layout from '@/layout/index.vue'
import { HomeFilled, Monitor, Odometer, Promotion } from '@element-plus/icons-vue'
export const routes: RouteRecordRaw[] = [
  {
    name: 'Root',
    path: '/',
    redirect: '/dashboard',
    component: Layout,
    meta: {
      title: '首页',
      icon: HomeFilled,
    },
    children: [
      {
        name: 'Dashboard',
        path: 'dashboard',
        component: () => import('@/views/dashboard/index.vue'),
        meta: {
          title: 'Dashboard',
          icon: Monitor,
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
      icon: Odometer,
    },
    children: [
      {
        name: 'MyBatisPlusGenerator',
        path: 'mp',
        component: () => import('@/views/generator/mp/index.vue'),
        meta: {
          title: 'MyBatisPlus生成器',
          icon: Promotion,
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
