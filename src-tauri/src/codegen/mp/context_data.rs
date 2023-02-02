use std::collections::HashMap;

use derive_builder::Builder;
use serde::Serialize;

use crate::error::Result;

use super::{config::IdType, config_builder::ConfigBuilder, model::TableInfo};

/// 模板数据渲染
pub trait TemplateRender {
    type Item: Serialize + Clone;
    fn render_data(&self, table_info: &TableInfo) -> Result<Self::Item>;
}

/// 模板上下文需要的数据
#[derive(Serialize, Builder)]
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
    rest_controller_style: bool,
    super_controller_class_package: Option<String>,
    super_controller_class: Option<String>,
}

#[derive(Clone, Serialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
pub struct MapperData {
    enable_cache: bool,
    cache_class_name: String,
    mapper_annotation_class: String,
    mapper_annotation_name: String,
    base_result_map: bool,
    base_column_list: bool,
    super_mapper_class_package: String,
    super_mapper_class: Option<String>,
}

#[derive(Clone, Serialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceData {
    super_service_class_package: String,
    super_service_class: Option<String>,
    super_service_impl_class_package: String,
    super_service_impl_class: Option<String>,
}

#[derive(Clone, Serialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
pub struct EntityData {
    id_type: Option<IdType>,
    logic_delete_field_name: String,
    version_field_name: String,
    active_record: bool,
    entity_serial_version_uid: bool,
    entity_column_constant: bool,
    entity_builder_model: bool,
    chain_model: bool,
    entity_lombok_model: bool,
    entity_boolean_column_remove_is_prefix: bool,
    super_entity_class: Option<String>,
}
