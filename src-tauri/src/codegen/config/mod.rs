use serde::Deserialize;
use sqlx::{
    any::AnyConnectOptions, mssql::MssqlConnectOptions, mysql::MySqlConnectOptions,
    postgres::PgConnectOptions, sqlite::SqliteConnectOptions, AnyConnection, ConnectOptions,
};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    str::FromStr,
};

use crate::error::{Result, SerializeError};

use super::protocol::types::DateType;

#[derive(Debug, Deserialize)]
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
    pub async fn connect(&self) -> Result<AnyConnection> {
        let options: AnyConnectOptions = match self.r#type.as_ref() {
            "mysql" => MySqlConnectOptions::new()
                .host(&self.host)
                .port(self.port)
                .username(&self.username)
                .password(&self.password)
                .into(),
            "sqlite" => SqliteConnectOptions::new().filename(&self.database).into(),
            "mssql" => MssqlConnectOptions::new()
                .host(&self.host)
                .port(self.port)
                .database(&self.database)
                .username(&self.username)
                .password(&self.password)
                .into(),
            "postgres" => PgConnectOptions::new()
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
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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
    /// 路径配置信息
    pub pathinfo: HashMap<OutputFile, String>,
    /// 包配置信息
    pub package_infos: HashMap<String, String>,
}

impl PackageConfig {
    pub fn get_package_infos(&self) -> HashMap<String, String> {
        if !self.package_infos.is_empty() {
            self.package_infos.clone()
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
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateConfig {
    /// 禁用所有模板
    pub disable: bool,
    /// 设置实体模板路径
    pub entity: String,
    /// 设置实体模板路径(kotlin模板)
    pub entity_kt: String,
    /// 设置控制器模板路径
    pub controller: String,
    /// 设置Mapper模板路径
    pub mapper: String,
    /// 设置MapperXml模板路径
    pub xml: String,
    /// 设置Service模板路径
    pub service: String,
    /// 设置ServiceImpl模板路径
    pub service_impl: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InjectConfig {}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    /// 自定义继承的Entity类全称，带包名
    pub super_class: Option<String>,
    /// 自定义基础的Entity类，公共字段
    pub super_entity_columns: HashSet<String>,
    /// 自定义忽略字段 https://github.com/baomidou/generator/issues/46
    pub ignore_columns: HashSet<String>,
    /// 禁用生成serialVersionUID
    pub disable_serial_version_uid: bool,
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
    pub table_field_annotation_enable: bool,
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
    pub id_type: IdType,
    /// 是否覆盖已有文件（默认 false）
    pub file_override: bool,
    /// 格式化文件名称
    pub format_filename: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Controller {
    ///生成 @RestController 控制器（默认 false）, @Controller -> @RestController
    pub rest_style: bool,
    ///驼峰转连字符（默认 false）,  @RequestMapping("/managerUserActionHistory") -> @RequestMapping("/manager-user-action-history")
    pub hyphen_style: bool,
    /// 自定义继承的Controller类全称，带包名
    pub super_class: Option<String>,
    ///是否覆盖已有文件（默认 false）
    pub file_override: bool,
    /// 格式化文件名称
    pub format_filename: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapper {
    /// 自定义继承的Mapper类全称，带包名, 默认 "com.baomidou.mybatisplus.core.mapper.BaseMapper"
    pub super_class: String,
    /// 是否添加 @Mapper 注解（默认 false）
    pub mapper_annotation: bool,
    /// Mapper标记注解
    pub mapper_annotation_class: Option<String>,
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    ///自定义继承的Service类全称，带包名, 默认: "com.baomidou.mybatisplus.extension.service.IService"
    pub super_service_class: Option<String>,
    /// 自定义继承的ServiceImpl类全称，带包名，默认："com.baomidou.mybatisplus.extension.service.impl.ServiceImpl"
    pub super_service_impl_class: Option<String>,
    ///是否覆盖已有文件（默认 false）
    pub file_override: bool,
    /// 格式化 service 接口文件名称
    pub format_service_filename: String,
    /// 格式化 service 实现类文件名称
    pub format_service_impl_filename: String,
}

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum OutputFile {
    Entity,
    Service,
    ServiceImpl,
    Mapper,
    Xml,
    Controller,
    Parent,
}

impl FromStr for OutputFile {
    type Err = SerializeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "entity" => Ok(Self::Entity),
            "service" => Ok(Self::Service),
            "serviceImpl" => Ok(Self::ServiceImpl),
            "mapper" => Ok(Self::Mapper),
            "xml" => Ok(Self::Xml),
            "controller" => Ok(Self::Controller),
            "parent" => Ok(Self::Parent),
            _ => Err(format!("不支持的输出文件类型: {}", s).into()),
        }
    }
}

#[derive(Default, Deserialize)]
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableFill {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
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
            "auto" => Ok(Self::AUTO),
            "none" => Ok(Self::NONE),
            "input" => Ok(Self::INPUT),
            "assignId" => Ok(Self::ASSIGN_ID),
            "assignUuid" => Ok(Self::ASSIGN_UUID),
            _ => Err(format!("不支持的主键类型: {}", s).into()),
        }
    }
}
