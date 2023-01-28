use std::collections::HashMap;

use crate::codegen::protocol::{
    db_query::{DbQuery, DefaultDbQuery},
    model::TableInfo,
};

use super::{
    DataSourceConfig, GlobalConfig, InjectConfig, OutputFile, PackageConfig, StrategyConfig,
    TemplateConfig,
};

#[derive(Clone)]
pub struct ConfigBuilder {
    template_config: TemplateConfig,
    table_infos: Vec<TableInfo>,
    pathinfo: Option<HashMap<OutputFile, String>>,
    strategy_config: StrategyConfig,
    global_config: GlobalConfig,
    inject_config: InjectConfig,
    package_config: PackageConfig,
    datasource_config: DataSourceConfig,
    db_query: DefaultDbQuery,
}

impl ConfigBuilder {
    pub fn new(
        template_config: TemplateConfig,
        strategy_config: StrategyConfig,
        global_config: GlobalConfig,
        inject_config: InjectConfig,
        package_config: PackageConfig,
        datasource_config: DataSourceConfig,
    ) -> Self {
        Self {
            template_config,
            table_infos: vec![],
            pathinfo: None,
            strategy_config: strategy_config.clone(),
            global_config,
            inject_config,
            package_config,
            datasource_config: datasource_config.clone(),
            db_query: DefaultDbQuery::new(datasource_config, strategy_config),
        }
    }
}
