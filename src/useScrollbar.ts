import { useOverlayScrollbars } from 'overlayscrollbars-react'
import { useStore } from './store'
export function useScrollbar() {
  const store = useStore()
  const [initialize] = useOverlayScrollbars({options: {
    overflow: {
      x: 'hidden',
    },
    scrollbars: {
      theme: store.ui.currentTheme === 'dark' ? 'os-theme-light' : 'os-theme-dark',
    },
  }})
  useEffect(() => {
    initialize(document.body)
  }, [initialize])
}
