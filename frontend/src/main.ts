import { createApp } from 'vue'
import App from './App.vue'
import router, { setupRouter } from './router'
import { setupStore } from './store'
import './styles/index.css'

async function bootstrap() {
  const app = createApp(App)
  setupStore(app)
  setupRouter(app)

  await router.isReady()
  app.mount('#app')
}

bootstrap()
