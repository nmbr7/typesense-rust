//! # Collection
//!
//! In Typesense, a group of related documents is called a collection. A collection
//! is roughly equivalent to a table in a relational database.
//!

use serde::{Deserialize, Serialize};
mod schema;
pub use schema::{CollectionSchema, CollectionSchemaBuilder};

use crate::client::Client;
use crate::document_trait::Document as DocumentTrait;
use crate::document::{DocumentClient, DocumentsClient};
use crate::transport::HttpLowLevel;
use crate::Result;

/// Client for the Typesense CollectionAPI
pub struct CollectionsClient<T> {
    pub(crate) client: Client<T>,
}

impl<T> CollectionsClient<T>
where
    T: HttpLowLevel,
{
    /// Create a collection in Typesense for a [`DocumentTrait`] type.
    pub async fn create<D: DocumentTrait>(&self) -> Result<CollectionResponse> {
        let schema = D::collection_schema();
        self.create_from_schema(schema).await
    }

    /// Create a Collection in Typesense given a ['CollectionSchema`]
    pub async fn create_from_schema(&self, schema: CollectionSchema) -> Result<CollectionResponse> {
        let body = serde_json::to_vec(&schema).expect("unable to serialize ");
        
        let response_body = self
            .client
            .post("/collections", body)
            .await
            .unwrap()
            .into_body();

        let response: CollectionResponse = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }

    /// Retrieve all the collections
    pub async fn retrieve(&self) -> Result<Vec<CollectionResponse>> {
        let response_body = self.client.get("/collections").await.unwrap().into_body();

        let response: Vec<CollectionResponse> = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }
}

/// Client for the Typesense CollectionAPI
pub struct CollectionClient<'a, T> {
    pub(crate) client: Client<T>,
    pub(crate) collection_name: &'a str,
}

impl<'a, T> CollectionClient<'a, T>
where
    T: HttpLowLevel + Clone,
{
    /// Retrieve the details of the collection given a collection name
    pub async fn retrieve(&self) -> Result<CollectionResponse> {
        let path = format!("/collections/{}", self.collection_name);

        let response_body = self.client.get(&path).await.unwrap().into_body();

        let response: CollectionResponse = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }

    /// Permanently drops a collection. This action cannot be undone.
    /// For large collections, this might have an impact on read latencies.
    pub async fn delete(&self) -> Result<CollectionResponse> {
        let path = format!("/collections/{}", self.collection_name);

        let response_body = self.client.delete(&path).await?.into_body();
        let response: CollectionResponse = serde_json::from_slice(&response_body).unwrap();

        Ok(response)
    }
    
    /// Creates a [`Documents`] type to interact with the Typesense Document API
    pub fn documents(&self) -> DocumentsClient<'a, T> {
        DocumentsClient {
            client: self.client.clone(),
            collection_name: self.collection_name, 
        }
    }
    
    /// Creates a [`Document`] type to interact with the Typesense Document API
    pub fn document(&self, document_id: &'a str) -> DocumentClient<'a, T> {
        DocumentClient {
            client: self.client.clone(),
            collection_name: self.collection_name,
            document_id,        
        }
    }
}

/// Represents a Response from the Typesense Collection API.
#[derive(Deserialize, Serialize, Debug)]
pub struct CollectionResponse {
    /// schema of the collection stored in Typesense
    #[serde(flatten)]
    pub schema: CollectionSchema,
    /// current number of documents in Typesense
    pub num_documents: usize,
}
