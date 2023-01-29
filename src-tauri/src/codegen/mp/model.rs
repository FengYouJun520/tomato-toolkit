/// 表信息
#[derive(Clone)]
pub struct TableInfo {
    /// 表名
    pub name: String,
    /// 注释
    pub comment: Option<String>,
    /// 表所属数据库名
    pub schema: String,
    /// 表所有列信息
    pub fields: Vec<TableField>,
}

/// 列信息
#[derive(Clone)]
pub struct TableField {
    /// 列名
    pub name: String,
    ///注释
    pub comment: Option<String>,
    /// 列类型
    pub r#type: String,
    /// 是否可为空
    pub is_nullable: String,
    /// 列长度
    pub length: usize,
    ///是否是主键
    pub key_flag: String,
}
