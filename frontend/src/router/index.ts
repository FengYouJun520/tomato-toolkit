import LAYOUT from '@/layouts/index.vue'
import { App } from 'vue'
import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Root',
    redirect: '/home',
    component: LAYOUT,
    children: [
      {
        path: '/home',
        name: 'Home',
        component: () => import('@/views/home/index.vue'),
      },
    ],
  },
  {
    path: '/mybatis-plus',
    name: 'MybatisPlus',
    redirect: '/mybatis-plus/code-generate',
    component: LAYOUT,
    children: [
      {
        path: '/mybatis-plus/code-generate',
        name: 'MybatisPlusCodeGenerate',
        component: () => import('@/views/mybatis-plus/index.vue'),
      },
    ],
  },
]

const router = createRouter({
  routes,
  history: createWebHashHistory(),
  scrollBehavior: (to, from, savedPositions) => {
    if (savedPositions) {
      return savedPositions
    }
    if (to.hash) {
      return {
        el: to.hash,
        behavior: 'smooth',
      }
    }
    return {
      top: 0,
      behavior: 'smooth',
    }
  },
})

export function setupRouter(app: App) {
  app.use(router)
}

export default router
