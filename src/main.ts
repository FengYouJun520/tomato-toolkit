import { createApp } from 'vue'
import './index.css'
import App from './App.vue'
import router from './router'
import store from './store'
import './userWorker'

const app = createApp(App)
app.use(router)
app.use(store)
app.mount('#app')
