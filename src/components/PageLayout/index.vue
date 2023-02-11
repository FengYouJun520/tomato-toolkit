<script setup lang="ts">


</script>

<template>
  <el-card>
    <template v-if="$slots.header" #header>
      <slot name="header" />
    </template>
    <slot name="default" />
    <div v-if="$slots.footer" class="flex justify-center items-center">
      <slot name="footer" />
    </div>
  </el-card>
</template>

<style lang="css" scoped>
.el-card {
  position: relative;
  min-height: calc(100vh - 105px);
}
</style>
