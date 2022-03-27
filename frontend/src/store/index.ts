import { createPinia } from 'pinia'
import { createPersistedStatePlugin } from 'pinia-plugin-persistedstate-2'
import { App } from 'vue'

export function setupPinia(app: App) {
  const pinia = createPinia()

  const installPersistedStatePlugin = createPersistedStatePlugin()
  pinia.use((context) => installPersistedStatePlugin(context))

  app.use(pinia)
}
