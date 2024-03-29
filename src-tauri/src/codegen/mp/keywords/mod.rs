pub mod mysql_keyword;
pub mod postgressql_keyword;

use self::{mysql_keyword::MysqlKeywordHandler, postgressql_keyword::PostgressqlKeywordHandler};
use super::types::DbType;
use dyn_fmt::AsStrFormatExt;
use enum_dispatch::enum_dispatch;

/// 关键字处理器
#[enum_dispatch]
pub trait KeywordHandler {
    fn is_keyword(&self, name: &str) -> bool;
    fn format_style(&self) -> String;
    fn format_column(&self, column_name: &str) -> String {
        self.format_style().format(&[column_name])
    }
}

/// 默认关键字处理器
#[enum_dispatch(KeywordHandler)]
pub enum DefaultKeywordHandler {
    MysqlKeywordHandler,
    PostgressqlKeywordHandler,
}

impl DefaultKeywordHandler {
    pub fn get_keyword_handler(db_type: DbType) -> Option<Self> {
        match db_type {
            DbType::MYSQL => Some(DefaultKeywordHandler::from(MysqlKeywordHandler)),
            DbType::MARIADB => None,
            DbType::ORACLE => None,
            DbType::DB2 => None,
            DbType::H2 => None,
            DbType::SQLITE => None,
            DbType::POSTGRES_SQL => Some(DefaultKeywordHandler::from(PostgressqlKeywordHandler)),
            DbType::SQL_SERVER => None,
            DbType::OTHER => None,
        }
    }
}
