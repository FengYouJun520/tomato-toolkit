import { PackageConfig } from '@/types/type'

export const usePackageConfigStore = defineStore('packageConfigState', {
  state: (): PackageConfig => ({
    parent: '',
    moduleName: '',
    entity: '',
    service: '',
    serviceImpl: '',
    mapper: '',
    xml: '',
    controller: '',
  }),
  persistedState: {
  },
})
