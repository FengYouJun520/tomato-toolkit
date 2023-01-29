use crate::error::Result;

use super::db_query::MpConfig;

pub struct MpGenerator {
    config: MpConfig,
}

impl MpGenerator {
    pub fn new(config: MpConfig) -> Self {
        Self { config }
    }

    pub async fn execute(&self) -> Result<()> {
        Ok(())
    }

    pub async fn batch_output(&self) -> Result<()> {
        Ok(())
    }
}
