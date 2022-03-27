import { App } from 'vue'
import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = []

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
