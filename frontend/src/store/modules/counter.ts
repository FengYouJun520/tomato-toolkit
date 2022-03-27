import { defineStore } from 'pinia'

interface CounterState {
  count: number
}

export const useCounter = defineStore('counter', {
  state: (): CounterState => {
    return {
      count: 0,
    }
  },
  actions: {
    increment() {
      this.count++
    },
    decrement() {
      this.count--
    },
  },
})
