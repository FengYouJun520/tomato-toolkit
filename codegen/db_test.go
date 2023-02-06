package codegen

import (
	"fmt"
	"testing"

	"gorm.io/driver/mysql"
	"gorm.io/gorm"
)

var (
	querySql = `SELECT
	table_name 'name',
	IFNULL( TABLE_COMMENT, table_name ) 'comment',
	TABLE_SCHEMA 'table_schema' 
FROM
	INFORMATION_SCHEMA.TABLES 
WHERE
	UPPER( table_type )= 'BASE TABLE' 
	AND LOWER( table_schema ) = ? 
	AND table_name IN (?);
`
)

func TestInQuery(t *testing.T) {
	var tables = []string{"sys_user", "sys_role", "sys_menu"}
	dsn := "root:root@tcp(localhost:3306)/blog?charset=utf8mb4&parseTime=True&loc=Local&timeout=5s"
	db, _ := gorm.Open(mysql.Open(dsn), &gorm.Config{})

	db = db.Raw(querySql, "blog", tables)
	if db.Error != nil {
		t.Fatal(db.Error)
	}

	rows, _ := db.Rows()
	defer rows.Close()

	var tableInfos []*TableInfo

	for rows.Next() {
		var tableinfo TableInfo
		rows.Scan(&tableinfo.Name, &tableinfo.Comment, &tableinfo.SchemaName)
		tableInfos = append(tableInfos, &tableinfo)
	}

	fmt.Println(tableInfos)

	for _, tableInfo := range tableInfos {
		if err := tableInfo.ExecuteColumns(db); err != nil {
			t.Fatal(err)
		}
	}
	fmt.Println(tableInfos)
}
