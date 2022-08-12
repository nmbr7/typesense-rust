use serde::{Deserialize, Serialize};
use typesense::Document;

#[cfg(all(test, feature = "tokio-rt", not(target_arch = "wasm32")))]
mod hyper_tests {
    use super::*;
    use typesense::transport::HttpLowLevel;
    use typesense::{Client, ClientBuilder};

    #[tokio::test]
    async fn document_create() {
        let host = "http://localhost:8108";
        let api_key = "xyz";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        create_collection(client.clone(), "companies").await;

        let doc_data = Company {
            id: None,
            company_name: "Company3".to_string(),
            country: "India1".to_string(),
            num_employees: 130,
        };

        let mut expected_doc = doc_data.clone();

        let doc_response = client
            .collection("companies")
            .documents()
            .create(doc_data)
            .await
            .unwrap();

        expected_doc.id = doc_response.id.clone();
        assert_eq!(doc_response, expected_doc);
    }

    #[tokio::test]
    async fn document_retrieve() {
        let host = "http://localhost:8108";
        let api_key = "xyz";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        create_collection(client.clone(), "companies").await;

        let doc_data = Company {
            id: None,
            company_name: "Company3".to_string(),
            country: "India1".to_string(),
            num_employees: 130,
        };

        let mut expected_doc = doc_data.clone();

        let doc_response = client
            .collection("companies")
            .documents()
            .create(doc_data)
            .await
            .unwrap();

        expected_doc.id = doc_response.id.clone();

        let result = client
            .collection("companies")
            .document("0")
            .retrieve::<Company>()
            .await
            .unwrap();

        assert_eq!(result, expected_doc);
    }

    #[tokio::test]
    async fn document_delete_by_filter() {
        let host = "http://localhost:8108";
        let api_key = "xyz";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        create_collection(client.clone(), "companies").await;

        let doc_data = Company {
            id: None,
            company_name: "Company3".to_string(),
            country: "India1".to_string(),
            num_employees: 10,
        };

        let mut expected_doc = doc_data.clone();

        let doc_response = client
            .collection("companies")
            .documents()
            .create(doc_data)
            .await
            .unwrap();

        let num_deleted = client
            .collection("companies")
            .documents()
            .delete("num_employees:<100")
            .await
            .unwrap();

        assert_eq!(num_deleted, 1);
    }

    #[tokio::test]
    async fn document_upsert() {
        let host = "http://localhost:8108";
        let api_key = "xyz";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        create_collection(client.clone(), "companies").await;

        let doc_data = Company {
            id: None,
            company_name: "Company3".to_string(),
            country: "India1".to_string(),
            num_employees: 130,
        };

        let mut expected_doc = doc_data.clone();

        let doc_response = client
            .collection("companies")
            .documents()
            .upsert(doc_data)
            .await
            .unwrap();

        expected_doc.id = doc_response.id.clone();
        assert_eq!(doc_response, expected_doc);
    }

    async fn create_collection<T>(client: Client<T>, collection_name: &str)
    where
        T: Clone + HttpLowLevel,
    {
        match client.collection(collection_name).delete().await {
            Ok(doc) => println!("Collection deleted, TotalDocs: {}", doc.num_documents),
            Err(e) => println!("Delete API returned: {:?}", e),
        };

        client.collections().create::<Company>().await.unwrap();
    }
}

#[allow(dead_code)]
#[derive(Document, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[typesense(default_sorting_field = "num_employees")]
#[typesense(collection_name = "companies")]
struct Company {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    company_name: String,
    num_employees: i32,
    #[typesense(facet)]
    country: String,
}
