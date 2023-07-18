use sea_orm::sea_query::{ArrayType, ColumnType, Nullable, ValueType, ValueTypeErr};
use sea_orm::{DbErr, TryFromU64, TryGetableFromJson, Value};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AsJson<T>(pub T);

impl<T: Serialize + DeserializeOwned> AsJson<T> {
    pub fn model(self) -> T {
        self.0
    }
}

impl<T: Serialize + DeserializeOwned> From<AsJson<T>> for Value {
    fn from(value: AsJson<T>) -> Self {
        Value::Json(Some(Box::new(
            serde_json::to_value(value).expect("failed to convert AsJson<T> to serde_json::Value"),
        )))
    }
}

impl<T: Serialize + DeserializeOwned> TryFromU64 for AsJson<T> {
    fn try_from_u64(_: u64) -> Result<Self, DbErr> {
        Err(DbErr::ConvertFromU64("AsJson<T>"))
    }
}

impl<T: Serialize + DeserializeOwned> Nullable for AsJson<T> {
    fn null() -> Value {
        Value::Json(None)
    }
}

impl<T: Serialize + DeserializeOwned> TryGetableFromJson for AsJson<T> {}

impl<T: Serialize + DeserializeOwned> ValueType for AsJson<T> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Json(Some(j)) => match serde_json::from_value(*j) {
                Ok(d) => Ok(d),
                Err(e) => {
                    tracing::warn!("cannot parse json as {}: {e}", Self::type_name());
                    Err(ValueTypeErr)
                }
            },

            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        format!("AsJson<{}>", std::any::type_name::<T>())
    }

    fn array_type() -> ArrayType {
        ArrayType::Json
    }

    fn column_type() -> ColumnType {
        ColumnType::Json
    }
}
