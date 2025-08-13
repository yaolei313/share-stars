mod account_device_repository;
mod account_device_repository_impl;
mod account_identity_repository;
mod account_identity_repository_impl;
mod account_repository;
mod account_repository_impl;
mod device_repository;
mod device_repository_impl;
mod lookup_account_repository;
mod lookup_account_repository_impl;
mod lookup_device_repository;
mod lookup_device_repository_impl;

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

pub fn sharding_for_phone() -> u64 {
    todo!()
}
