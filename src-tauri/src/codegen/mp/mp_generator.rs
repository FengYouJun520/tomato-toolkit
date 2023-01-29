use crate::error::Result;

use super::db_query::{DbQuery, MpConfig};

pub struct MpGenerator {
    config: MpConfig,
    query: Box<dyn DbQuery>,
}

impl MpGenerator {
    pub fn new(config: MpConfig) -> Self {
        println!("config: {:#?}", config);
        let query = config.datasource.db_query();
        Self { config, query }
    }

    pub fn init(&self) -> Result<()> {
        Ok(())
    }

    pub async fn execute(&self) -> Result<()> {
        // 进行一些必要的初始化
        self.init()?;

        // 执行输出
        self.batch_output().await
    }

    pub async fn batch_output(&self) -> Result<()> {
        let mut table_infos = self.query.table_infos(&self.config).await?;
        for table_info in &mut table_infos {
            let table_field = self.query.table_field(table_info, &self.config).await?;
            table_info.fields = Some(table_field);
        }

        println!("table_infos: {:#?}", table_infos);
        Ok(())
    }
}
