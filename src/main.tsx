import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import { store, StoreContext } from './store'
import '@arco-design/web-react/dist/css/arco.css'
import 'overlayscrollbars/overlayscrollbars.css'
import './index.css'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <StoreContext.Provider value={store} >
      <App />
    </StoreContext.Provider>
  </React.StrictMode>,
)
