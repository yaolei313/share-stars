use crate::db::models::{LoginPrincipal, OidcProviderEnum, Passport};
use crate::db::repositories::{
    PassportRepository, PgPassportRepository, PgPhoneMappingRepository, PhoneMappingRepository,
};
use std::sync::Arc;

pub struct PassportService {
    pub passport_repo: Arc<PgPassportRepository>,
    pub phone_mapping_repo: Arc<PgPhoneMappingRepository>,
}

impl PassportService {
    pub fn new(
        passport_repo: Arc<PgPassportRepository>,
        phone_mapping_repo: Arc<PgPhoneMappingRepository>,
    ) -> Self {
        Self {
            passport_repo,
            phone_mapping_repo,
        }
    }

    pub async fn query_passport(
        &self,
        principal: &LoginPrincipal<'_>,
    ) -> Result<Option<Passport>, sqlx::Error> {
        let user_id = match principal {
            LoginPrincipal::Phone(phone) => self.by_phone(phone).await?,
            LoginPrincipal::Email(email) => self.by_email(email).await?,
            LoginPrincipal::OpenId { provider, open_id } => {
                self.by_open_id(provider, open_id).await?
            }
        };

        let passport = if let Some(user_id) = user_id {
            self.passport_repo.by_user_id(user_id).await?
        } else {
            None
        };

        Ok(passport)
    }

    pub async fn create_passport(
        &self,
        passport: &LoginPrincipal<'_>,
        user_id: i64,
    ) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn by_phone(&self, e164_phone: &str) -> Result<Option<i64>, sqlx::Error> {
        let phone_mapping = self.phone_mapping_repo.by_phone(e164_phone).await?;
        let user_id = phone_mapping.map(|m| m.user_id);
        Ok(user_id)
    }

    async fn by_email(&self, email: &str) -> Result<Option<i64>, sqlx::Error> {
        todo!()
    }

    async fn by_open_id(
        &self,
        provider: &OidcProviderEnum,
        open_id: &str,
    ) -> Result<Option<i64>, sqlx::Error> {
        todo!()
    }
}
