use async_trait::async_trait;

use mongodb::bson::{doc, from_bson, oid::ObjectId, to_bson, Bson, Document};

use mongodb::results::DeleteResult;
use mongodb::{options, Collection, Database};
use serde::{de::DeserializeOwned, Serialize};

pub mod cursor;
pub mod error;

use error::Result;

#[async_trait]
pub trait MongoModel
where
    Self: Serialize + DeserializeOwned,
{
    const COLLECTION_NAME: &'static str;

    /// converts bson to current model
    fn from_bson(document: Document) -> Result<Self> {
        Ok(from_bson(Bson::Document(document))?)
    }

    /// The model's read concern.
    fn read_concern() -> Option<options::ReadConcern> {
        None
    }

    /// The model's write concern.
    fn write_concern() -> Option<options::WriteConcern> {
        None
    }

    /// The model's selection criteria.
    ///
    /// When deriving a model, a function or an associated function should be specified
    /// which
    /// should be used to produce the desired value.
    fn selection_criteria() -> Option<options::SelectionCriteria> {
        None
    }

    /// Get collection for this model.
    fn collection(db: Database) -> Collection {
        db.collection_with_options(
            Self::COLLECTION_NAME,
            options::CollectionOptions::builder()
                .selection_criteria(Self::selection_criteria())
                .read_concern(Self::read_concern())
                .write_concern(Self::write_concern())
                .build(),
        )
    }

    /// Get number of documents matching filter for current Collection
    async fn count<F, O>(db: Database, filter: F, options: O) -> Result<i64>
    where
        F: Into<Option<Document>> + Send,
        O: Into<Option<options::CountOptions>> + Send,
    {
        Ok(Self::collection(db)
            .count_documents(filter, options)
            .await?)
    }

    /// Finds distinct values for given field
    async fn distinct<F, O>(
        db: Database,
        field_name: &str,
        filter: F,
        options: O,
    ) -> Result<Vec<Bson>>
    where
        F: Into<Option<Document>> + Send,
        O: Into<Option<options::DistinctOptions>> + Send,
    {
        Ok(Self::collection(db)
            .distinct(field_name, filter, options)
            .await?)
    }

    /// Finds the documents in the collection matching filter.
    async fn find<F, O>(db: Database, filter: F, options: O) -> Result<Option<Self>>
    where
        F: Into<Option<Document>> + Send,
        O: Into<Option<options::FindOneOptions>> + Send,
    {
        Ok(Self::collection(db)
            .find_one(filter, options)
            .await?
            .map(Self::from_bson)
            .transpose()?)
    }

    async fn filter<F, O>(db: Database, filter: F, options: O) -> Result<cursor::ModelCursor<Self>>
    where
        F: Into<Option<Document>> + Send,
        O: Into<Option<options::FindOptions>> + Send,
    {
        Ok(Self::collection(db)
            .find(filter, options)
            .await
            .map(cursor::ModelCursor::new)?)
    }

    /// Instance Methods
    ///
    /// Set Document id
    fn set_id(&mut self, id: ObjectId);

    /// Get document id
    fn id(&self) -> Option<ObjectId>;

    /// Save current object to collection.
    /// If collection has a id, existing collection will be updated.
    /// If id is `None` then a new id will be created
    /// To save the instance update method is called with upsert set to true.
    async fn save(&mut self, db: Database) -> Result<()> {
        let collection = Self::collection(db);

        let mut write_concern = Self::write_concern().unwrap_or_default();
        write_concern.journal = Some(true);

        if self.id().is_none() {
            self.set_id(ObjectId::new());
        }

        let doc = self.to_document()?;
        collection
            .update_one(
                doc! { "_id": self.id().unwrap() },
                doc,
                options::UpdateOptions::builder()
                    .upsert(Some(true))
                    .write_concern(write_concern)
                    .build(),
            )
            .await?;
        Ok(())
    }

    /// Delete this model instance by ID.
    /// Returns error if ID is null.
    async fn delete(&self, db: Database) -> Result<DeleteResult> {
        let id = self
            .id().ok_or(error::Error::ModelIdRequiredForOperation)?;

        Ok(Self::collection(db)
            .delete_one(doc! {"_id": id}, None)
            .await?)
    }

    /// Convert instance to document.
    fn to_document(&self) -> Result<Document> {
        match to_bson(self)? {
            Bson::Document(doc) => Ok(doc),
            bsn => Err(error::Error::ModelSerToDocument(bsn.element_type())),
        }
    }
}
