use std::collections::{BTreeSet, HashMap};

use convert_case::{Case, Casing};
use derive_builder::Builder;
use dyn_fmt::AsStrFormatExt;
use serde::Serialize;
use sqlx::FromRow;

use crate::error::Result;

use super::{
    config::{DataSourceConfig, Entity, FieldFill, GlobalConfig, NamingStrategy, StrategyConfig},
    config_builder::ConfigBuilder,
    convert::NameConvert,
    types::DbColumnType,
};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Table {
    /// 表名
    pub name: String,
    /// 注释
    #[sqlx(default)]
    pub comment: Option<String>,
    /// 表所属数据库名
    #[sqlx(default)]
    pub schema: String,
}

/// 表信息
#[derive(Debug, Clone, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableInfo {
    strategy_config: StrategyConfig,
    global_config: GlobalConfig,
    pub import_packages: BTreeSet<String>,
    pub convert: bool,
    /// 表名
    pub name: String,
    /// 注释
    pub comment: Option<String>,
    pub entity_name: String,
    pub entity_path: String,
    pub mapper_name: String,
    pub xml_name: String,
    pub service_name: String,
    pub service_impl_name: String,
    pub controller_name: String,
    pub have_primary_key: bool,
    pub field_names: String,
    pub entity: Entity,
    pub common_fields: Vec<TableField>,
    /// 表所有列信息
    pub fields: Vec<TableField>,
}

impl TableInfo {
    pub fn new(
        config: &ConfigBuilder,
        comment: Option<String>,
        table_name: impl Into<String>,
    ) -> Self {
        // TODO: 设置entity名称
        Self {
            strategy_config: config.strategy_config.clone(),
            global_config: config.global_config.clone(),
            name: table_name.into(),
            entity: config.strategy_config.entity.clone(),
            import_packages: BTreeSet::new(),
            comment,
            entity_name: "".to_string(),
            entity_path: "".to_string(),
            common_fields: vec![],
            fields: vec![],
            convert: false,
            mapper_name: "".into(),
            xml_name: "".into(),
            service_name: "".into(),
            service_impl_name: "".into(),
            controller_name: "".into(),
            have_primary_key: false,
            field_names: "".into(),
        }
    }

    /// 处理文件名与导包
    pub fn process_table(&mut self) -> Result<()> {
        let entity_name = self
            .entity
            .name_convert(self.strategy_config.clone())
            .entity_name_convert(self)?;

        self.set_entity_name(self.entity.format_filename.format(&[&entity_name]));
        self.entity_path = self.get_entity_path();
        self.mapper_name = self
            .strategy_config
            .mapper
            .format_mapper_filename
            .format(&[&entity_name]);
        self.xml_name = self
            .strategy_config
            .mapper
            .format_xml_filename
            .format(&[&entity_name]);
        self.service_name = self
            .strategy_config
            .service
            .format_service_filename
            .format(&[&entity_name]);
        self.service_impl_name = self
            .strategy_config
            .service
            .format_service_impl_filename
            .format(&[&entity_name]);
        self.controller_name = self
            .strategy_config
            .controller
            .format_filename
            .format(&[&entity_name]);

        self.import_packages();

        Ok(())
    }

    pub fn set_entity_name(&mut self, entity_name: impl Into<String>) {
        self.entity_name = entity_name.into();
        self.set_convert();
    }

    fn set_convert(&mut self) {
        if self.strategy_config.start_with_table_prefix(&self.name)
            || self.entity.enable_table_field_annotation
        {
            self.convert = true;
        } else {
            self.convert = !self.entity_name.eq_ignore_ascii_case(&self.name);
        }
    }

    pub fn add_field(&mut self, field: TableField) {
        // 忽略字段
        if self.entity.match_ingore_columns(&field.column_name) {
            return;
        }

        // 如果是公共字段
        if self.entity.match_super_entity_columns(&field.column_name) {
            self.common_fields.push(field);
        } else {
            self.fields.push(field);
        }
    }

