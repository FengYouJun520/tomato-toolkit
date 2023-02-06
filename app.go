package main

import (
	"context"
	"fmt"

	"tomoto/codegen"

	_ "embed"

	"github.com/getlantern/systray"
	"github.com/wailsapp/wails/v2/pkg/runtime"
	"gorm.io/driver/mysql"
	"gorm.io/gorm"
)

//go:embed frontend/public/favicon.ico
var openIcon []byte

// App struct
type App struct {
	ctx context.Context
}

// NewApp creates a new App application struct
func NewApp() *App {
	return &App{}
}

// startup is called at application startup
func (a *App) startup(ctx context.Context) {
	// Perform your setup here
	a.ctx = ctx
	// 初始化系统托盘
	systray.Run(a.onReady, a.onExit)
}

// domReady is called after the front-end dom has been loaded
func (a *App) domReady(ctx context.Context) {
	// Add your action here

}

// shutdown is called at application termination
func (a *App) shutdown(ctx context.Context) {
	// Perform your teardown here
	systray.Quit()
}

// PingDb 测试连接是否正常
// 如果正常，返回数据库的表信息
func (a *App) PingDb(dataSource codegen.DataSourceConfig) ([]codegen.DatabaseOptions, error) {
	var (
		dsn = ""
		db  *gorm.DB
		err error
	)

	switch dataSource.Typ {
	case "mysql":
		dsn = fmt.Sprintf("%v:%v@tcp(%v:%v)/%v?charset=utf8mb4&parseTime=True&loc=Local&timeout=5s",
			dataSource.Username, dataSource.Password, dataSource.Host,
			dataSource.Port, dataSource.Database)
		db, err = gorm.Open(mysql.Open(dsn), &gorm.Config{})
		if err != nil {
			return nil, err
		}
	}

	var options []codegen.DatabaseOptions
	db.Raw(`SELECT table_name name,IFNULL(TABLE_COMMENT,table_name) comment
FROM INFORMATION_SCHEMA.TABLES
WHERE UPPER(table_type)='BASE TABLE'
  AND LOWER(table_schema) = ?`, dataSource.Database).Scan(&options)

	return options, nil
}

func (a *App) onReady() {
	systray.SetIcon(openIcon)
	systray.SetTitle("tomato-toolkit")
	systray.SetTooltip("番茄工具箱")
	openMenu := systray.AddMenuItem("主界面", "显示主界面")

	systray.AddSeparator()
	quitMenu := systray.AddMenuItem("退出", "退出")

	go func() {
		for {
			select {
			case <-openMenu.ClickedCh:
				runtime.WindowShow(a.ctx)
			case <-quitMenu.ClickedCh:
				runtime.Quit(a.ctx)
				return
			}
		}
	}()
}

func (a *App) onExit() {
	// clean up here
}
