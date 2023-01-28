use std::{rc::Rc, str::FromStr};

use dyn_fmt::AsStrFormatExt;
use sqlx::any::AnyKind;

use crate::{
    codegen::config::{DataSourceConfig, StrategyConfig},
    error::Result,
};

use super::model::TableInfo;

pub trait DbQuery {
    /// 表信息查询 SQL
    fn table_sql(&self) -> String;
    /// 表字段信息查询 SQL
    fn table_fields_sql(&self) -> String;
    /// 表名称
    fn table_name(&self) -> String;
    /// 表注释
    fn table_comment(&self) -> String;
    /// 字段名称
    fn field_name(&self) -> String;
    /// 字段类型
    fn field_type(&self) -> String;
    /// 字段注释
    fn field_comment(&self) -> String;
    /// 主键字段
    fn field_key(&self) -> String;
}

#[derive(Clone)]
pub struct DefaultDbQuery {
    db_query: Rc<dyn DbQuery>,
    db_type: AnyKind,
    strategy_config: StrategyConfig,
    schema: Option<String>,
}

impl DefaultDbQuery {
    pub fn new(datasource: DataSourceConfig, strategy_config: StrategyConfig) -> Self {
        Self {
            db_query: datasource.db_query(),
            db_type: AnyKind::from_str(&datasource.db_url()).unwrap_or(AnyKind::MySql),
            strategy_config,
            schema: None,
        }
    }

    pub fn table_fields_sql_tablename(&self, table_name: &str) -> String {
        let table_fields_sql = self.table_fields_sql();
        match self.db_type {
            AnyKind::Postgres => table_fields_sql.format(&[table_name, table_name, table_name]),
            _ => table_fields_sql.format(&[table_name]),
        }
    }

    pub async fn query_tables(&self) -> Result<Vec<TableInfo>> {
        Ok(vec![])
    }
}

impl DbQuery for DefaultDbQuery {
    fn table_sql(&self) -> String {
        self.db_query.table_sql()
    }

    fn table_fields_sql(&self) -> String {
        self.db_query.table_fields_sql()
    }

    fn table_name(&self) -> String {
        self.db_query.table_name()
    }

    fn table_comment(&self) -> String {
        self.db_query.table_comment()
    }

    fn field_name(&self) -> String {
        self.db_query.field_name()
    }

    fn field_type(&self) -> String {
        self.db_query.field_type()
    }

    fn field_comment(&self) -> String {
        self.db_query.field_comment()
    }

    fn field_key(&self) -> String {
        self.db_query.field_key()
    }
}

pub struct MysqlDbQuery;

impl DbQuery for MysqlDbQuery {
    fn table_sql(&self) -> String {
        "show table status WHERE 1=1 ".to_string()
    }

    fn table_fields_sql(&self) -> String {
        "show full columns from `{}`".to_string()
    }

    fn table_name(&self) -> String {
        "NAME".to_string()
    }

    fn table_comment(&self) -> String {
        "COMMENT".to_string()
    }

    fn field_name(&self) -> String {
        "FIELD".to_string()
    }

    fn field_type(&self) -> String {
        "TYPE".to_string()
    }

    fn field_comment(&self) -> String {
        "COMMENT".to_string()
    }

    fn field_key(&self) -> String {
        "KEY".to_string()
    }
}

pub struct SqliteDbQuery;

impl DbQuery for SqliteDbQuery {
    fn table_sql(&self) -> String {
        "select * from sqlite_master where type='table'".to_string()
    }

    fn table_fields_sql(&self) -> String {
        "pragma table_info('{}');".to_string()
    }

    fn table_name(&self) -> String {
        "name".to_string()
    }

    fn table_comment(&self) -> String {
        "".to_string()
    }

    fn field_name(&self) -> String {
        "name".to_string()
    }

    fn field_type(&self) -> String {
        "type".to_string()
    }

    fn field_comment(&self) -> String {
        "".to_string()
    }

    fn field_key(&self) -> String {
        "pk".to_string()
    }
}

pub struct MsSqlDbQuery;

