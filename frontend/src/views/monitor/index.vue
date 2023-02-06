<template>
  <to-page extractClass="gap-3" operationClass="justify-end" title="首页">
    <t-button theme="primary">primary</t-button>
    <t-alert message="这是一条成功的消息提示" theme="success"/>
    <t-button theme="success">success</t-button>
    <t-button theme="warning">warning</t-button>
    <t-button theme="danger">danger</t-button>

    <t-divider/>

    <t-table
      :columns="columns"
      :data="data"
      :selected-row-keys="selectedRowKeys"
      bordered
      row-key="tid"
      @select-change="rehandleSelectChange"
    >
      <template #status="{ row }">
        <p v-if="row.status === 0" class="status">健康</p>
        <p v-if="row.status === 1" class="status unhealth">异常</p>
      </template>
      <template #op-column>
        <p>操作</p>
      </template>
      <template #op="slotProps">
        <a class="link" @click="rehandleClickOp(slotProps)">管理</a>
        <a class="link" @click="rehandleClickOp(slotProps)">删除</a>
      </template>
    </t-table>

    <template #extract>
      <t-button theme="warning">warning</t-button>
      <t-button theme="danger">danger</t-button>
    </template>
    <template #footer>
      <span>footer</span>
    </template>
    <template #operation>
      <t-button theme="warning">warning</t-button>
      <t-button theme="danger">danger</t-button>
    </template>
  </to-page>
</template>
<script lang="ts" setup>
import ToPage from '@/components/ToPage/index.vue'

const columns = [
  {
    colKey: 'row-select',
    type: 'multiple',
    // disabled 参数：{row: RowData; rowIndex: number })
    disabled: ({rowIndex}: { rowIndex: number | string }) => rowIndex === 1 || rowIndex === 3,
    width: 50,
  },
  {colKey: 'instance', title: '集群名称', width: 150},
  {
    colKey: 'status',
    title: '状态',
    width: 100,
    cell: 'status',
  },
  {colKey: 'owner', title: '管理员'},
  {colKey: 'description', title: '描述'},
  {
    colKey: 'op',
    width: 200,
    title: 'op-column',
    cell: 'op',
  },
]
const data = [
  {
    tid: 1,
    instance: 'JQTest1',
    status: 0,
    owner: 'jenny;peter',
    description: 'test',
  },
  {
    tid: '2',
    instance: 'JQTest2',
    status: 1,
    owner: 'jenny',
    description: 'test',
  },
  {
    tid: 3,
    instance: 'JQTest3',
    status: 0,
    owner: 'jenny',
    description: 'test',
  },
  {
    tid: 4,
    instance: 'JQTest4',
    status: 1,
    owner: 'peter',
    description: 'test',
  },
]
const selectedRowKeys = ref([3, '2'])

const rehandleClickOp = ({text, row}: any) => {
  console.log(text, row)
}

const rehandleSelectChange = (value: any, {selectedRowData}: any) => {
  selectedRowKeys.value = value
  console.log(value, selectedRowData)
}
</script>

<style lang="scss" scoped></style>
