use sqlx::FromRow;

/// 表信息
#[derive(Debug, Clone, FromRow)]
pub struct TableInfo {
    /// 表名
    pub name: String,
    /// 注释
    #[sqlx(default)]
    pub comment: Option<String>,
    /// 表所属数据库名
    #[sqlx(default)]
    pub schema: String,
    /// 表所有列信息
    #[sqlx(default)]
    pub fields: Option<Vec<TableField>>,
}

/// 列信息
#[derive(Debug, Clone, FromRow)]
pub struct TableField {
    /// 列名
    pub name: String,
    ///注释
    #[sqlx(default)]
    pub comment: Option<String>,
    /// 列类型
    pub r#type: String,
    /// 是否可为空
    pub is_nullable: bool,
    /// 列长度
    pub length: Option<i64>,
    ///是否是主键
    pub key_flag: bool,
}
