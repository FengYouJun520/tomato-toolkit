use std::fmt::{Debug, Display};

use serde::Serialize;

pub struct SerializeError(anyhow::Error);
pub type Result<T, E = SerializeError> = std::result::Result<T, E>;

impl Serialize for SerializeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_ref())
    }
}

impl<M> From<M> for SerializeError
where
    M: Display + Debug + Send + Sync + 'static,
{
    fn from(value: M) -> Self {
        Self(anyhow::Error::msg(value))
    }
}
