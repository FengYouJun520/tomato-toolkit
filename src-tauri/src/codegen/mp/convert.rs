use std::{collections::HashSet, fmt::Debug};

use crate::error::Result;

use super::{
    config::{NamingStrategy, StrategyConfig},
    model::{TableField, TableInfo},
};

/// 名称转换器
pub trait NameConvert: Debug {
    fn entity_name_convert(&self, table_info: TableInfo) -> Result<String>;
    fn property_name_convert(&self, field: TableField) -> Result<String>;
}

/// 默认名称转换器
#[derive(Debug)]
pub struct DefaultNameConvert {
    strategy_config: StrategyConfig,
}

impl DefaultNameConvert {
    pub fn new(strategy_config: StrategyConfig) -> Self {
        Self { strategy_config }
    }

    fn process_name(
        &self,
        name: &str,
        strategy: NamingStrategy,
        prefix: &HashSet<String>,
        suffix: &HashSet<String>,
    ) -> Result<String> {
        let mut property_name = name.to_string();

        // 删除前缀
        if !prefix.is_empty() {
            property_name = NamingStrategy::remove_prefix(&property_name, prefix);
        }

        // 删除后缀
        if !suffix.is_empty() {
            property_name = NamingStrategy::remove_suffix(&property_name, suffix);
        }

        if property_name.is_empty() {
            return Err(format!("{:?} 的名称转换结果为空，请检查是否配置问题", name).into());
        }

        if matches!(strategy, NamingStrategy::UnderlineToCamel) {
            return Ok(NamingStrategy::underline_to_camel(&property_name));
        }

        Ok(property_name)
    }
}

impl NameConvert for DefaultNameConvert {
    fn entity_name_convert(&self, table_info: TableInfo) -> Result<String> {
        let name = self.process_name(
            &table_info.name,
            self.strategy_config.entity.naming,
            &self.strategy_config.table_prefix,
            &self.strategy_config.table_suffix,
        )?;
        let res = NamingStrategy::capital(&name);

        Ok(res)
    }

    fn property_name_convert(&self, field: TableField) -> Result<String> {
        self.process_name(
            &field.name,
            self.strategy_config
                .entity
                .column_naming
                .unwrap_or(self.strategy_config.entity.naming),
            &self.strategy_config.field_prefix,
            &self.strategy_config.field_suffix,
        )
    }
}
