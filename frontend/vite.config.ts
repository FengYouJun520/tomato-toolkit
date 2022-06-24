import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import AutoImport from 'unplugin-auto-import/vite'
import { defineConfig } from 'vite'
import svgLoader from 'vite-svg-loader'
import Unocss from 'unocss/vite'
import { presetAttributify, presetUno, presetIcons } from 'unocss'
import transformerDirective from '@unocss/transformer-directives'

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: [
      {
        find: '@',
        replacement: resolve(__dirname, 'src'),
      },
    ],
  },
  server: {
    port: 22330,
  },
  plugins: [
    vue(),
    AutoImport({
      imports: ['vue'],
      dts: false,
    }),
    svgLoader(),
    Unocss({
      presets: [
        presetAttributify({
          /* preset options */
        }),
        presetUno(),
        presetIcons({
          prefix: '',
        }),
      ],
      transformers: [transformerDirective()],
    }),
  ],
  build: {
    chunkSizeWarningLimit: 4096,
    reportCompressedSize: false,
  },
})
