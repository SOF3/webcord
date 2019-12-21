use serde::{Deserialize, Serialize};
use webcord_schema::models::SnowflakeData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Snowflake(pub u64);

impl From<Snowflake> for SnowflakeData {
    fn from(sf: Snowflake) -> Self {
        sf.0 as Self // direct cast
    }
}

impl From<SnowflakeData> for Snowflake {
    fn from(sf: SnowflakeData) -> Self {
        Self(sf as u64)
    }
}
