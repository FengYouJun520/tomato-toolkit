<template>
  <t-content>
    <router-view v-slot="{ Component, route }">
      <transition name="slide-fade" appear mode="out-in">
        <keep-alive>
          <component :is="Component" :key="route.fullpath" />
        </keep-alive>
      </transition>
    </router-view>
  </t-content>
</template>
<script setup lang="ts"></script>

<style scoped>
.t-layout__content {
  @apply p-6 duration-200 w-full;
}

/*
  进入和离开动画可以使用不同
  持续时间和速度曲线。
*/
.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: all 0.2s ease-out;
}

.slide-fade-enter-from {
  transform: translateX(-20px);
  opacity: 0;
}

.slide-fade-leave-to {
  transform: translateX(20px);
  opacity: 0;
}
</style>
