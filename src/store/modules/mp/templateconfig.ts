import { TemplateConfig } from '@/types/type'

export const useTemplateConfig = defineStore('templateConfigState', {
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
