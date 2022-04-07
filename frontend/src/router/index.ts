import LAYOUT from '@/layouts/index.vue'
import { App } from 'vue'
import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Root',
    redirect: '/home',
    component: LAYOUT,
    meta: {
      title: '首页',
      icon: 'dashboard',
    },
    children: [
      {
        path: 'home',
        name: 'Home',
        component: () => import('@/views/home/index.vue'),
        meta: {
          title: '首页',
          icon: 'dashboard',
        },
      },
    ],
  },
  {
    path: '/mybatis-plus',
    name: 'MybatisPlus',
    redirect: '/mybatis-plus/code-generate',
    component: LAYOUT,
    meta: {
      title: 'MybatisPlus',
      icon: 'server',
    },
    children: [
      {
        path: 'code-generate',
        name: 'MybatisPlusCodeGenerate',
        component: () => import('@/views/mybatis-plus/generator/index.vue'),
        meta: {
          title: 'MybatisPlus代码生成器',
        },
      },
    ],
  },
  {
    path: '/json',
    name: 'Json',
    redirect: '/json/json2ts',
    component: LAYOUT,
    meta: {
      title: 'json管理',
      icon: 'control-platform',
    },
    children: [
      {
        path: 'json2ts',
        name: 'Json2Ts',
        component: () => import('@/views/json/json2ts/index.vue'),
        meta: {
          title: 'json转ts',
          icon: 'precise-monitor',
        },
      },
    ],
  },
  {
    path: '/crypto',
    name: 'Crypto',
    redirect: '/crypto/crypto',
    component: LAYOUT,
    meta: {
      title: '加解密管理',
      icon: 'control-platform',
    },
    children: [
      {
        path: 'crypto',
        name: 'CryptoEnDe',
        component: () => import('@/views/crypto/en-de/index.vue'),
        meta: {
          title: '加解密',
          icon: 'precise-monitor',
        },
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
