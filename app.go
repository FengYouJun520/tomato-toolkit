package main

import (
    "context"
    "fmt"
    "math"
    "runtime"

    "tomoto/codegen"

    "gorm.io/driver/mysql"
    "gorm.io/gorm"
)

const (
    GB = 1024 * 1024 * 1024
    MB = 1024 * 1024
    KB = 1024
)

func ConvetMemSize(kbSize uint64) string {
    if kbSize/GB > 0 {
        return fmt.Sprintf("%.2fGB", math.Round(float64(kbSize)/GB))
    } else if kbSize/MB > 0 {
        return fmt.Sprintf("%.2fMB", math.Round(float64(kbSize)/MB))
    } else if kbSize/KB > 0 {
        return fmt.Sprintf("%.2fKB", math.Round(float64(kbSize)/KB))
    } else {
        return fmt.Sprintf("%vByte", kbSize)
    }
}

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
    // t := time.Tick(2 * time.Second)
    // go func() {
    // 	for {
    // 		select {
    // 		case <-t:
    // 			p := zdpgo_psutil.New(zdpgo_log.Tmp)
    // 			info, err := p.GetBaseInfo()
    // 			if err != nil {
    // 				return
    // 			}
    // 			fmt.Printf("使用内存：%.2f%%\n", info.MemoryUsedPercent)
    // 		case <-time.After(3 * time.Second):
    // 			return
    // 		}
    // 	}
    // }()
}

// domReady is called after the front-end dom has been loaded
func (a *App) domReady(ctx context.Context) {
    // Add your action here
}

// shutdown is called at application termination
func (a *App) shutdown(ctx context.Context) {
    // Perform your teardown here
}

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

func (a *App) GetOs() string {
    return runtime.GOOS
}
