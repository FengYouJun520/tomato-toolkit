import localforage from 'localforage'
import { configurePersistable } from 'mobx-persist-store'
import { createContext, useContext } from 'react'
import { MyBaTisPlusStore } from './mp'
import { UiState } from './ui'

configurePersistable({
  storage: import.meta.env.DEV ? localStorage : localforage,
  debugMode: import.meta.env.DEV,
  stringify: true,
})

class RootStore {
  ui: UiState
  mp: MyBaTisPlusStore

  constructor() {
    this.ui = new UiState()
    this.mp = new MyBaTisPlusStore()
  }

}

export const store = new RootStore()
export const StoreContext = createContext(store)
export const useStore = () => useContext(StoreContext)
