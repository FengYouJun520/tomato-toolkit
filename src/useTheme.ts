import { useStore } from './store'

export function useTheme() {
  const store = useStore()

  useEffect(() => {
    const darkThemeMq = window.matchMedia('(prefers-color-scheme: dark)')
    darkThemeMq.addEventListener('change',e => {
      if(store.ui.theme !== 'system') {
        return
      }
      changeTheme(e.matches ? 'dark' : 'light')
    })
  }, [store.ui.theme])

  autorun(() => {
    let theme = 'light'
    if(store.ui.theme === 'system') {
      const isDark = window.matchMedia('(prefers-color-scheme: dark)')
      if(isDark.matches) {
        theme = 'dark'
      }
    }else if(store.ui.theme === 'dark') {
      theme = 'dark'
    }else{
      theme = 'light'
    }

    changeTheme(theme)
  })
}

function changeTheme(theme: string) {

  if(theme === 'light') {
    document.body.removeAttribute('arco-theme')
  } else {
    document.body.setAttribute('arco-theme', 'dark')
  }
}
