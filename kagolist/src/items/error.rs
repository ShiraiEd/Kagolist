use rapina::database::DbError;
use rapina::prelude::*;

pub enum ItemError {
    DbError(DbError),
}

impl IntoApiError for ItemError {
    fn into_api_error(self) -> Error {
        match self {
            ItemError::DbError(e) => e.into_api_error(),
        }
    }
}

impl DocumentedError for ItemError {
    fn error_variants() -> Vec<ErrorVariant> {
        vec![
            ErrorVariant {
                status: 404,
                code: "NOT_FOUND",
                description: "Item not found",
            },
            ErrorVariant {
                status: 500,
                code: "DATABASE_ERROR",
                description: "Database operation failed",
            },
        ]
    }
}

impl From<DbError> for ItemError {
    fn from(e: DbError) -> Self {
        ItemError::DbError(e)
    }
}
