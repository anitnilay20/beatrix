use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    /// Error from `mongodb::error::Error`.
    #[error("{0}")]
    Mongo(#[from]mongodb::error::Error),

    /// Error from `mongodb::bson::oid::Error`
    #[error("{0}")]
    BsonOid(#[from] mongodb::bson::oid::Error),

    /// Error from `mongodb::bson::de::Error`.
    #[error("{0}")]
    BsonDe(#[from] mongodb::bson::de::Error),

    /// Error from `mongodb::bson::ser::Error`.
    #[error("{0}")]
    BsonSer(#[from] mongodb::bson::ser::Error),

    /// An error indicating that an ObjectId is required for the requested operation.
    #[error("Model must have an ObjectId for this operation.")]
    ModelIdRequiredForOperation,

    /// An error indicating that a model was serialized to a BSON variant other than a document.
    #[error("Serializing model to BSON failed to produce a Bson::Document, got type {0:?}")]
    ModelSerToDocument(mongodb::bson::spec::ElementType),
}
