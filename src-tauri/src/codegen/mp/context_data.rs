use derive_builder::Builder;
use serde::Serialize;

use crate::error::Result;

use super::model::TableInfo;

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
    controller: ControllerData,
    #[serde(flatten)]
    mapper: MapperData,
}

#[derive(Clone, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ControllerData {
    controller_mapping_hyphen: String,
    controller_mapping_hyphen_style: bool,
    reset_controller_style: bool,
    super_controller_class_package: Option<String>,
    super_controller_class: Option<String>,
}

#[derive(Clone, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct MapperData {
    mapper_annotation: Option<bool>,
    mapper_annotation_class: String,
    base_result_map: Option<bool>,
    base_column_list: Option<bool>,
    super_mapper_class_package: Option<String>,
    super_mapper_class: Option<String>,
}
