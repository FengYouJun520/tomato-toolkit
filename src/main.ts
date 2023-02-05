import { createApp } from 'vue'
import './index.css'
import App from './App.vue'
import router from './router'
import store from './store'
import './userWorker'
import 'vfonts/FiraCode.css'
import 'vue-json-pretty/lib/styles.css'

const meta = document.createElement('meta')
meta.name = 'naive-ui-style'
document.head.appendChild(meta)

const app = createApp(App)
app.use(router)
app.use(store)
app.mount('#app')
