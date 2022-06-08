package util

import (
	"bytes"
	"strings"
	"unicode"

	"golang.org/x/text/cases"
	"golang.org/x/text/language"
)

// SnakeCaseToTitleCamel 将snakeCase字符串转换为首字母大写的驼峰命名
func SnakeCaseToTitleCamel(s string) string {
	cas := cases.Title(language.English)
	lines := strings.Split(s, "_")
	for i, line := range lines {
		line = cas.String(line)
		lines[i] = line
	}
	return strings.Join(lines, "")
}

// SnakeCaseToCamel 将snakeCase字符串转换为首字母小写的驼峰命名
func SnakeCaseToCamel(s string) string {
	cas := cases.Title(language.English)
	if strings.HasPrefix(s, "_") {
		s = s[1:]
	}
	lines := strings.Split(s, "_")
	lines[0] = strings.ToLower(lines[0])

	for i := 1; i < len(lines); i++ {
		line := cas.String(lines[i])
		lines[i] = line
	}
	return strings.Join(lines, "")
}

// CamelToKebabCase 将驼峰命名转为以-短横线连接的kebabCase风格
func CamelToKebabCase(s string) string {
	var buf bytes.Buffer
	for i, v := range s {
		if i == 0 {
			buf.WriteRune(v)
			continue
		}
		if unicode.IsUpper(v) {
			buf.WriteRune('-')
		}
		buf.WriteRune(v)
	}
	return strings.ToLower(buf.String())
}

// Title 将字符串转为标题形式
func Title(s string) string {
	cas := cases.Title(language.English)
	return cas.String(s)
}
