import { createPinia } from 'pinia'
import PiniaPersisted from 'pinia-plugin-persistedstate'
const store = createPinia()

store.use(PiniaPersisted)

export default store
