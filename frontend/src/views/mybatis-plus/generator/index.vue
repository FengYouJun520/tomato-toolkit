<template>
  <to-page>
    <template #title>
      <div class="flex items-center gap-3">
        <img :src="MybatisIcon" alt="mybatis-plus" class="h-12" />
        <div>Mybatis-Plus代码生成器</div>
      </div>
    </template>
    <div class="flex flex-col gap-y-3">
      <!-- 基础配置 -->
      <basic-config />
    </div>

    <template #extract>
      <div class="flex justify-center">
        <t-button>生成</t-button>
      </div>
    </template>
  </to-page>
</template>
<script setup lang="ts">
import MybatisIcon from '@/assets/mybatis.png'
import ToPage from '@/components/ToPage/index.vue'
import BasicConfig from './components/BasicConfig/index.vue'
</script>

<style scoped></style>
