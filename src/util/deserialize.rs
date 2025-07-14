use serde::{self, Deserialize, Deserializer};
use serde_json::Value;

/// num 转 bool：将整数 0 转为 false，非零转为 true
pub fn deserialize_num_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Number(ref num) if num.is_i64() => {
            let i_val = num.as_i64().unwrap_or(0);
            Ok(i_val != 0)
        }
        _ => Err(serde::de::Error::custom("Expected an integer")),
    }
}
