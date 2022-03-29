package util

import (
	"bytes"
	"strings"
	"unicode"

	"golang.org/x/text/cases"
	"golang.org/x/text/language"
)

// SnakecaseToTitleCamel 将snakecase字符串转换为首字母大写的驼峰命名
func SnakecaseToTitleCamel(s string) string {
	caseser := cases.Title(language.English)
	lines := strings.Split(s, "_")
	for i, line := range lines {
		line = caseser.String(line)
		lines[i] = line
	}
	return strings.Join(lines, "")
}

// SnakecaseToCamel 将snakecase字符串转换为首字母小写的驼峰命名
func SnakecaseToCamel(s string) string {
	caseser := cases.Title(language.English)
	if strings.HasPrefix(s, "_") {
		s = s[1:]
	}
	lines := strings.Split(s, "_")
	lines[0] = strings.ToLower(lines[0])

	for i := 1; i < len(lines); i++ {
		line := caseser.String(lines[i])
		lines[i] = line
	}
	return strings.Join(lines, "")
}

// ToKebabcase 将驼峰命名转为以-短横线连接的kebabcase风格
func CamelToKebabcase(s string) string {
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

func Title(s string) string {
	caseser := cases.Title(language.English)
	return caseser.String(s)
}
