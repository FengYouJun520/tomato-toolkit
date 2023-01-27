import { TemplateConfig } from '@/types/type'

export const useTemplateConfigStore = defineStore('templateConfigState', {
  state: (): TemplateConfig => ({
    disable: false,
    entity: 'templates/entity.java',
    entityKt: 'templates/entity.java',
    controller: 'templates/controller.java',
    mapper: 'templates/mapper.java',
    xml: 'templates/mapper.xml',
    service: 'templates/service.java',
    serviceImpl: 'templates/serviceImpl.java',
  }),
  persistedState: {
  },
})
