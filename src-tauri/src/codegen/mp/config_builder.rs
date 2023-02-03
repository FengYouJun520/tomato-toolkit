use serde::Serialize;

use crate::error::Result;

use super::{
    config::{
        DataSourceConfig, FieldFill, GlobalConfig, InjectConfig, OutputFile, PackageConfig,
        StrategyConfig, TemplateConfig,
    },
    db_query::{DbQuery, MpConfig},
    keywords::DefaultKeywordHandler,
    model::{self, TableField, TableInfo},
};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigBuilder {
    pub datasource_config: DataSourceConfig,
    pub template_config: TemplateConfig,
    pub strategy_config: StrategyConfig,
    pub package_config: PackageConfig,
    pub global_config: GlobalConfig,
    pub injection_config: Option<InjectConfig>,
    pub table_infos: Vec<TableInfo>,
    pub path_info: HashMap<String, PathBuf>,
    #[serde(skip)]
    pub db_query: Arc<dyn DbQuery>,
}

impl ConfigBuilder {
    pub fn new(config: MpConfig, resource_path: PathBuf) -> Self {
        let db_query = config.datasource.db_query();
        let global = &config.global;
        let template = &config.template;
        let package = &config.package;
        let path_info = PathInfoHandler::new(global, template, package, resource_path)
            .get_path_info()
            .clone();

        Self {
            table_infos: vec![],
            path_info,
            db_query,
            datasource_config: config.datasource,
            global_config: config.global,
            injection_config: config.injection,
            package_config: config.package,
            strategy_config: config.strategy,
            template_config: config.template,
        }
    }

    pub async fn query_tables(&mut self) -> Result<Vec<TableInfo>> {
        let tables = self.db_query.query_tables(self).await?;
        let mut table_infos = vec![];

        for table in tables {
            let mut table_info = TableInfo::new(self, table.comment, table.name);
            self.convert_table_fields(&mut table_info).await?;
            table_infos.push(table_info);
        }

        self.table_infos = table_infos.clone();

        Ok(table_infos)
    }

    async fn convert_table_fields(&self, table_info: &mut TableInfo) -> Result<()> {
        let fields = self.db_query.query_table_fields(table_info, self).await?;
        let strategy_config = &self.strategy_config;
        let entity = &strategy_config.entity;

        let fields: Result<Vec<TableField>> = fields
            .into_iter()
            .map(|field| {
                // 属性名
                let property_name = entity
                    .name_convert(strategy_config.clone())
                    .property_name_convert(&field)?;

                // 字段类型
                let column_type = self
                    .datasource_config
                    .get_type_convert()
                    .type_convert(&self.global_config, &field);

                let keyword_handler =
                    DefaultKeywordHandler::get_keyword_handler(self.datasource_config.db_type());

                // 是否是关键字
                let is_keywords = keyword_handler
                    .as_ref()
                    .map(|handler| handler.is_keyword(&field.name))
                    .unwrap_or_default();

                // 关键字处理，设置列名
                let column_name = if is_keywords {
                    keyword_handler
                        .map(|handler| handler.format_column(&field.name))
                        .unwrap_or(field.name.to_string())
                } else {
                    field.name.to_string()
                };

                // 注解注释处理
                let annotation_column_name = if is_keywords && column_name.starts_with('\"') {
                    format!(r#"\"{}\""#, field.name)
                } else {
                    column_name.to_string()
                };

                let mut field = model::TableFieldBuilder::default()
                    .name(field.name.clone())
                    .r#type(field.r#type)
                    .column_type(column_type)
                    .convert(false)
                    .fill(FieldFill::DEFAULT)
                    .property_name(property_name.clone())
                    .capital_name(property_name.clone())
                    .column_name(column_name)
                    .annotation_column_name(annotation_column_name)
                    .entity(self.strategy_config.entity.clone())
                    .datasource_config(self.datasource_config.clone())
                    .global_config(self.global_config.clone())
                    .comment(field.comment.unwrap_or_default())
                    .custom_map(HashMap::new())
                    .have_primary(field.key_flag)
                    .key_flag(field.key_flag)
                    .version_field(false)
                    .logic_delete_field(false)
                    .key_identity_flag(field.auto_increment)
                    .build()?;

                // 设置字段填充
                field.set_fill();
                // 设置属性名称
                field.set_property_name(&property_name, column_type);
                // 设置乐观锁字段
                field.set_version_field();
                // 设置逻辑删除字段
                field.set_logic_delete_field();

                Ok(field)
            })
            .collect();

        for field in fields? {
            table_info.add_field(field);
        }

        table_info.process_table()?;

        Ok(())
    }
}

struct PathInfoHandler<'a> {
    out_dir: PathBuf,
    package: &'a PackageConfig,
    path_info: HashMap<String, PathBuf>,
}

impl<'a> PathInfoHandler<'a> {
    pub fn new(
        global: &'a GlobalConfig,
        template: &'a TemplateConfig,
        package: &'a PackageConfig,
        resource_path: PathBuf,
    ) -> Self {
        let mut path_info_handler = Self {
            out_dir: global.output_dir.clone(),
            package,
            path_info: HashMap::new(),
        };

        path_info_handler.set_default_path_info(global, template, resource_path);

        // 覆盖自定义路径
        let Some(path_info) = package.get_path_info() else  {
            return path_info_handler;
        };

        // TODO: 存储自定义路径信息
        // e.g. "Controller": "D://com/blog/model/controller",

        if !path_info.is_empty() {
            path_info_handler.path_info.extend(path_info);
        }

        path_info_handler
    }

    pub fn get_path_info(&self) -> &HashMap<String, PathBuf> {
        &self.path_info
    }

    fn set_default_path_info(
        &mut self,
        global: &'a GlobalConfig,
        template: &'a TemplateConfig,
        resource_path: PathBuf,
    ) {
        self.put_path_info(
            template.get_entity(
                global.kotlin,
                resource_path.join("entity.java"),
                resource_path.join("entity.kt.java"),
            ),
            OutputFile::Entity.as_str(),
            "Entity",
        );
        self.put_path_info(
            template.get_controller(resource_path.join("controller.java")),
            OutputFile::Controller.as_str(),
            "Controller",
        );
        self.put_path_info(
            template.get_mapper(resource_path.join("mapper.java")),
            OutputFile::Mapper.as_str(),
            "Mapper",
        );
        self.put_path_info(
            template.get_controller(resource_path.join("mapper.xml")),
            OutputFile::Xml.as_str(),
            "Xml",
        );
        self.put_path_info(
            template.get_service(resource_path.join("service.java")),
            OutputFile::Service.as_str(),
            "Service",
        );
        self.put_path_info(
            template.get_service_impl(resource_path.join("service.impl.java")),
            OutputFile::ServiceImpl.as_str(),
            "ServiceImpl",
        );
        self.add_path_info(OutputFile::Parent.as_str(), "Parent");
    }

    fn put_path_info(&mut self, template: Option<PathBuf>, output_file: &str, module: &str) {
        let Some(template) = template else {
            return
        };
        if template.to_string_lossy().is_empty() {
            return;
        }

        self.add_path_info(output_file, module);
    }

    pub fn add_path_info(&mut self, output_file: &str, module: &str) {
        let module = self.package.get_package_info(module).unwrap_or("".into());
        let entry = self.path_info.entry(output_file.to_string());
        let out = PathInfoHandler::join_path(self.out_dir.clone(), &module);
        entry.or_insert(out);
    }

    fn join_path(out_dir: PathBuf, module: &str) -> PathBuf {
        out_dir.join(module.replace('.', "/"))
    }
}
