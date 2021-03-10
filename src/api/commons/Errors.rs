// https://github.com/mongodb/mongo-rust-driver/blob/master/src/error.rs
use actix_web::HttpResponse;
#[non_exhaustive]
#[derive(Debug)]
pub enum ApplicationError {
    ServerError,
//    DieselError,
    EnvironmentError,
    PostgreSQLError,
    MongoDBError,
    MongoDBValueAccessError,
    BsonOidError,
    BsonSerializationError,
    BsonDeserializationError,
    EntityNotFound,
    UserError(String),
    UuidError
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "Application error")
    }
}

impl From<std::env::VarError> for ApplicationError {
    fn from(_: std::env::VarError) -> ApplicationError {
        ApplicationError::EnvironmentError
    }
}

impl From<tokio_postgres::error::Error> for ApplicationError {
    fn from(_: tokio_postgres::error::Error) -> ApplicationError {
        ApplicationError::PostgreSQLError
    }
}

impl From<mongodb::error::Error> for ApplicationError {
    fn from(_: mongodb::error::Error) -> ApplicationError {
        ApplicationError::MongoDBError
    }
}

impl From<bson::document::ValueAccessError> for ApplicationError {
    fn from(_: bson::document::ValueAccessError) -> ApplicationError {
        ApplicationError::MongoDBValueAccessError
    }
}

impl From<bson::oid::Error> for ApplicationError {
    fn from(_: bson::oid::Error) -> ApplicationError {
        ApplicationError::BsonOidError
    }
}

impl From<bson::ser::Error> for ApplicationError {
    fn from(_: bson::ser::Error) -> ApplicationError {
        ApplicationError::BsonSerializationError
    }
}

impl From<bson::de::Error> for ApplicationError {
    fn from(_: bson::de::Error) -> ApplicationError {
        ApplicationError::BsonDeserializationError
    }
}

impl From<uuid::Error> for ApplicationError {
    fn from(_: uuid::Error) -> ApplicationError {
        ApplicationError::UuidError
    }
}

/*
impl From<diesel::result::Error> for ApplicationError {
    fn from(err: diesel::result::Error) -> ApplicationError {
        match err {
            diesel::result::Error::NotFound => ApplicationError::UserError("Username not found.".to_string()),
            _ => ApplicationError::DieselError
        }
    }
}
*/

impl actix_web::error::ResponseError for ApplicationError {
    
    fn error_response(&self) -> HttpResponse {
        /*
        match *self {
            ApplicationError::ServerError => HttpResponse::InternalServerError().json("Service Error."),
            ApplicationError::PostgreSQLError => HttpResponse::InternalServerError().json("PostgreSQL Pool Error."),
            ApplicationError::DieselError => HttpResponse::InternalServerError().json("Diesel Error."),
            ApplicationError::EnvironmentError => HttpResponse::InternalServerError().json("Environment Error."),
            ApplicationError::UserError(data) => HttpResponse::NotFound().json(data)
        }
        */

        let mut msg = "";

        match self {
            ApplicationError::ServerError => msg = "Service Error.",
            ApplicationError::PostgreSQLError => msg = "PostgreSQL Pool Error.",
            //ApplicationError::DieselError => msg = "Diesel Error.",
            ApplicationError::MongoDBError => msg = "MongoDB Error.",
            ApplicationError::MongoDBValueAccessError => {},
            ApplicationError::BsonOidError => msg = "ObjectID serialization Error",
            ApplicationError::BsonSerializationError => msg = "Document serialization Error",
            ApplicationError::BsonDeserializationError => msg = "Document deserialization Error",
            ApplicationError::EnvironmentError => msg = "Environment Error.",
            ApplicationError::EntityNotFound => msg = "Entity Not Found", // To be implemented...: HttpResponse::NotFound().json(data),
            ApplicationError::UserError(data) => msg = data.as_str(),
            ApplicationError::UuidError => msg = "failed to parse Uuid data type"
        }

        HttpResponse::InternalServerError().json(msg)
    }
}