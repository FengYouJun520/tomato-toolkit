<script setup lang="ts">


</script>

<template>
  <el-card>
    <slot name="default" class="page-layout-content" />
  </el-card>
</template>

<style lang="css" scoped>
.el-card {
  position: relative;
}
.el-card, :deep(.el-card__body) {
  height: calc(100vh - 105px);
}

.page-layout-content {
  height: 100%;
  width: 100%;
}
</style>
