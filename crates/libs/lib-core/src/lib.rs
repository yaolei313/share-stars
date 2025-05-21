use anyhow::Context;
use clap::{Parser, ValueEnum};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::str::FromStr;
use std::sync::Arc;
use axum::extract::FromRef;

mod db;
pub mod dtos;







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
