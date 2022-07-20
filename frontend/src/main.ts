import TDesign from 'tdesign-vue-next'
import {createApp} from 'vue'
import App from './App.vue'
import router, {setupRouter} from './router'
import {setupStore} from './store'
import 'uno.css'
import 'tdesign-vue-next/es/style/index.css'
import './styles/index.css'

async function bootstrap() {
  const app = createApp(App)
  setupStore(app)
  setupRouter(app)
  app.use(TDesign)

  await router.isReady()
  app.mount('#app')
}

bootstrap()
