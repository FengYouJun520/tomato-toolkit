import { FC } from 'react'
import { Breadcrumb as ArcoBreadcrumb } from '@arco-design/web-react'
import { RouteMenu } from '@/router/type'
import { Link, useMatches } from 'react-router-dom'
import { Home } from '@/router'

const BreadcrumbItem = ArcoBreadcrumb.Item

const Breadcrumb: FC = () => {
  const isHome = (breadcrumb: RouteMenu) => {
    return breadcrumb.handle.submenu && breadcrumb.path === '/'
  }
  let breadcrumbs = useMatches()
    .map(match => ({ ...match, path: match.pathname } as RouteMenu))
  if(breadcrumbs.every(crumb => crumb.path !== '/')) {
    breadcrumbs= [Home].concat(breadcrumbs)
  }

  return (
    <ArcoBreadcrumb className="flex-1 ml-5">
      {breadcrumbs.map(breadcrumb => (
        <BreadcrumbItem
          key={breadcrumb.handle.title}
        >
          <div className="flex items-center space-x-2">
            {breadcrumb.handle.icon && breadcrumb.handle.icon}
            {
              isHome(breadcrumb) || (!breadcrumb.handle.submenu && breadcrumb.path !== location.pathname)
                ?
                <Link to={{pathname: breadcrumb.path || '/'}}>
                  {breadcrumb.handle.title}
                </Link>
                :
                <span>{breadcrumb.handle.title}</span>
            }
          </div>
        </BreadcrumbItem>
      ))}
    </ArcoBreadcrumb>
  )
}

export default Breadcrumb
