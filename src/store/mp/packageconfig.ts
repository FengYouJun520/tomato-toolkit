import { PackageConfig } from '@/types/type'
import { isHydrated, makePersistable } from 'mobx-persist-store'

export const initialPackageConfig: PackageConfig = {
  parent: 'com.baomidou',
  moduleName: '',
  entity: 'entity',
  controller: 'controller',
  service: 'service',
  serviceImpl: 'service.impl',
  mapper: 'mapper',
  xml: 'mapper.xml',
}

export class PackageConfigStore {
  package: PackageConfig = initialPackageConfig

  constructor() {
    makeAutoObservable(this, {}, {autoBind: true})
    makePersistable(this, {
      name: 'PackageConfigStore',
      properties: ['package'],
    })
  }

  get isHydrated(){
    return isHydrated(this)
  }

  setPackageConfig(packageConfig: Partial<PackageConfig>) {
    this.package = {
      ...this.package,
      ...packageConfig,
    }
  }
}