impl DbQuery for MsSqlDbQuery {
    fn table_sql(&self) -> String {
        r#"select * from (select cast(so.name as varchar(500)) as TABLE_NAME, 
 cast(sep.value as varchar(500)) as COMMENTS from sysobjects so 
 left JOIN sys.extended_properties sep on sep.major_id=so.id and sep.minor_id=0
 where (xtype='U' or xtype='v')) a where 1=1 "#
            .to_string()
    }

    fn table_fields_sql(&self) -> String {
        r#"SELECT  cast(a.name AS VARCHAR(500)) AS TABLE_NAME,cast(b.name AS VARCHAR(500)) AS COLUMN_NAME,
 cast(c.VALUE AS NVARCHAR(500)) AS COMMENTS,cast(sys.types.name AS VARCHAR (500)) AS DATA_TYPE,
 (" + " SELECT CASE count(1) WHEN 1 then 'PRI' ELSE '' END
 FROM syscolumns,sysobjects,sysindexes,sysindexkeys,systypes
 WHERE syscolumns.xusertype = systypes.xusertype AND syscolumns.id = object_id (a.name) AND sysobjects.xtype = 'PK'
 AND sysobjects.parent_obj = syscolumns.id " + " AND sysindexes.id = syscolumns.id
 AND sysobjects.name = sysindexes.name AND sysindexkeys.id = syscolumns.id
 AND sysindexkeys.indid = sysindexes.indid
 AND syscolumns.colid = sysindexkeys.colid AND syscolumns.name = b.name) as 'KEY',
 b.is_identity isIdentity
 FROM ( select name,object_id from sys.tables UNION all select name,object_id from sys.views ) a
 INNER JOIN sys.columns b ON b.object_id = a.object_id
 LEFT JOIN sys.types ON b.user_type_id = sys.types.user_type_id
 LEFT JOIN sys.extended_properties c ON c.major_id = b.object_id AND c.minor_id = b.column_id
 WHERE a.name = '%s' and sys.types.name !='sysname' "#.to_string()
    }

    fn table_name(&self) -> String {
        "TABLE_NAME".to_string()
    }

    fn table_comment(&self) -> String {
        "COMMENTS".to_string()
    }

    fn field_name(&self) -> String {
        "COLUMN_NAME".to_string()
    }

    fn field_type(&self) -> String {
        "DATA_TYPE".to_string()
    }

    fn field_comment(&self) -> String {
        "COMMENTS".to_string()
    }

    fn field_key(&self) -> String {
        "KEY".to_string()
    }
}

pub struct PostgresDbQuery;

impl DbQuery for PostgresDbQuery {
    fn table_sql(&self) -> String {
        "SELECT A.tablename, obj_description(relfilenode, 'pg_class') AS comments FROM pg_tables A, pg_class B WHERE A.schemaname='%s' AND A.tablename = B.relname".to_string()
    }

    fn table_fields_sql(&self) -> String {
        r#"SELECT
    A.attname AS name,format_type (A.atttypid,A.atttypmod) AS type,col_description (A.attrelid,A.attnum) AS comment,
    D.column_default,
        CASE WHEN length(B.attname) > 0 THEN 'PRI' ELSE '' END AS key
FROM
LEFT JOIN
    SELECT
        pg_attribute.attname
    FROM
        pg_index,
        pg_class,
        pg_attribute
    WHERE
        pg_class.oid ='"{}"' :: regclass
    AND pg_index.indrelid = pg_class.oid
    AND pg_attribute.attrelid = pg_class.oid
    AND pg_attribute.attnum = ANY (pg_index.indkey)
) B ON A.attname = b.attname
INNER JOIN pg_class C on A.attrelid = C.oid
INNER JOIN information_schema.columns D on A.attname = D.column_name
WHERE A.attrelid ='"{}"' :: regclass AND A.attnum> 0 AND NOT A.attisdropped AND D.table_name = '{}'
ORDER BY A.attnum;"#
        .to_string()
    }

    fn table_name(&self) -> String {
        "tablename".to_string()
    }

    fn table_comment(&self) -> String {
        "comments".to_string()
    }

    fn field_name(&self) -> String {
        "name".to_string()
    }

    fn field_type(&self) -> String {
        "type".to_string()
    }

    fn field_comment(&self) -> String {
        "comment".to_string()
    }

    fn field_key(&self) -> String {
        "key".to_string()
    }
}
