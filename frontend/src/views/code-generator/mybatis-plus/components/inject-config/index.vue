<template>
  <div>
    <t-form :label-width="140" colon label-align="right">
      <t-row :gutter="[24, 24]" mt-6>
        <t-col>
          <t-button theme="warning" @click="injectStore.clearInject">重置</t-button>
        </t-col>
      </t-row>
    </t-form>
  </div>
</template>
<script lang="ts" setup>
import {useInject} from '@/store/modules/mybatis/useInject'

const injectStore = useInject()
</script>

<style lang="scss" scoped></style>
