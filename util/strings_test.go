// this is util package
package util

import (
	"path/filepath"
	"testing"
)

func TestSnakecaseToTitleCamel(t *testing.T) {
	type TestStruct struct {
		input  string
		expect string
	}

	tests := []TestStruct{
		{
			input:  "sys_user",
			expect: "SysUser",
		},
		{
			input:  "_sys_user",
			expect: "SysUser",
		},
		{
			input:  "sys_user_",
			expect: "SysUser",
		},
		{
			input:  "_sys_user_",
			expect: "SysUser",
		},
		{
			input:  "sys_user_role",
			expect: "SysUserRole",
		},
		{
			input:  "_sys_user_role",
			expect: "SysUserRole",
		},
		{
			input:  "sys_user_role_",
			expect: "SysUserRole",
		},
		{
			input:  "_sys_user_role_",
			expect: "SysUserRole",
		},
	}

	for _, v := range tests {
		res := SnakecaseToTitleCamel(v.input)
		if res != v.expect {
			t.Fatalf("input: %v, res: %v, expect: %v\n", v.input, res, v.expect)
		}
	}
}

func TestSnakecaseToCamel(t *testing.T) {
	type TestStruct struct {
		input  string
		expect string
	}

	tests := []TestStruct{
		{
			input:  "sys_user",
			expect: "sysUser",
		},
		{
			input:  "_sys_user",
			expect: "sysUser",
		},
		{
			input:  "sys_user_",
			expect: "sysUser",
		},
		{
			input:  "_sys_user_",
			expect: "sysUser",
		},
		{
			input:  "sys_user_role",
			expect: "sysUserRole",
		},
		{
			input:  "_sys_user_role",
			expect: "sysUserRole",
		},
		{
			input:  "sys_user_role_",
			expect: "sysUserRole",
		},
		{
			input:  "_sys_user_role_",
			expect: "sysUserRole",
		},
		{
			input:  "_Sys_user_role_",
			expect: "sysUserRole",
		},
	}

	for _, v := range tests {
		res := SnakecaseToCamel(v.input)
		if res != v.expect {
			t.Fatalf("input: %v, res: %v, expect: %v\n", v.input, res, v.expect)
		}
	}
}

func TestCameToKebabcase(t *testing.T) {
	type TestStruct struct {
		input  string
		expect string
	}

	tests := []TestStruct{
		{
			input:  "sysUser",
			expect: "sys-user",
		},
		{
			input:  "category",
			expect: "category",
		},
		{
			input:  "sysRoleMenu",
			expect: "sys-role-menu",
		},
		{
			input:  "sysUser",
			expect: "sys-user",
		},
	}

	for _, v := range tests {
		res := CamelToKebabcase(v.input)
		if res != v.expect {
			t.Fatalf("input: %v, res: %v, expect: %v\n", v.input, res, v.expect)
		}
	}
}

func TestOpenDir(t *testing.T) {
	t.Log(filepath.Clean("D:/code"))
}
