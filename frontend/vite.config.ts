import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import AutoImport from 'unplugin-auto-import/vite'
import { TDesignResolver } from 'unplugin-vue-components/resolvers'
import Components from 'unplugin-vue-components/vite'
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
        find: '@wails',
        replacement: resolve(__dirname, 'src/wails'),
      },
    ],
  },
  plugins: [
    vue(),
    AutoImport({
      resolvers: [
        TDesignResolver({
          library: 'vue-next',
        }),
      ],
      dts: resolve(__dirname, 'src/types/auto-imports.d.ts'),
    }),
    Components({
      resolvers: [
        TDesignResolver({
          library: 'vue-next',
        }),
      ],
      dts: resolve(__dirname, 'src/types/global-components.d.ts'),
    }),
  ],
})
