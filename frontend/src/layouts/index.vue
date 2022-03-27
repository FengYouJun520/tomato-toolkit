<template>
  <t-layout>
    <LayoutAside />
    <t-layout>
      <LayoutHeader />
      <LayoutContent />
      <LayoutFooter />
    </t-layout>
  </t-layout>
</template>
<script setup lang="ts">
import LayoutAside from './components/LayoutAside/index.vue'
import LayoutContent from './components/LayoutContent/index.vue'
import LayoutFooter from './components/LayoutFooter/index.vue'
import LayoutHeader from './components/LayoutHeader/index.vue'
</script>

<style lang="scss" scoped>
.t-layout {
  height: 100%;
  width: 100%;
}
</style>
