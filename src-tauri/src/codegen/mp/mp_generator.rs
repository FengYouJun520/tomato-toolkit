use sqlx::types::chrono::Local;
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
    context_data::{self, ContextData, TemplateRender},
    db_query::MpConfig,
    model::TableInfo,
};

pub struct MpGenerator {
    /// 配置信息
    config: ConfigBuilder,
    /// 模板渲染引擎实例
    tera_template: Tera,
    /// 存储模板名称列表
    template_names: HashMap<String, PathBuf>,
}

impl MpGenerator {
    pub fn new(resource_path: PathBuf, config: MpConfig) -> Result<Self> {
        let config = ConfigBuilder::new(config, resource_path.clone());
        // 进行一些必要的初始化
        let mut generate = Self {
            config,
            tera_template: Tera::default(),
            template_names: HashMap::new(),
        };

        generate.init_template(resource_path)?;

        Ok(generate)
    }

    pub fn init_template(&mut self, resource_path: PathBuf) -> Result<()> {
        let config = &self.config;
        let tc = &config.template_config;
        // 模板名称列表
        let mut tempate_names = HashMap::new();
        let is_kotlin = self.config.global_config.kotlin;

        if let Some(entity) = tc.get_entity(
            is_kotlin,
            resource_path.join("entity.java"),
            resource_path.join("entity.kt.java"),
        ) {
            self.tera_template
                .add_template_file(&entity, entity.to_str())?;
            tempate_names.insert(OutputFile::Entity.to_string(), entity);
        }
        if let Some(controller) = tc.get_controller(resource_path.join("controller.java")) {
            self.tera_template
                .add_template_file(&controller, controller.to_str())?;
            tempate_names.insert(OutputFile::Controller.to_string(), controller);
        }
        if let Some(mapper) = tc.get_mapper(resource_path.join("mapper.java")) {
            self.tera_template
                .add_template_file(&mapper, mapper.to_str())?;
            tempate_names.insert(OutputFile::Mapper.to_string(), mapper);
        }
        if let Some(xml) = tc.get_xml(resource_path.join("mapper.xml")) {
            self.tera_template.add_template_file(&xml, xml.to_str())?;
            tempate_names.insert(OutputFile::Xml.to_string(), xml);
        }
        if let Some(service) = tc.get_service(resource_path.join("service.java")) {
            self.tera_template
                .add_template_file(&service, service.to_str())?;
            tempate_names.insert(OutputFile::Service.to_string(), service);
        }
        if let Some(service_impl) = tc.get_service_impl(resource_path.join("serviceImpl.java")) {
            self.tera_template
                .add_template_file(&service_impl, service_impl.to_str())?;
            tempate_names.insert(OutputFile::ServiceImpl.to_string(), service_impl);
        }

        self.template_names = tempate_names;

        // 添加自定义模板路径
        if let Some(ref injection) = config.injection_config {
            for custom_file in injection.custom_files.iter() {
                self.tera_template.add_template_file(
                    custom_file.template_path.clone(),
                    Some(&custom_file.template_path.to_string_lossy()),
                )?;
            }
        }

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

    /// 生成预览数据
    pub async fn preview(&mut self) -> Result<serde_json::Value> {
        let table_infos = self.config.query_tables().await?;
        let mut contexts = tera::Context::new();
        for table_info in table_infos {
            // 转化为模板数据
            let context_data = self.build_context_data(&table_info)?;
            contexts.extend(tera::Context::from_serialize(context_data)?);
        }

        if let Some(ref injection) = self.config.injection_config {
            contexts.extend(tera::Context::from_serialize(&injection.custom_map)?);
        }

        Ok(contexts.into_json())
    }

    pub async fn batch_output(&mut self) -> Result<()> {
        let table_infos = self.config.query_tables().await?;

        for table_info in table_infos {
            // 转化为模板数据
            let data = self.build_context_data(&table_info)?;
            let mut context = tera::Context::from_serialize(data)?;

            self.output_custom_file(&table_info, &mut context)?;
            self.output_entity(&table_info, &context)?;
            self.output_mapper(&table_info, &context)?;
            self.output_service(&table_info, &context)?;
            self.output_controller(&table_info, &context)?;
        }
        Ok(())
    }

    pub fn build_context_data(&mut self, table_info: &TableInfo) -> Result<ContextData> {
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
            .date(Local::now().format(&global.comment_date).to_string())
            .schema_name("".to_string())
            .table(table_info.clone())
            .entity(table_info.entity_name.clone())
            .build()?;

        Ok(data)
    }

    /// 生成实体文件
    fn output_entity(&self, table_info: &TableInfo, context: &tera::Context) -> Result<()> {
        let entity_name = &table_info.entity_name;
        let entity_path = self
            .config
            .path_info
            .get(&OutputFile::Entity)
            .ok_or(SerializeError::from("实体模板文件路径未找到"))?;

        let file_override = self.config.strategy_config.entity.file_override;
        if !entity_name.is_empty() && !entity_path.to_string_lossy().is_empty() {
            if let Some(entity) = self.get_template_name(OutputFile::Entity.as_str()) {
                let entity_file =
                    entity_path.join(format!("{}{}", entity_name, self.file_suffix()));
                self.output_file(
                    entity_file,
                    &entity.to_string_lossy(),
                    context,
                    file_override,
                )?;
            }
        }

        Ok(())
    }

    /// 生成mapper文件
    fn output_mapper(&self, table_info: &TableInfo, context: &tera::Context) -> Result<()> {
        let suffix = self.file_suffix();

        // 生成MpMapper.java
        let mapper_name = &table_info.mapper_name;
        let mapper_path = self
            .config
            .path_info
            .get(&OutputFile::Mapper)
            .ok_or(SerializeError::from("mapper模板文件路径未找到"))?;

        let file_override = self.config.strategy_config.mapper.file_override;
        if !mapper_name.is_empty() && !mapper_path.to_string_lossy().is_empty() {
            if let Some(mapper) = self.get_template_name(OutputFile::Mapper.as_str()) {
                let mapper_file = mapper_path.join(format!("{}{}", mapper_name, suffix));
                self.output_file(
                    mapper_file,
                    &mapper.to_string_lossy(),
                    context,
                    file_override,
                )?;
            }
        }

        // 生成MpMapper.xml文件
        let xml_name = &table_info.xml_name;
        let xml_path = self
            .config
            .path_info
            .get(&OutputFile::Xml)
            .ok_or(SerializeError::from("xml模板文件路径未找到"))?;
        if !xml_name.is_empty() && !xml_path.to_string_lossy().is_empty() {
            if let Some(xml) = self.get_template_name(OutputFile::Xml.as_str()) {
                let xml_file = xml_path.join(format!("{xml_name}.xml"));
                self.output_file(xml_file, &xml.to_string_lossy(), context, file_override)?;
            }
        }

        Ok(())
    }

    /// 生成Service文件
    fn output_service(&self, table_info: &TableInfo, context: &tera::Context) -> Result<()> {
        let suffix = self.file_suffix();

        // 生成MpService.java
        let service_name = &table_info.service_name;
        let service_path = self
            .config
            .path_info
            .get(&OutputFile::Service)
            .ok_or(SerializeError::from("service模板文件路径未找到"))?;

        let file_override = self.config.strategy_config.service.file_override;
        if !service_name.is_empty() && !service_path.to_string_lossy().is_empty() {
            if let Some(service) = self.get_template_name(OutputFile::Service.as_str()) {
                let service_file = service_path.join(format!("{}{}", service_name, suffix));
                self.output_file(
                    service_file,
                    &service.to_string_lossy(),
                    context,
                    file_override,
                )?;
            }
        }

        // 生成MpServiceImpl.java文件
        let service_impl_name = &table_info.service_impl_name;
        let service_impl_path = self
            .config
            .path_info
            .get(&OutputFile::ServiceImpl)
            .ok_or(SerializeError::from("ServiceImpl模板文件路径未找到"))?;
        if !service_impl_name.is_empty() && !service_impl_path.to_string_lossy().is_empty() {
            if let Some(service_impl) = self.get_template_name(OutputFile::ServiceImpl.as_str()) {
                let service_impl_file =
                    service_impl_path.join(format!("{}{}", service_impl_name, suffix));
                self.output_file(
                    service_impl_file,
                    &service_impl.to_string_lossy(),
                    context,
                    file_override,
                )?;
            }
        }

        Ok(())
    }

    /// 生成Controller文件
    fn output_controller(&self, table_info: &TableInfo, context: &tera::Context) -> Result<()> {
        let controller_name = &table_info.controller_name;
        let controller_path = self
            .config
            .path_info
            .get(&OutputFile::Controller)
            .ok_or(SerializeError::from("Controller模板文件路径未找到"))?;

        let file_override = self.config.strategy_config.controller.file_override;
        if !controller_name.is_empty() && !controller_path.to_string_lossy().is_empty() {
            if let Some(controller) = self.get_template_name(OutputFile::Controller.as_str()) {
                let controller_file =
                    controller_path.join(format!("{}{}", controller_name, self.file_suffix()));
                self.output_file(
                    controller_file,
                    &controller.to_string_lossy(),
                    context,
                    file_override,
                )?;
            }
        }

        Ok(())
    }

    /// 添加自定义属性
    fn before_output_file(
        &self,
        custom_map: &HashMap<String, serde_json::Value>,
        _table_info: &TableInfo,
        context: &mut tera::Context,
    ) -> Result<()> {
        if !custom_map.is_empty() {
            context.extend(tera::Context::from_serialize(custom_map)?);
        }

        Ok(())
    }

    /// 输出自定义文件
    fn output_custom_file(
        &mut self,
        table_info: &TableInfo,
        context: &mut tera::Context,
    ) -> Result<()> {
        let Some(ref injection) =  self.config.injection_config else {
            return Ok(());
        };

        self.before_output_file(&injection.custom_map, table_info, context)?;

        let entity_name = &table_info.entity_name;
        let parent_path = self.config.path_info.get(&OutputFile::Parent);

        for file in injection.custom_files.iter() {
            let mut file_path = if file.file_path.to_string_lossy().is_empty() {
                parent_path.cloned().unwrap_or(PathBuf::from(""))
            } else {
                file.file_path.clone()
            };
            if !file.package_name.is_empty() {
                file_path.push(file.package_name.replace('.', "/"));
            }

            if file.add_entity_prefix {
                file_path.push(format!("{}{}", entity_name, &file.file_name));
            } else {
                file_path.push(&file.file_name);
            }

            self.output_file(
                file_path,
                &file.template_path.to_string_lossy(),
                context,
                file.file_override,
            )?;
        }
        Ok(())
    }

    fn file_suffix(&self) -> &str {
        if self.config.global_config.kotlin {
            ".kt"
        } else {
            ".java"
        }
    }

    /// 获取模板名称
    fn get_template_name(&self, output_file: &str) -> Option<&PathBuf> {
        self.template_names.get(output_file)
    }

    /// 输出渲染后的文件
    ///
    /// - file: 文件路径
    /// - template_path: 模板名称
    /// - context: 模板上下文数据
    /// - file_override: 是否覆盖已有文件
    fn output_file<P: AsRef<Path>>(
        &self,
        file: P,
        template_name: &str,
        context: &tera::Context,
        file_override: bool,
    ) -> Result<()> {
        if self.is_create(&file, file_override) {
            let exists = file.as_ref().exists();
            if !exists {
                let parent_file = file.as_ref().parent().unwrap_or(file.as_ref());
                std::fs::create_dir_all(parent_file)?;
            }

            self.write(file, template_name, context)?;
        }
        Ok(())
    }

    /// 是否需要覆盖文件
    fn is_create<P: AsRef<Path>>(&self, file: P, file_override: bool) -> bool {
        !file.as_ref().exists() || file_override
    }

    /// 渲染模板引擎数据
    ///
    /// - file: 文件路径
    /// - template_path: 模板名称
    /// - context: 模板上下文数据
    fn write<P: AsRef<Path>>(
        &self,
        file: P,
        template_name: &str,
        context: &tera::Context,
    ) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file)?;

        self.tera_template.render_to(template_name, context, file)?;
        Ok(())
    }
}
