use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use sqlx::{
    any::AnyConnectOptions, mssql::MssqlConnectOptions, mysql::MySqlConnectOptions,
    postgres::PgConnectOptions, sqlite::SqliteConnectOptions, AnyConnection, ConnectOptions,
    MssqlConnection, MySqlConnection, PgConnection, SqliteConnection,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    path::PathBuf,
    str::FromStr,
};

use crate::error::{Result, SerializeError};

use super::{
    context_data::{self, ControllerData, EntityData, MapperData, ServiceData, TemplateRender},
    convert::{DefaultNameConvert, NameConvert, TypeConvertHandler, TypeConverts},
    db_query::{DbQueryHandler, MsSqlQuery, MysqlQuery, PostgresQuery, SqliteQuery},
    model::TableInfo,
    types::{DateType, DbType},
    utils,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceConfig {
    pub r#type: String,
    pub database: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl DataSourceConfig {
    pub fn db_type(&self) -> DbType {
        DbType::from(&self.r#type)
    }

    pub fn db_url(&self) -> String {
        match self.r#type.as_ref() {
            "sqlite" => format!("{}://{}", &self.r#type, &self.database),
            _ => format!(
                "{}://{}:{}@{}:{}/{}",
                &self.r#type, &self.username, &self.password, &self.host, self.port, &self.database
            ),
        }
    }

    pub fn db_query(&self) -> DbQueryHandler {
        match self.db_type() {
            DbType::POSTGRES_SQL => DbQueryHandler::from(PostgresQuery),
            DbType::MYSQL => DbQueryHandler::from(MysqlQuery),
            DbType::SQLITE => DbQueryHandler::from(SqliteQuery),
            DbType::SQL_SERVER => DbQueryHandler::from(MsSqlQuery),
            _ => DbQueryHandler::from(MysqlQuery),
        }
    }

    pub async fn connect_mysql(&self) -> Result<MySqlConnection> {
        Ok(MySqlConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .connect()
            .await?)
    }

    pub async fn connect_sqlite(&self) -> Result<SqliteConnection> {
        Ok(SqliteConnectOptions::new()
            .filename(&self.database)
            .connect()
            .await?)
    }

    pub async fn connect_mssql(&self) -> Result<MssqlConnection> {
        Ok(MssqlConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .database(&self.database)
            .username(&self.username)
            .password(&self.password)
            .connect()
            .await?)
    }

    pub async fn connect_postgres(&self) -> Result<PgConnection> {
        Ok(PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .database(&self.database)
            .username(&self.username)
            .password(&self.password)
            .connect()
            .await?)
    }

    pub async fn connect(&self) -> Result<AnyConnection> {
        let options: AnyConnectOptions = match self.db_type() {
            DbType::MYSQL => MySqlConnectOptions::new()
                .host(&self.host)
                .port(self.port)
                .username(&self.username)
                .password(&self.password)
                .into(),
            DbType::SQLITE => SqliteConnectOptions::new().filename(&self.database).into(),
            DbType::SQL_SERVER => MssqlConnectOptions::new()
                .host(&self.host)
                .port(self.port)
                .database(&self.database)
                .username(&self.username)
                .password(&self.password)
                .into(),
            DbType::POSTGRES_SQL => PgConnectOptions::new()
                .host(&self.host)
                .port(self.port)
                .database(&self.database)
                .username(&self.username)
                .password(&self.password)
                .into(),
            _ => return Err(format!("不支持的数据库类型: {}", self.r#type).into()),
        };

        Ok(options.connect().await?)
    }

    pub fn get_type_convert(&self) -> TypeConvertHandler {
        TypeConverts::get_type_convert(self.db_type())
    }

    pub fn table_info_query_sql(&self) -> Result<String> {
        match self.r#type.as_ref() {
            "mysql" => Ok(format!(
                r#"SELECT table_name name,IFNULL(TABLE_COMMENT,table_name) comment
FROM INFORMATION_SCHEMA.TABLES
WHERE UPPER(table_type)='BASE TABLE'
  AND LOWER(table_schema) = '{}'"#,
                &self.database
            )),
            "sqlite" => Ok(
                "select name, '' comment from sqlite_master where type='table' order by name"
                    .to_string(),
            ),
            "sqlserver" => Ok("".to_string()),
            "postgressql" => Ok("".to_string()),
            _ => Err(format!("不支持的数据库类型: {}", self.r#type).into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlobalConfig {
    /// 生成文件的输出目录【 windows:D:// linux or mac:/tmp 】
    pub output_dir: PathBuf,
    /// 是否打开输出目录
    pub open: bool,
    /// 作者
    pub author: String,
    /// 开启 Kotlin 模式（默认 false）
    pub kotlin: bool,
    /// 开启 swagger 模式（默认 false 与 springdoc 不可同时使用）
    pub swagger: bool,
    /// 开启 springdoc 模式（默认 false 与 swagger 不可同时使用）
    pub springdoc: bool,
    /// 时间类型对应策略
    pub date_type: DateType,
    /// 获取注释日期, 默认: yyyy-MM-dd
    pub comment_date: String,
}

impl GlobalConfig {
    /// springdoc 设置优先于 swagger
    pub fn is_swagger(&self) -> bool {
        if self.springdoc {
            false
        } else {
            self.swagger
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackageConfig {
    /// 父包名。如果为空，将下面子包名必须写全部， 否则就只需写子包名, 默认："com.baomidou"
    pub parent: String,
    /// 父包模块名
    pub module_name: String,
    /// Entity包名
    pub entity: String,
    /// Service包名
    pub service: String,
    /// Service Impl包名
    pub service_impl: String,
    /// Mapper包名
    pub mapper: String,
    /// Mapper XML包名
    pub xml: String,
    /// Controller包名
    pub controller: String,
    /// 自定义路径配置信息
    pub path_info: Option<HashMap<OutputFile, PathBuf>>,
    /// 包配置信息
    pub package_infos: Option<HashMap<String, String>>,
}

impl PackageConfig {
    pub fn get_package_infos(&self) -> HashMap<String, String> {
        if self.package_infos.is_some() {
            self.package_infos.clone().unwrap()
        } else {
            let mut package_infos = HashMap::new();
            package_infos.insert("ModuleName".to_string(), self.module_name.clone());
            package_infos.insert("Entity".to_string(), self.join_path(&self.entity));
            package_infos.insert("Mapper".to_string(), self.join_path(&self.mapper));
            package_infos.insert("Xml".to_string(), self.join_path(&self.xml));
            package_infos.insert("Service".to_string(), self.join_path(&self.service));
            package_infos.insert(
                "ServiceImpl".to_string(),
                self.join_path(&self.service_impl),
            );
            package_infos.insert("Controller".to_string(), self.join_path(&self.controller));
            package_infos.insert("Parent".to_string(), self.get_parent());

            package_infos
        }
    }

    pub fn get_package_info(&self, module: &str) -> Option<String> {
        self.get_package_infos().get(module).cloned()
    }

    pub fn join_path(&self, sub_package: &str) -> String {
        let parent = self.get_parent();
        if parent.is_empty() {
            sub_package.to_string()
        } else {
            parent + "." + sub_package
        }
    }

    pub fn get_parent(&self) -> String {
        if !self.module_name.is_empty() {
            self.parent.clone() + "." + &self.module_name
        } else {
            self.parent.clone()
        }
    }

    pub fn get_path_info(&self) -> Option<HashMap<OutputFile, PathBuf>> {
        self.path_info.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TemplateConfig {
    /// 设置实体模板路径
    pub entity: Option<PathBuf>,
    /// 设置实体模板路径(kotlin模板)
    pub entity_kt: Option<PathBuf>,
    /// 设置控制器模板路径
    pub controller: Option<PathBuf>,
    /// 设置Mapper模板路径
    pub mapper: Option<PathBuf>,
    /// 设置MapperXml模板路径
    pub xml: Option<PathBuf>,
    /// 设置Service模板路径
    pub service: Option<PathBuf>,
    /// 设置ServiceImpl模板路径
    pub service_impl: Option<PathBuf>,
    pub disable_entity: bool,
    pub disable_controller: bool,
    pub disable_mapper: bool,
    pub disable_xml: bool,
    pub disable_service: bool,
    pub disable_service_impl: bool,
}

impl TemplateConfig {
    pub fn get_entity(
        &self,
        is_kotlin: bool,
        entity_path: PathBuf,
        kt_path: PathBuf,
    ) -> Option<PathBuf> {
        if self.disable_entity {
            return None;
        }

        if is_kotlin {
            Some(self.get_path(self.entity_kt.clone(), kt_path))
        } else {
            Some(self.get_path(self.entity.clone(), entity_path))
        }
    }

    pub fn get_controller(&self, default_controller: PathBuf) -> Option<PathBuf> {
        if self.disable_controller {
            return None;
        }
        Some(self.get_path(self.controller.clone(), default_controller))
    }

    pub fn get_mapper(&self, default_mapper: PathBuf) -> Option<PathBuf> {
        if self.disable_mapper {
            return None;
        }
        Some(self.get_path(self.mapper.clone(), default_mapper))
    }

    pub fn get_xml(&self, default_xml: PathBuf) -> Option<PathBuf> {
        if self.disable_xml {
            return None;
        }
        Some(self.get_path(self.xml.clone(), default_xml))
    }

    pub fn get_service(&self, default_servicer: PathBuf) -> Option<PathBuf> {
        if self.disable_service {
            return None;
        }
        Some(self.get_path(self.service.clone(), default_servicer))
    }

    pub fn get_service_impl(&self, default_service_impl: PathBuf) -> Option<PathBuf> {
        if self.disable_service_impl {
            return None;
        }

        Some(self.get_path(self.service_impl.clone(), default_service_impl))
    }

    fn get_path(&self, value: Option<PathBuf>, default_value: PathBuf) -> PathBuf {
        match value {
            Some(val) if val.to_string_lossy().is_empty() => default_value,
            Some(val) => val,
            None => default_value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomFile {
    pub file_name: String,
    pub template_path: PathBuf,
    pub package_name: String,
    pub file_path: PathBuf,
    pub file_override: bool,
    pub add_entity_prefix: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InjectConfig {
    pub custom_map: HashMap<String, serde_json::Value>,
    pub custom_files: Vec<CustomFile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StrategyConfig {
    /// 是否大写命名（默认 false）
    pub is_capital_mode: bool,
    /// 是否跳过视图（默认 false）
    pub skip_view: bool,
    /// 过滤表前缀 example: addTablePrefix("t_") result: t_simple -> Simple
    pub table_prefix: HashSet<String>,
    ///过滤表后缀 example: addTableSuffix("_0") result: t_simple_0 -> Simple
    pub table_suffix: HashSet<String>,
    /// 过滤字段前缀 example: addFieldPrefix("is_") result: is_deleted -> deleted
    pub field_prefix: HashSet<String>,
    /// 过滤字段后缀 example: addFieldSuffix("_flag") result: deleted_flag -> deleted
    pub field_suffix: HashSet<String>,
    /// 需要包含的表名，允许正则表达式（与exclude二选一配置） 当enableSqlFilter为true时，正则表达式无效.
    pub include: HashSet<String>,
    /// 需要排除的表名，允许正则表达式 当enableSqlFilter为true时，正则表达式无效
    pub exclude: HashSet<String>,
    /// 启用sql过滤，语法不能支持使用sql过滤表的话，可以考虑关闭此开关. 默认 true
    pub enable_sql_filter: bool,
    /// 启用 schema 默认 false
    pub enable_schema: bool,
    pub entity: Entity,
    pub controller: Controller,
    pub mapper: Mapper,
    pub service: Service,
}
impl StrategyConfig {
    /// 表名过滤前缀
    pub fn start_with_table_prefix(&self, name: &str) -> bool {
        self.table_prefix.iter().any(|tp| name.starts_with(tp))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    /// 自定义继承的Entity类全称，带包名
    pub super_class: String,
    /// 自定义基础的Entity类，公共字段
    pub super_entity_columns: HashSet<String>,
    /// 自定义忽略字段 https://github.com/baomidou/generator/issues/46
    pub ignore_columns: HashSet<String>,
    /// 启动生成serialVersionUID
    pub serial_version_uid: bool,
    /// 【实体】是否生成字段常量（默认 false）
    /// -----------------------------------
    /// public static final String ID = "test_id";
    pub column_constant: bool,
    /// 【实体】是否为链式模型（默认 false）
    pub chain_mode: bool,
    /// 【实体】是否为lombok模型（默认 false）
    pub lombok: bool,
    /// Boolean类型字段是否移除is前缀（默认 false）
    /// 比如 : 数据库字段名称 : 'is_xxx',类型为 : tinyint.
    /// 在映射实体的时候则会去掉is,在实体类中映射最终结果为 xxx
    pub boolean_column_remove_is_prefix: bool,
    /// 是否生成实体时，生成字段注解（默认 false）
    pub enable_table_field_annotation: bool,
    /// 乐观锁字段名称(数据库字段)
    pub version_column_name: String,
    /// 乐观锁属性名称(实体字段)
    pub version_property_name: String,
    /// 逻辑删除字段名称(数据库字段)
    pub logic_delete_column_name: String,
    /// 逻辑删除属性名称(实体字段)
    pub logic_delete_property_name: String,
    /// 表字段填充
    pub table_fill_list: Vec<TableFill>,
    /// 数据库表映射到实体的命名策略，默认下划线转驼峰命名
    pub naming: NamingStrategy,
    /// 数据库表字段映射到实体的命名策略,未指定按照 naming 执行
    pub column_naming: Option<NamingStrategy>,
    /// 开启 ActiveRecord 模式（默认 false）
    pub active_record: bool,
    /// 指定生成的主键的ID类型
    pub id_type: Option<IdType>,
    /// 是否覆盖已有文件（默认 false）
    pub file_override: bool,
    /// 格式化文件名称
    pub format_filename: String,
}

impl TemplateRender for Entity {
    type Item = EntityData;

    fn render_data(&self, _table_info: &TableInfo) -> Result<Self::Item> {
        Ok(context_data::EntityDataBuilder::default()
            .id_type(self.id_type)
            .logic_delete_field_name(self.logic_delete_column_name.clone())
            .version_field_name(self.version_column_name.clone())
            .active_record(self.active_record)
            .entity_serial_version_uid(self.serial_version_uid)
            .entity_column_constant(self.column_constant)
            .entity_builder_model(self.chain_mode)
            .chain_model(self.chain_mode)
            .entity_lombok_model(self.lombok)
            .entity_boolean_column_remove_is_prefix(self.boolean_column_remove_is_prefix)
            .super_entity_class(utils::get_simple_name(&self.super_class))
            .build()?)
    }
}

impl Entity {
    pub fn name_convert(&self, strategy_config: StrategyConfig) -> impl NameConvert {
        DefaultNameConvert::new(strategy_config)
    }

    pub fn column_naming(&self) -> NamingStrategy {
        self.column_naming.unwrap_or(self.naming)
    }

    pub fn match_ingore_columns(&self, field_name: &str) -> bool {
        self.ignore_columns
            .iter()
            .any(|column| column.eq_ignore_ascii_case(field_name))
    }

    pub fn match_super_entity_columns(&self, field_name: &str) -> bool {
        self.super_entity_columns
            .iter()
            .any(|column| column.eq_ignore_ascii_case(field_name))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Controller {
    /// 自定义继承的Controller类全称，带包名
    pub super_class: String,
    ///生成 @RestController 控制器（默认 false）, @Controller -> @RestController
    pub rest_style: bool,
    ///驼峰转连字符（默认 false）,  @RequestMapping("/managerUserActionHistory") -> @RequestMapping("/manager-user-action-history")
    pub hyphen_style: bool,
    ///是否覆盖已有文件（默认 false）
    pub file_override: bool,
    /// 格式化文件名称
    pub format_filename: String,
}

impl TemplateRender for Controller {
    type Item = ControllerData;

    fn render_data(&self, table_info: &TableInfo) -> Result<Self::Item> {
        let super_class = if self.super_class.is_empty() {
            None
        } else {
            Some(self.super_class.to_string())
        };
        let data = context_data::ControllerDataBuilder::default()
            .controller_mapping_hyphen(table_info.get_entity_path().to_case(Case::Kebab))
            .controller_mapping_hyphen_style(self.hyphen_style)
            .rest_controller_style(self.rest_style)
            .super_controller_class_package(super_class.clone())
            .super_controller_class(utils::get_simple_name(
                &super_class.unwrap_or("".to_string()),
            ))
            .build()?;

        Ok(data)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mapper {
    /// 自定义继承的Mapper类全称，带包名, 默认 "com.baomidou.mybatisplus.core.mapper.BaseMapper"
    pub super_class: String,
    /// 是否添加 @Mapper 注解（默认 false）
    pub mapper_annotation: bool,
    /// 是否开启BaseResultMap（默认 false）
    pub base_result_map: bool,
    /// 是否开启baseColumnList（默认 false）
    pub base_column_list: bool,
    ///是否覆盖已有文件（默认 false）
    pub file_override: bool,
    /// 格式化 mapper 文件名称
    pub format_mapper_filename: String,
    /// 格式化 xml 实现类文件名称
    pub format_xml_filename: String,
}
impl TemplateRender for Mapper {
    type Item = MapperData;

    fn render_data(&self, _table_info: &TableInfo) -> Result<Self::Item> {
        let super_class = if self.super_class.is_empty() {
            "com.baomidou.mybatisplus.core.mapper.BaseMapper"
        } else {
            &self.super_class
        };

        let mapper_annotation_class = if self.mapper_annotation {
            "org.apache.ibatis.annotations.Mapper"
        } else {
            ""
        };

        Ok(context_data::MapperDataBuilder::default()
            .enable_cache(false)
            .cache_class_name("".to_string())
            .mapper_annotation_name(
                utils::get_simple_name(mapper_annotation_class).unwrap_or("".to_string()),
            )
            .mapper_annotation_class(mapper_annotation_class.to_string())
            .base_result_map(self.base_result_map)
            .base_column_list(self.base_column_list)
            .super_mapper_class_package(super_class.to_string())
            .super_mapper_class(utils::get_simple_name(super_class))
            .build()?)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    ///自定义继承的Service类全称，带包名, 默认: "com.baomidou.mybatisplus.extension.service.IService"
    pub super_service_class: String,
    /// 自定义继承的ServiceImpl类全称，带包名，默认："com.baomidou.mybatisplus.extension.service.impl.ServiceImpl"
    pub super_service_impl_class: String,
    ///是否覆盖已有文件（默认 false）
    pub file_override: bool,
    /// 格式化 service 接口文件名称
    pub format_service_filename: String,
    /// 格式化 service 实现类文件名称
    pub format_service_impl_filename: String,
}
impl TemplateRender for Service {
    type Item = ServiceData;

    fn render_data(&self, _table_info: &TableInfo) -> Result<Self::Item> {
        let super_class = if self.super_service_class.is_empty() {
            "com.baomidou.mybatisplus.extension.service.IService"
        } else {
            &self.super_service_class
        };
        let super_class_impl = if self.super_service_impl_class.is_empty() {
            "com.baomidou.mybatisplus.extension.service.impl.ServiceImpl"
        } else {
            &self.super_service_impl_class
        };
        Ok(context_data::ServiceDataBuilder::default()
            .super_service_class_package(super_class.to_string())
            .super_service_class(utils::get_simple_name(super_class))
            .super_service_impl_class_package(super_class_impl.to_string())
            .super_service_impl_class(utils::get_simple_name(super_class_impl))
            .build()?)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum OutputFile {
    Entity,
    Service,
    ServiceImpl,
    Mapper,
    Xml,
    Controller,
    Parent,
}

impl Display for OutputFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            OutputFile::Entity => "Entity",
            OutputFile::Service => "Service",
            OutputFile::ServiceImpl => "ServiceImpl",
            OutputFile::Mapper => "Mapper",
            OutputFile::Xml => "Xml",
            OutputFile::Controller => "Controller",
            OutputFile::Parent => "Parent",
        };

        write!(f, "{}", res)
    }
}

impl OutputFile {
    pub fn as_str(&self) -> &str {
        match self {
            OutputFile::Entity => "Entity",
            OutputFile::Service => "Service",
            OutputFile::ServiceImpl => "ServiceImpl",
            OutputFile::Mapper => "Mapper",
            OutputFile::Xml => "Xml",
            OutputFile::Controller => "Controller",
            OutputFile::Parent => "Parent",
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Copy, Clone)]
pub enum NamingStrategy {
    NoChange,
    #[default]
    UnderlineToCamel,
}

impl FromStr for NamingStrategy {
    type Err = SerializeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "noChange" => Ok(Self::NoChange),
            "underlineToCamel" => Ok(Self::UnderlineToCamel),
            _ => Err(format!("不支持的命名策略: {}", s).into()),
        }
    }
}

impl NamingStrategy {
    pub fn convert(&self, property_name: &str) -> String {
        property_name.to_string()
    }
}

impl NamingStrategy {
    pub fn remove_prefix(name: &str, prefix: &HashSet<String>) -> String {
        if name.is_empty() {
            return "".to_string();
        }

        prefix
            .iter()
            .find(|p| name.to_lowercase().starts_with(&p.to_lowercase()))
            .map(|p| name[p.len()..].to_string())
            .unwrap_or(name.to_string())
    }

    pub fn remove_suffix(name: &str, suffix: &HashSet<String>) -> String {
        suffix
            .iter()
            .find(|p| name.to_lowercase().starts_with(&p.to_lowercase()))
            .map(|p| name[0..(name.len() - p.len())].to_string())
            .unwrap_or(name.to_string())
    }

    pub fn underline_to_camel(name: &str) -> String {
        name.to_case(Case::Camel)
    }

    pub fn capital(name: &str) -> String {
        name[0..1].to_uppercase() + name[1..].as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TableFill {
    pub property_name: String,
    pub field_fill: FieldFill,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum FieldFill {
    ///默认不处理
    DEFAULT,
    /// 插入时填充字段
    INSERT,
    ///更新时填充字段
    UPDATE,
    ///插入和更新时填充字段
    INSERT_UPDATE,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum IdType {
    /// 数据库ID自增, 该类型请确保数据库设置了 ID自增 否则无效
    AUTO,
    /// 该类型为未设置主键类型(注解里等于跟随全局,全局里约等于 INPUT)
    NONE,
    /// 用户输入ID, 该类型可以通过自己注册自动填充插件进行填充
    INPUT,
    /// 以下2种类型、只有当插入对象ID 为空，才自动填充。
    ///分配ID (主键类型为number或string）, 默认实现类 com.baomidou.mybatisplus.core.incrementer.DefaultIdentifierGenerator(雪花算法)
    ASSIGN_ID,
    ///分配UUID (主键类型为 string) 默认实现类 com.baomidou.mybatisplus.core.incrementer.DefaultIdentifierGenerator(UUID.replace("-",""))
    ASSIGN_UUID,
}

impl FromStr for IdType {
    type Err = SerializeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "AUTO" => Ok(Self::AUTO),
            "NONE" => Ok(Self::NONE),
            "INPUT" => Ok(Self::INPUT),
            "ASSIGN_ID" => Ok(Self::ASSIGN_ID),
            "ASSIGN_UUID" => Ok(Self::ASSIGN_UUID),
            _ => Err(format!("不支持的主键类型: {}", s).into()),
        }
    }
}
