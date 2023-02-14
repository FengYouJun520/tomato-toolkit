import { TemplateConfig } from '@/types/type'
import { isHydrated, makePersistable } from 'mobx-persist-store'


export const initialTemplateConfig: TemplateConfig = {
  entity: '',
  entityKt: '',
  controller: '',
  mapper: '',
  xml: '',
  service: '',
  serviceImpl: '',
  disableEntity: false,
  disableController: false,
  disableMapper: false,
  disableXml: false,
  disableService: false,
  disableServiceImpl: false,
}

export class TemplateConfigStore {
  template: TemplateConfig = initialTemplateConfig

  constructor() {
    makeAutoObservable(this, {}, {autoBind: true})
    makePersistable(this, {
      name: 'TemplateConfigStore',
      properties: ['template'],
    })
  }

  get isHydrated(){
    return isHydrated(this)
  }

  setTemplateConfig(templateConfig: Partial<TemplateConfig>) {
    this.template = {
      ...this.template,
      ...templateConfig,
    }
  }
}
