//! # Document
//!
//! In Typesense, a group of related documents is called a collection. A collection
//! is roughly equivalent to a table in a relational database.
//!

use crate::client::Client;
use crate::document_trait::Document as DocumentTrait;
use crate::transport::HttpLowLevel;
use crate::Result;

/// Client for the Typesense CollectionAPI
pub struct DocumentsClient<'a, T> {
    pub(crate) client: Client<T>,
    pub(crate) collection_name: &'a str,
}

/// Client for the Typesense DocumentsAPI
impl<'a, T> DocumentsClient<'a, T>
where
    T: HttpLowLevel,
{
    /// Create a document
    pub async fn create<D: DocumentTrait>(&self, schema: D) -> Result<D> {
        let path = format!("/collections/{}/documents", self.collection_name);
        let body = serde_json::to_vec(&schema).expect("unable to serialize ");

        let response_body = self.client.post(&path, body).await?.into_body();

        let response: D = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }

    /// Upsert a document
    pub async fn upsert<D: DocumentTrait>(&self, schema: D) -> Result<D> {
        let path = format!(
            "/collections/{}/documents?action=upsert",
            self.collection_name
        );
        let body = serde_json::to_vec(&schema).expect("unable to serialize ");

        let response_body = self.client.post(&path, body).await?.into_body();

        let response: D = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }

    /// Delete documents by filter
    pub async fn delete(&self, filter_by: impl AsRef<str>) -> Result<u64> {
        use urlencoding::encode;

        let path = format!(
            "/collections/{}/documents?filter_by={}",
            self.collection_name, encode(filter_by.as_ref())
        );

        println!("{:?}", path);

        let response_body = self.client.delete(&path).await?.into_body();

        let response: serde_json::Value = serde_json::from_slice(&response_body).unwrap();

        Ok(response["num_deleted"].as_u64().unwrap())
    }
}

/// Client for the Typesense DocumentAPI
pub struct DocumentClient<'a, T> {
    pub(crate) client: Client<T>,
    pub(crate) collection_name: &'a str,
    pub(crate) document_id: &'a str,
}

impl<'a, T> DocumentClient<'a, T>
where
    T: HttpLowLevel,
{
    /// Retrieve the details of the Document given a collection name and Document Id
    pub async fn retrieve<D: DocumentTrait>(&self) -> Result<D> {
        let path = format!(
            "/collections/{}/documents/{}",
            self.collection_name, self.document_id
        );
        let response_body = self.client.get(&path).await?.into_body();
        let response: D = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }

    /// Update the details of a Document given a collection name and Dovument Id
    pub async fn update<D: DocumentTrait>(&self, schema: impl DocumentTrait) -> Result<D> {
        let path = format!(
            "/collections/{}/documents/{}",
            self.collection_name, self.document_id
        );

        let body = serde_json::to_vec(&schema).expect("unable to serialize ");
        let response_body = self.client.patch(&path, body).await?.into_body();
        let response: D = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }

    /// Permanently drops a collection. This action cannot be undone.
    /// For large collections, this might have an impact on read latencies.
    pub async fn delete<D: DocumentTrait>(&self) -> Result<D> {
        let path = format!(
            "/collections/{}/documents/{}",
            self.collection_name, self.document_id
        );

        let response_body = self.client.delete(&path).await?.into_body();
        let response: D = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }
}
