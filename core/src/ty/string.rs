use std::fmt::Display;
use std::str::FromStr;

use sea_orm::sea_query::value::{ArrayType, Value, ValueType, ValueTypeErr};
use sea_orm::sea_query::{ColumnType, Nullable};
use sea_orm::{DbErr, QueryResult, TryFromU64, TryGetError, TryGetable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AsString<T>(pub T);

pub trait ToDatabaseType {
    type DatabaseType;
    fn dty(self) -> Self::DatabaseType;
}

impl<T: FromStr + Display> ToDatabaseType for T {
    type DatabaseType = AsString<T>;

    fn dty(self) -> Self::DatabaseType {
        AsString(self)
    }
}

impl<T: FromStr + Display> AsString<T> {
    pub fn model(self) -> T {
        self.0
    }
}

impl<T: FromStr + Display> From<AsString<T>> for Value {
    fn from(value: AsString<T>) -> Self {
        Value::String(Some(Box::new(value.0.to_string())))
    }
}

impl<T: FromStr + Display> TryFromU64 for AsString<T> {
    fn try_from_u64(_: u64) -> Result<Self, DbErr> {
        Err(DbErr::ConvertFromU64("AsString<T>"))
    }
}

impl<T: FromStr + Display> TryGetable for AsString<T>
where
    <T as FromStr>::Err: std::error::Error,
{
    fn try_get_by<I: sea_orm::ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let str_value = <String as TryGetable>::try_get_by(res, index)?;
        match str_value.parse() {
            Ok(s) => Ok(AsString(s)),
            Err(e) => {
                let ty = Self::type_name();

                tracing::warn!("cannot parse \"{str_value}\" as {ty}: {e}");
                Err(sea_orm::TryGetError::DbErr(DbErr::Type(format!(
                    "failed to parse database value as {ty}: see tracing log for details",
                ))))
            }
        }
    }
}

impl<T: FromStr + Display> ValueType for AsString<T>
where
    <T as FromStr>::Err: std::error::Error,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(s)) => match s.parse() {
                Ok(d) => Ok(AsString(d)),
                Err(e) => {
                    tracing::warn!("cannot parse \"{s}\" as {}: {e}", Self::type_name());
                    Err(ValueTypeErr)
                }
            },

            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        format!("AsString<{}>", std::any::type_name::<T>())
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(None)
    }
}

impl<T: FromStr + Display> Nullable for AsString<T> {
    fn null() -> Value {
        Value::String(None)
    }
}
