<script setup lang="ts">
import { DialogPlugin, MessagePlugin } from 'tdesign-vue-next'
import { ref } from 'vue'

defineProps<{ msg: string }>()

MessagePlugin.success({
  content: 'this is success message',
  duration: 2000,
})

const confirmDia = DialogPlugin({
  header: 'Dialog-Confirm-Plugin',
  body: 'Are you sure to delete it?',
  cancelBtn: 'cancel',
  onConfirm: ({ e }) => {
    console.log('confirm button has been clicked!')
    console.log('e: ', e)
    confirmDia.hide?.()
  },
  onClose: ({ e, trigger }) => {
    console.log('e: ', e)
    console.log('trigger: ', trigger)
    confirmDia.hide?.()
  },
})

const count = ref(0)
</script>

<template>
  <h1>{{ msg }}</h1>

  <p>
    Recommended IDE setup:
    <a href="https://code.visualstudio.com/" target="_blank">VSCode</a>
    +
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
  </p>

  <p>See <code>README.md</code> for more information.</p>

  <p>
    <a href="https://vitejs.dev/guide/features.html" target="_blank"> Vite Docs </a>
    |
    <a href="https://v3.vuejs.org/" target="_blank">Vue 3 Docs</a>
  </p>

  <button type="button" @click="count++">count is: {{ count }}</button>
  <p>
    Edit
    <code>components/HelloWorld.vue</code> to test hot module replacement.
  </p>
</template>

<style scoped>
a {
  color: #42b983;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

code {
  background-color: #eee;
  padding: 2px 4px;
  border-radius: 4px;
  color: #304455;
}
</style>
