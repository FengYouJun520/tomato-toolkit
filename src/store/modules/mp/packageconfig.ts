import { PackageConfig } from '@/types/type'

export const usePackageConfigStore = defineStore('packageConfigState', {
  state: (): PackageConfig => ({
    parent: 'com.baomidou',
    moduleName: '',
    entity: 'entity',
    service: 'service',
    serviceImpl: 'service.impl',
    mapper: 'mapper',
    xml: 'mapper.xml',
    controller: 'controller',
  }),
  persist: true,
})
