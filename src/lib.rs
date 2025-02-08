#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
pub mod tracing;

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub mod postgres;

#[derive(Clone, bon::Builder)]
pub struct Services {
    #[cfg(feature = "postgres")]
    #[builder(setters(vis = "", name = pg_internal))]
    pub postgres: sqlx::PgPool,
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("service was not initialised")]
    NotInitialised,
    #[error("unknown data store error")]
    Unknown,
    #[error("invalid config `{0}`")]
    Configuration(String),
    #[cfg(feature = "postgres")]
    #[error(transparent)]
    /// Postgres error
    Postgres(#[from] sqlx::Error),
}
