import { BasicTableInfo } from '@/types/type'
import { InjectionKey, Ref } from 'vue'

interface TablesInfo {
  tables: Ref<BasicTableInfo[]>
}

const TablesKey = Symbol() as InjectionKey<TablesInfo>

export const createTableContext = (tables: Ref<BasicTableInfo[]>) => provide(TablesKey, {
  tables,
})

export const useTables = () => inject(TablesKey, { tables: ref([]) })
