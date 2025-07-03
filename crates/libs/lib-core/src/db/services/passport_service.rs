use crate::db::models::{NewPassport, NewPhoneMapping, OidcProviderEnum, Passport, Principal};
use crate::db::repositories::{
    PassportRepository, PgPassportRepository, PgPhoneMappingRepository, PhoneMappingRepository,
};
use chrono::Utc;
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
        principal: &Principal<'_>,
    ) -> Result<Option<Passport>, sqlx::Error> {
        let user_id = match principal {
            Principal::Phone(phone) => self.by_phone(phone).await?,
            Principal::Email(email) => self.by_email(email).await?,
            Principal::OpenId { provider, open_id } => self.by_open_id(provider, open_id).await?,
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
        principal: &Principal<'_>,
        user_id: i64,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        let mut passport = NewPassport {
            user_id,
            phone: "".to_string(),
            email: "".to_string(),
            created_at: now,
            updated_at: now,
        };
        match principal {
            Principal::Phone(phone) => passport.phone = phone.to_string(),
            Principal::Email(email) => passport.email = email.to_string(),
            _ => {}
        }
        self.passport_repo.insert(passport).await?;

        match principal {
            Principal::Phone(phone) => self.insert_phone(phone, user_id).await?,
            Principal::Email(email) => self.insert_email(email, user_id).await?,
            Principal::OpenId { provider, open_id } => {
                self.insert_open_id(provider, open_id, user_id).await?
            }
        };

        Ok(())
    }

    async fn by_phone(&self, e164_phone: &str) -> Result<Option<i64>, sqlx::Error> {
        let phone_mapping = self.phone_mapping_repo.by_phone(e164_phone).await?;
        let user_id = phone_mapping.map(|m| m.user_id);
        Ok(user_id)
    }

    async fn insert_phone(&self, phone: &str, user_id: i64) -> Result<(), sqlx::Error> {
        let mapping = NewPhoneMapping {
            phone: phone.to_string(),
            user_id,
        };
        self.phone_mapping_repo.insert(mapping).await?;
        Ok(())
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
