<template>
  <div class="layout">
    <router-view />
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
</script>

<style lang="scss" scoped>
.layout {
  width: 100%;
  height: 100%;
}
</style>
