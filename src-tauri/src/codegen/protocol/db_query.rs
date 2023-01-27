use std::str::FromStr;

use dyn_fmt::AsStrFormatExt;
use sqlx::{any::AnyKind, AnyConnection};

use crate::codegen::config::{DataSourceConfig, StrategyConfig};

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

pub struct DefaultDbQuery {
    db_query: Box<dyn DbQuery>,
    conn: AnyConnection,
    db_type: AnyKind,
    strategy_config: StrategyConfig,
    schema: Option<String>,
}

impl DefaultDbQuery {
    pub fn new(
        datasource: DataSourceConfig,
        conn: AnyConnection,
        strategy_config: StrategyConfig,
    ) -> Self {
        Self {
            db_query: datasource.db_query(),
            conn,
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
        todo!()
    }

    fn table_fields_sql(&self) -> String {
        todo!()
    }

    fn table_name(&self) -> String {
        todo!()
    }

    fn table_comment(&self) -> String {
        todo!()
    }

    fn field_name(&self) -> String {
        todo!()
    }

    fn field_type(&self) -> String {
        todo!()
    }

    fn field_comment(&self) -> String {
        todo!()
    }

    fn field_key(&self) -> String {
        todo!()
    }
}

pub struct SqliteDbQuery;

impl DbQuery for SqliteDbQuery {
    fn table_sql(&self) -> String {
        todo!()
    }

    fn table_fields_sql(&self) -> String {
        todo!()
    }

    fn table_name(&self) -> String {
        todo!()
    }

    fn table_comment(&self) -> String {
        todo!()
    }

    fn field_name(&self) -> String {
        todo!()
    }

    fn field_type(&self) -> String {
        todo!()
    }

    fn field_comment(&self) -> String {
        todo!()
    }

    fn field_key(&self) -> String {
        todo!()
    }
}

pub struct MsSqlDbQuery;

impl DbQuery for MsSqlDbQuery {
    fn table_sql(&self) -> String {
        todo!()
    }

    fn table_fields_sql(&self) -> String {
        todo!()
    }

    fn table_name(&self) -> String {
        todo!()
    }

    fn table_comment(&self) -> String {
        todo!()
    }

    fn field_name(&self) -> String {
        todo!()
    }

    fn field_type(&self) -> String {
        todo!()
    }

    fn field_comment(&self) -> String {
        todo!()
    }

    fn field_key(&self) -> String {
        todo!()
    }
}

pub struct PostgresDbQuery;

impl DbQuery for PostgresDbQuery {
    fn table_sql(&self) -> String {
        todo!()
    }

    fn table_fields_sql(&self) -> String {
        todo!()
    }

    fn table_name(&self) -> String {
        todo!()
    }

    fn table_comment(&self) -> String {
        todo!()
    }

    fn field_name(&self) -> String {
        todo!()
    }

    fn field_type(&self) -> String {
        todo!()
    }

    fn field_comment(&self) -> String {
        todo!()
    }

    fn field_key(&self) -> String {
        todo!()
    }
}
