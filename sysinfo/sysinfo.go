package sysinfo

import (
	"fmt"

	"github.com/shirou/gopsutil/v3/cpu"
)

type SystemInfo struct {
}

func NewSystemInfo() *SystemInfo {
	return &SystemInfo{}
}

func (sys *SystemInfo) GetSystemInfo() {
	cpuInfo, _ := cpu.Info()
	fmt.Println(cpuInfo)
}
