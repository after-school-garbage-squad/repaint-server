use email_address::EmailAddress;

use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Admin {
    pub admin_id: Id<Self>,
    pub email: EmailAddress,
}
