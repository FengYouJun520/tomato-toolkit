import { createApp } from 'vue'
import './index.css'
import App from './App.vue'
import router from './router'
import 'vfonts/FiraCode.css'

const meta = document.createElement('meta')
meta.name = 'naive-ui-style'
document.head.appendChild(meta)

const app = createApp(App)
app.use(router)
app.mount('#app')
