import { ReactNode } from 'react'

export interface BreadcrumbProps {
  name: string,
  title: string
  submenu?: boolean
  icon?: ReactNode
}
