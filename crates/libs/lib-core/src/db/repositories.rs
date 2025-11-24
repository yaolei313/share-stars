mod account_device_repository;
mod account_device_repository_impl;
mod account_identity_repository;
mod account_identity_repository_impl;
mod account_repository;
mod account_repository_impl;
mod device_repository;
mod device_repository_impl;
mod email_template_repository;
mod email_template_repository_impl;
mod lookup_account_repository;
mod lookup_account_repository_impl;
mod lookup_device_repository;
mod lookup_device_repository_impl;
mod sms_template_repository;
mod sms_template_repository_impl;

pub use account_repository::*;

pub use account_repository_impl::*;

pub use lookup_account_repository::*;

pub use lookup_account_repository_impl::*;

pub use account_identity_repository::*;

pub use account_identity_repository_impl::*;

pub use device_repository::*;

pub use device_repository_impl::*;

pub use account_device_repository::*;
pub use account_device_repository_impl::*;

pub use sms_template_repository::*;
pub use sms_template_repository_impl::*;

pub use email_template_repository::*;
pub use email_template_repository_impl::*;

pub fn sharding_for_phone(e164_phone: &str) -> u64 {
    todo!()
}
