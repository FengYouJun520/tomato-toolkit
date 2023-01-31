use std::collections::HashMap;

use derive_builder::Builder;
use serde::Serialize;

use crate::error::Result;

use super::{config_builder::ConfigBuilder, model::TableInfo};

/// 模板数据渲染
pub trait TemplateRender {
    type Item: Serialize + Clone;
    fn render_data(&self, table_info: &TableInfo) -> Result<Self::Item>;
}

/// 模板上下文需要的数据
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextData {
    #[serde(flatten)]
    pub controller_data: ControllerData,
    #[serde(flatten)]
    pub mapper_data: MapperData,
    #[serde(flatten)]
    pub service_data: ServiceData,
    #[serde(flatten)]
    pub entity_data: EntityData,
    pub config: ConfigBuilder,
    pub package: HashMap<String, String>,
    pub author: String,
    pub kotlin: bool,
    pub swagger: bool,
    pub springdoc: bool,
    pub date: String,
    pub schema_name: String,
    pub table: TableInfo,
    pub entity: String,
}

#[derive(Clone, Serialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
pub struct ControllerData {
    controller_mapping_hyphen: String,
    controller_mapping_hyphen_style: bool,
    reset_controller_style: bool,
    super_controller_class_package: Option<String>,
    super_controller_class: Option<String>,
}

#[derive(Clone, Serialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
pub struct MapperData {
    mapper_annotation: Option<bool>,
    mapper_annotation_class: String,
    base_result_map: Option<bool>,
    base_column_list: Option<bool>,
    super_mapper_class_package: Option<String>,
    super_mapper_class: Option<String>,
}

#[derive(Clone, Serialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceData {
    mapper_annotation: Option<bool>,
    mapper_annotation_class: String,
    base_result_map: Option<bool>,
    base_column_list: Option<bool>,
    super_mapper_class_package: Option<String>,
    super_mapper_class: Option<String>,
}

#[derive(Clone, Serialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
pub struct EntityData {
    mapper_annotation: Option<bool>,
    mapper_annotation_class: String,
    base_result_map: Option<bool>,
    base_column_list: Option<bool>,
    super_mapper_class_package: Option<String>,
    super_mapper_class: Option<String>,
}
