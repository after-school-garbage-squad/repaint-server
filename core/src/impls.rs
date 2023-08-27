use repaint_server_usecase::infra::repo::IsUpdated;
use sea_orm::{DbErr, DeleteResult, UpdateResult};

use crate::Error;

pub mod admin;
pub mod event;

trait IsUpdatedExt {
    fn to_is_updated(self) -> Result<IsUpdated, Error>;
}

impl IsUpdatedExt for Result<UpdateResult, DbErr> {
    fn to_is_updated(self) -> Result<IsUpdated, Error> {
        match self {
            Ok(u) => Ok(IsUpdated(u.rows_affected > 0)),
            Err(DbErr::RecordNotFound(_)) => Ok(IsUpdated(false)),
            Err(e) => Err(Error::SeaOrm(e)),
        }
    }
}

impl IsUpdatedExt for Result<DeleteResult, DbErr> {
    fn to_is_updated(self) -> Result<IsUpdated, Error> {
        match self {
            Ok(u) => Ok(IsUpdated(u.rows_affected > 0)),
            Err(DbErr::RecordNotFound(_)) => Ok(IsUpdated(false)),
            Err(e) => Err(Error::SeaOrm(e)),
        }
    }
}

impl<T> IsUpdatedExt for Result<T, DbErr> {
    default fn to_is_updated(self) -> Result<IsUpdated, Error> {
        match self {
            Ok(_) => Ok(IsUpdated(true)),
            Err(DbErr::RecordNotFound(_)) => Ok(IsUpdated(false)),
            Err(e) => Err(Error::SeaOrm(e)),
        }
    }
}
