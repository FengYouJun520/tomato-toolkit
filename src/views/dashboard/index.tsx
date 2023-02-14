import { Card, Tag, Typography } from '@arco-design/web-react'
import { app } from '@tauri-apps/api'
import { FC } from 'react'
import tomatoIcon from '@/assets/tomato.svg'

const Dashboard: FC = () => {
  const [version, setVersion] = useState('')
  const [projectName, setProjectName] = useState('')

  useEffect(() => {
    app.getVersion().then(v => setVersion(v))
    app.getName().then(name => setProjectName(name))
  })
  return (
    <Card style={{height: 'calc(100vh - 104px)'}} bodyStyle={{height: '100%'}}>
      <div className="flex h-full flex-col justify-center items-center space-y-3">
        <img src={tomatoIcon} width={300} height={300} />

        <div>
          <Typography.Title heading={6}>
            Version:&nbsp;&nbsp;&nbsp;&nbsp;
            <Tag size="large" color="rgba(0,180,42)">
              {version}
            </Tag>
          </Typography.Title>
          <Typography.Title heading={6}>
            project:&nbsp;&nbsp;&nbsp;&nbsp;
            <Tag size="large" color="rgba(0,180,42)">
              {projectName}
            </Tag>
          </Typography.Title>
        </div>
      </div>
    </Card>
  )
}

export default Dashboard
