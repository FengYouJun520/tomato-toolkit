import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: [
      {
        find: '@',
        replacement: resolve(__dirname, 'src'),
      },
      {
        find: '~wails',
        replacement: resolve(__dirname, 'src/wailsjs'),
      },
    ],
  },
  plugins: [vue()],
})
