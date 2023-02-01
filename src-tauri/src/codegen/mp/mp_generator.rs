use std::{
    collections::HashMap,
    fs::OpenOptions,
    path::{Path, PathBuf},
};
use tera::Tera;

use crate::error::{Result, SerializeError};

use super::{
    config::OutputFile,
    config_builder::ConfigBuilder,
    context_data::{self, TemplateRender},
    db_query::MpConfig,
    model::TableInfo,
};

pub struct MpGenerator {
    config: ConfigBuilder,
    tera_template: Tera,
    template_path: HashMap<OutputFile, PathBuf>,
}

impl MpGenerator {
    pub fn new(resource_path: PathBuf, config: MpConfig) -> Result<Self> {
        let config = ConfigBuilder::new(config, resource_path.clone());
        // 进行一些必要的初始化
        let mut generate = Self {
            config,
            tera_template: Tera::default(),
            template_path: HashMap::new(),
        };

        generate.init_template(resource_path)?;

        Ok(generate)
    }

    pub fn init_template(&mut self, resource_path: PathBuf) -> Result<()> {
        let tc = &self.config.template_config;
        let mut tempate_path = HashMap::new();
        let is_kotlin = self.config.global_config.kotlin;

        if let Some(entity) = tc.get_entity(
            is_kotlin,
            resource_path.join("entity.java"),
            resource_path.join("entity.kt.java"),
        ) {
            self.tera_template
                .add_template_file(&entity, entity.to_str())?;
            tempate_path.insert(OutputFile::Entity, entity);
        }
        if let Some(controller) = tc.get_controller(resource_path.join("controller.java")) {
            self.tera_template
                .add_template_file(&controller, controller.to_str())?;
            tempate_path.insert(OutputFile::Controller, controller);
        }
        if let Some(mapper) = tc.get_mapper(resource_path.join("mapper.java")) {
            self.tera_template
                .add_template_file(&mapper, mapper.to_str())?;
            tempate_path.insert(OutputFile::Mapper, mapper);
        }
        if let Some(xml) = tc.get_xml(resource_path.join("mapper.xml")) {
            self.tera_template.add_template_file(&xml, xml.to_str())?;
            tempate_path.insert(OutputFile::Xml, xml);
        }
        if let Some(service) = tc.get_service(resource_path.join("service.java")) {
            self.tera_template
                .add_template_file(&service, service.to_str())?;
            tempate_path.insert(OutputFile::Service, service);
        }
        if let Some(service_impl) = tc.get_service_impl(resource_path.join("serviceImpl.java")) {
            self.tera_template
                .add_template_file(&service_impl, service_impl.to_str())?;
            tempate_path.insert(OutputFile::ServiceImpl, service_impl);
        }

        self.template_path = tempate_path;

        Ok(())
    }

    pub async fn execute(&mut self) -> Result<()> {
        // 执行输出
        self.batch_output().await?;
        if self.config.global_config.open {
            open::that(&self.config.global_config.output_dir)?;
        }

        Ok(())
    }

    pub async fn batch_output(&mut self) -> Result<()> {
        let table_infos = self.config.query_tables().await?;

        for table_info in table_infos {
            // 转化为模板数据
            let context = self.build_context(&table_info)?;
            self.output_entity(&table_info, &context)?;
        }
        Ok(())
    }

    pub fn build_context(&mut self, table_info: &TableInfo) -> Result<tera::Context> {
        let strategy = &self.config.strategy_config;
        let global = &self.config.global_config;
        let controller_data = strategy.controller.render_data(table_info)?;
        let mapper_data = strategy.mapper.render_data(table_info)?;
        let service_data = strategy.service.render_data(table_info)?;
        let entity_data = strategy.entity.render_data(table_info)?;

        let data = context_data::ContextDataBuilder::default()
            .controller_data(controller_data)
            .mapper_data(mapper_data)
            .service_data(service_data)
            .entity_data(entity_data)
            .config(self.config.clone())
            .package(self.config.package_config.get_package_infos())
            .author(global.author.clone())
            .kotlin(global.kotlin)
            .swagger(global.swagger)
            .springdoc(global.springdoc)
            .date(global.comment_date.clone())
            .schema_name("".to_string())
            .table(table_info.clone())
            .entity(table_info.name.clone())
            .build()?;

        let context = tera::Context::from_serialize(data)?;
        Ok(context)
    }

    fn output_entity(&self, table_info: &TableInfo, context: &tera::Context) -> Result<()> {
        let entity_name = &table_info.entity_name;
        let entity_path = self
            .config
            .path_info
            .get(&OutputFile::Entity)
            .ok_or(SerializeError::from("实体模板文件路径未找到"))?;

        if !entity_name.is_empty() && !entity_path.to_string_lossy().is_empty() {
            if let Some(entity) = self.get_template_path(OutputFile::Entity) {
                let suffix = if self.config.global_config.kotlin {
                    ".kt"
                } else {
                    ".java"
                };
                let entity_file = entity_path.join(format!("{}{}", entity_name, suffix));
                self.output_file(
                    entity_file,
                    context,
                    entity.to_path_buf(),
                    self.config.strategy_config.entity.file_override,
                )?;
            }
        }

        Ok(())
    }

    fn get_template_path(&self, output_file: OutputFile) -> Option<&PathBuf> {
        self.template_path.get(&output_file)
    }

    fn output_file<P: AsRef<Path>>(
        &self,
        file: P,
        context: &tera::Context,
        template_path: P,
        file_override: bool,
    ) -> Result<()> {
        if self.is_create(&file, file_override) {
            let exists = file.as_ref().exists();
            if !exists {
                let parent_file = file.as_ref().parent().unwrap_or(file.as_ref());
                std::fs::create_dir_all(parent_file)?;
            }

            self.write(file, template_path, context)?;
        }
        Ok(())
    }

    fn is_create<P: AsRef<Path>>(&self, file: P, file_override: bool) -> bool {
        // if file.as_ref().exists() && !file_override.unwrap_or_default() {}
        !file.as_ref().exists() || file_override
    }

    fn write<P: AsRef<Path>>(
        &self,
        file: P,
        template_path: P,
        context: &tera::Context,
    ) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file)?;

        let template_path =
            template_path
                .as_ref()
                .to_str()
                .ok_or(SerializeError::from(format!(
                    "模板文件路径不存在: {:?}",
                    template_path.as_ref()
                )))?;
        self.tera_template.render_to(template_path, context, file)?;
        Ok(())
    }
}
