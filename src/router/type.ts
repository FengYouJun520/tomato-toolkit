import { BreadcrumbProps } from '@/layout/type'
import { RouteObject } from 'react-router-dom'

export type RouteMenu = {
  children?: RouteMenu[]
  handle: BreadcrumbProps
} & Omit<RouteObject, 'children'|'handle'>