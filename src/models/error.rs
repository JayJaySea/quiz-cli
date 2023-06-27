use diesel::result::Error as DieselError;

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("This record was not found in the database")]
    NotFound,

    #[error("Unique constraint failed {0}")]
    Collision(String),

    #[error("Operation on reference failed {0}")]
    InvalidReferenceOperation(String),

    #[error(transparent)]
    Diesel(DieselError),

    #[error(transparent)]
    Generic(#[from] anyhow::Error)
}

pub fn db_map_err(e: DieselError) -> DbError {
    match e {
        DieselError::NotFound => DbError::NotFound,
        DieselError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, info)
            => DbError::Collision(info.constraint_name().unwrap_or("[UNKNOWN]").to_string()),
        DieselError::DatabaseError(diesel::result::DatabaseErrorKind::ForeignKeyViolation, info)
            => DbError::InvalidReferenceOperation(info.constraint_name().unwrap_or("[UNKNOWN]").to_string()),
        e => DbError::Diesel(e)
    }
}
