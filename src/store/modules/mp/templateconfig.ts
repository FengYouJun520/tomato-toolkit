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
  }),
  persistedState: {
  },
})
