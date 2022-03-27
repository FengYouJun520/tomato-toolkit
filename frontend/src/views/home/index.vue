<template>
  <div>
    <img :src="logo" />
    <hello-world msg="vite+vue3+wails template" />
  </div>
</template>
<script setup lang="ts">
import logo from '@/assets/logo.png'
import HelloWorld from '@/components/HelloWorld.vue'
</script>

<style lang="scss" scoped></style>
