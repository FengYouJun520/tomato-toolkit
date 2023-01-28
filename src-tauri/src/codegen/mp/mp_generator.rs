use derive_builder::Builder;

use crate::{
    codegen::config::{
        builder::ConfigBuilder, DataSourceConfig, GlobalConfig, InjectConfig, PackageConfig,
        StrategyConfig, TemplateConfig,
    },
    error::Result,
};

#[derive(Builder)]
#[builder(setter(strip_option))]
pub struct MpGenerator {
    config: ConfigBuilder,
    injection: InjectConfig,
    datasource: DataSourceConfig,
    strategy: StrategyConfig,
    package_info: PackageConfig,
    tempate: TemplateConfig,
    global: GlobalConfig,
}

impl MpGenerator {
    pub async fn execute(&self) -> Result<()> {
        Ok(())
    }

    pub async fn batch_output(&self) -> Result<()> {
        Ok(())
    }
}
