use std::{collections::HashMap, path::PathBuf};
use tera::Tera;

use crate::error::Result;

use super::{
    config::OutputFile,
    config_builder::ConfigBuilder,
    context_data::{ContextData, TemplateRender},
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
        for entry in self.template_path.iter() {
            println!("key: {:?}, path: {}", entry.0, entry.1.display());
        }

        Ok(())
    }

    pub async fn execute(&mut self) -> Result<()> {
        // 执行输出
        self.batch_output().await
    }

    pub async fn batch_output(&mut self) -> Result<()> {
        let table_infos = self.config.query_tables().await?;

        for table_info in table_infos {
            // 转化为模板数据
            self.build_context(&table_info)?;
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

        let data = ContextData {
            controller_data,
            mapper_data,
            service_data,
            entity_data,
            config: self.config.clone(),
            package: self.config.package_config.get_package_infos(),
            author: global.author.clone(),
            kotlin: global.kotlin,
            swagger: global.swagger,
            springdoc: global.springdoc,
            date: global.comment_date.clone(),
            schema_name: "".to_string(),
            table: table_info.clone(),
            entity: table_info.name.clone(),
        };

        let context = tera::Context::from_serialize(data)?;
        Ok(context)
    }
}
