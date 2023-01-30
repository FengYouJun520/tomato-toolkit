use crate::error::Result;

use super::{
    config::{
        DataSourceConfig, GlobalConfig, InjectConfig, OutputFile, PackageConfig, StrategyConfig,
        TemplateConfig,
    },
    db_query::{DbQuery, MpConfig},
    model::TableInfo,
};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct ConfigBuilder {
    pub datasource_config: DataSourceConfig,
    pub template_config: TemplateConfig,
    pub strategy_config: StrategyConfig,
    pub package_config: PackageConfig,
    pub global_config: GlobalConfig,
    pub injection_config: Option<InjectConfig>,
    pub table_infos: Vec<TableInfo>,
    pub path_info: HashMap<OutputFile, PathBuf>,
    pub db_query: Box<dyn DbQuery>,
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

    pub async fn query_tables(&mut self) -> Result<&Vec<TableInfo>> {
        let mut table_infos = self.db_query.table_infos(self).await?;
        for table_info in &mut table_infos {
            let table_field = self.db_query.table_field(table_info, self).await?;
            table_info.fields = Some(table_field);
        }

        self.table_infos = table_infos;

        Ok(&self.table_infos)
    }
}

struct PathInfoHandler<'a> {
    out_dir: PathBuf,
    package: &'a PackageConfig,
    path_info: HashMap<OutputFile, PathBuf>,
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

        if !path_info.is_empty() {
            path_info_handler.path_info.extend(path_info);
        }

        path_info_handler
    }

    pub fn get_path_info(&self) -> &HashMap<OutputFile, PathBuf> {
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
            OutputFile::Entity,
            "Entity",
        );
        self.put_path_info(
            template.get_controller(resource_path.join("controller.java")),
            OutputFile::Controller,
            "Controller",
        );
        self.put_path_info(
            template.get_mapper(resource_path.join("mapper.java")),
            OutputFile::Mapper,
            "Mapper",
        );
        self.put_path_info(
            template.get_controller(resource_path.join("mapper.xml")),
            OutputFile::Xml,
            "Xml",
        );
        self.put_path_info(
            template.get_service(resource_path.join("service.java")),
            OutputFile::Service,
            "Service",
        );
        self.put_path_info(
            template.get_service_impl(resource_path.join("service.impl.java")),
            OutputFile::ServiceImpl,
            "ServiceImpl",
        );
        self.add_path_info(OutputFile::Parent, "Parent");
    }

    fn put_path_info(&mut self, template: Option<PathBuf>, output_file: OutputFile, module: &str) {
        let Some(template) = template else {
            return
        };
        if template.to_string_lossy().is_empty() {
            return;
        }

        self.add_path_info(output_file, module);
    }

    pub fn add_path_info(&mut self, output_file: OutputFile, module: &str) {
        let module = self.package.get_package_info(module).unwrap_or("".into());
        let entry = self.path_info.entry(output_file);
        let out = PathInfoHandler::join_path(self.out_dir.clone(), &module);
        entry.or_insert(out);
    }

    fn join_path(out_dir: PathBuf, module: &str) -> PathBuf {
        out_dir.join(module.replace('.', "/"))
    }
}
