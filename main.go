package main

import (
	"embed"
	"log"

	"tomoto/codegen"
	"tomoto/crypt"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/logger"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/windows"
)

//go:embed frontend/dist
var assets embed.FS

//go:embed templates
var templates embed.FS

func main() {

	// Create an instance of the app structure
	app := NewApp()
	// Create an instance of the app structure
	manager := codegen.NewManager(templates)
	crypto := crypt.NewCrypt()

	// Create application with options
	err := wails.Run(&options.App{
		Title:             "番茄工具包",
		Width:             1200,
		Height:            800,
		MinWidth:          720,
		MinHeight:         570,
		MaxWidth:          1920,
		MaxHeight:         1080,
		DisableResize:     false,
		Fullscreen:        false,
		Frameless:         false,
		StartHidden:       false,
		HideWindowOnClose: true,
		BackgroundColour:  &options.RGBA{R: 255, G: 255, B: 255, A: 255},
		Assets:            assets,
		LogLevel:          logger.DEBUG,
		OnStartup:         app.startup,
		OnDomReady:        app.domReady,
		OnShutdown:        app.shutdown,
		Bind: []interface{}{
			app,
			manager,
			crypto,
		},
		// Windows platform specific options
		Windows: &windows.Options{
			WebviewIsTransparent: false,
			WindowIsTranslucent:  false,
			DisableWindowIcon:    false,
		},
	})

	if err != nil {
		log.Fatal(err)
	}
}
