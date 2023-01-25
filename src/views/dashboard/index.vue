<script setup lang="ts">

</script>

<template>
  <n-card>
    <div>dashboard</div>
    <div v-for="i in 50" :key="i">
      {{ i }}
    </div>
  </n-card>
</template>

<style lang="css" scoped>
</style>
