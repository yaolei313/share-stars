mod passport_repository;
mod passport_repository_impl;
mod phone_mapping_repository;
mod phone_mapping_repository_impl;

pub use passport_repository::*;

pub use phone_mapping_repository::*;

pub use passport_repository_impl::*;

pub use phone_mapping_repository_impl::*;

pub fn sharding_for_phone() -> u64 {
    todo!()
}
