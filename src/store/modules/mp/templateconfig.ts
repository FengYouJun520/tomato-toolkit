import { TemplateConfig } from '@/types/type'

export const useTemplateConfigStore = defineStore('templateConfigState', {
  state: (): TemplateConfig => ({
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
  }),
  persistedState: {
  },
})
