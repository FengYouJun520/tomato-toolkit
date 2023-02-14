import { RouterProvider } from 'react-router-dom'
import router from './router'
import { useTheme } from './useTheme'

function App() {
  useTheme()
  return (
    <RouterProvider router={router} />
  )
}

export default observer(App)
