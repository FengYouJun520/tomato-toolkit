import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { setupPinia } from './store'

async function bootstrap() {
  const app = createApp(App)
  setupPinia(app)
  app.use(router)

  await router.isReady()
  app.mount('#app')
}

bootstrap()