    /// 导包处理
    pub fn import_packages(&mut self) {
        let super_entity = &self.entity.super_class;
        if !super_entity.is_empty() {
            self.import_packages.insert(super_entity.into());
        } else if self.entity.active_record {
            self.import_packages
                .insert("com.baomidou.mybatisplus.extension.activerecord.Model".into());
        }
        if self.entity.serial_version_uid | self.entity.active_record {
            self.import_packages.insert("java.io.Serializable".into());
        }
        if self.convert {
            self.import_packages
                .insert("com.baomidou.mybatisplus.annotation.TableName".into());
        }

        if self.entity.id_type.is_some() && self.have_primary_key {
            self.import_packages
                .insert("com.baomidou.mybatisplus.annotation.IdType".into());
            self.import_packages
                .insert("com.baomidou.mybatisplus.annotation.TableId".into());
        }

        for field in self.fields.iter_mut() {
            let column_type = field.column_type;
            if let Some(pkg) = column_type.get_pkg() {
                self.import_packages.insert(pkg);
            }

            if field.key_flag {
                // 主键
                if field.convert || field.key_identity_flag {
                    self.import_packages
                        .insert("com.baomidou.mybatisplus.annotation.TableId".into());
                }
                // 自增
                if field.key_identity_flag {
                    self.import_packages
                        .insert("com.baomidou.mybatisplus.annotation.IdType".into());
                }
            } else if field.convert {
                self.import_packages
                    .insert("com.baomidou.mybatisplus.annotation.TableField".into());
            }

            if field.fill.is_some() {
                self.import_packages
                    .insert("com.baomidou.mybatisplus.annotation.TableField".into());
                self.import_packages
                    .insert("com.baomidou.mybatisplus.annotation.FieldFill".into());
            }
            if field.version_field {
                self.import_packages
                    .insert("com.baomidou.mybatisplus.annotation.Version".into());
            }
            if field.logic_delete_field {
                self.import_packages
                    .insert("com.baomidou.mybatisplus.annotation.TableLogic".into());
            }
        }
    }

    pub fn get_entity_path(&self) -> String {
        self.entity_name.to_case(Case::Camel)
    }
}

/// 列信息
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Field {
    /// 列名
    pub name: String,
    ///注释
    #[sqlx(default)]
    pub comment: Option<String>,
    /// 列类型
    pub r#type: String,
    /// 是否可为空
    pub is_nullable: bool,
    /// 列长度
    pub length: Option<i64>,
    ///是否是主键
    pub key_flag: bool,
    #[sqlx(default)]
    pub default_value: Option<String>,
    #[sqlx(default)]
    pub auto_increment: bool,
}

/// 列信息
#[derive(Debug, Clone, FromRow, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
pub struct TableField {
    pub convert: bool,
    pub key_flag: bool,
    pub key_identity_flag: bool,
    pub name: String,
    pub r#type: String,
    pub property_name: String,
    pub capital_name: String,
    pub column_type: DbColumnType,
    pub comment: String,
    pub column_name: String,
    pub annotation_column_name: String,
    pub custom_map: Option<HashMap<String, serde_json::Value>>,
    pub entity: Entity,
    pub datasource_config: DataSourceConfig,
    pub global_config: GlobalConfig,
    pub fill: Option<FieldFill>,
    pub version_field: bool,
    pub logic_delete_field: bool,
    have_primary: bool,
}

impl TableField {
    /// 设置属性名称
    pub fn set_property_name(&mut self, property_name: &str, column_type: DbColumnType) {
        self.column_type = column_type;

        if self.entity.boolean_column_remove_is_prefix
            && (column_type.get_type() == "boolean" || column_type.get_type() == "Boolean")
            && property_name.starts_with("is")
        {
            self.convert = true;
            self.property_name = (&property_name[2..]).to_case(Case::Camel);
            self.set_capital_name();
            return;
        }

        match self.entity.column_naming() {
            //下划线转驼峰策略
            NamingStrategy::UnderlineToCamel => {
                self.convert = !property_name
                    .eq_ignore_ascii_case(&NamingStrategy::underline_to_camel(&self.column_name))
            }
            // 原样输出策略
            NamingStrategy::NoChange => {
                self.convert = !property_name.eq_ignore_ascii_case(&self.column_name)
            }
        };
        if self.entity.enable_table_field_annotation {
            self.convert = true;
        }
        self.property_name = property_name.to_string();
        self.set_capital_name();
    }

    fn set_capital_name(&mut self) {
        self.capital_name = if self.property_name.len() == 1 {
            self.property_name.to_uppercase()
        } else if (&self.property_name[1..2]).is_case(Case::Lower) {
            self.property_name[0..1].to_uppercase() + &self.property_name[1..]
        } else {
            self.property_name.clone()
        };
    }

    pub fn set_fill(&mut self) {
        let fill = self
            .entity
            .table_fill_list
            .iter()
            .find(|tf| tf.property_name.eq_ignore_ascii_case(&self.name))
            .map(|tf| tf.field_fill);
        self.fill = fill;
    }

    pub fn set_version_field(&mut self) {
        let property_name = &self.entity.version_property_name;
        let column_name = &self.entity.version_column_name;
        self.version_field = !property_name.is_empty()
            && property_name.eq_ignore_ascii_case(&self.property_name)
            || !column_name.is_empty() && column_name.eq_ignore_ascii_case(&self.name)
    }

    pub fn set_logic_delete_field(&mut self) {
        let property_name = &self.entity.logic_delete_property_name;
        let column_name = &self.entity.logic_delete_column_name;
        self.logic_delete_field = !property_name.is_empty()
            && property_name.eq_ignore_ascii_case(&self.property_name)
            || !column_name.is_empty() && column_name.eq_ignore_ascii_case(&self.name)
    }
}
