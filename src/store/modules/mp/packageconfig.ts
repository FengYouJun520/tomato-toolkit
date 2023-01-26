import { PackageConfig } from '@/types/type'

export const usePackageConfig = defineStore('packageConfigState', {
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
