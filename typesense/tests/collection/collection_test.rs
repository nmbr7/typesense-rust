use serde::{Deserialize, Serialize};
use typesense::document_trait::Document as DocumentTrait;
use typesense::ClientBuilder;
use typesense::Document;

//#[cfg(all(test, feature = "tokio-rt", not(target_arch = "wasm32")))]
mod hyper_tests {
    use typesense::document;

    use super::*;
    #[tokio::test]
    async fn collection_create() {
        let host = "http://localhost:8108";
        let api_key = "xyz";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        let collection_client = client.collections();

        let collection_schema_response = collection_client.create::<Company>().await.unwrap();

        let doc_data = Company {
            id: String::new(),
            company_name: "Company3".to_string(),
            country: "India1".to_string(),
            num_employees: 13,
        };

        let expected_doc = doc_data.clone();

        let doc_response = client
            .collection("companies")
            .documents()
            .create(doc_data)
            .await
            .unwrap();

        assert_eq!(doc_response.company_name, expected_doc.company_name);
        assert_eq!(collection_schema_response.num_documents, 0);
        assert_eq!(
            collection_schema_response.schema,
            Company::collection_schema()
        );
    }

    #[tokio::test]
    async fn collection_retrieve() {
        let host = "http://localhost:8108";
        let api_key = "xyz";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        let aa = String::from("resfd");
        let collection_schema_response = client.collection(aa.as_str()).retrieve().await.unwrap();

        // let doc: Company = client
        //     .collection("companies")
        //     .document("0")
        //     .delete()
        //     .await
        //     .unwrap();

        //println!("{:?}", doc);
        assert_eq!(collection_schema_response.num_documents, 0);
        assert_eq!(
            collection_schema_response.schema,
            Company::collection_schema()
        );
    }

    #[tokio::test]
    async fn collection_delete() {
        let host = "http://localhost:8108";
        let api_key = "xyz";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        let collection_client = client.collection("companies");

        match collection_client.delete().await {
            Ok(doc) => {
                //assert_eq!(collection_schema_response.num_documents, 1200);
                assert_eq!(
                    doc.schema,
                    Company::collection_schema()
                );
            }
            Err(e) => println!("Error: {:?}", e),
        };
    }

    #[tokio::test]
    async fn collection_retrieve_all() {
        let host = "http://localhost:8108";
        let api_key = "xyz";

        let client = ClientBuilder::new_hyper()
            .host(host)
            .api_key(api_key)
            .build()
            .unwrap();

        let collection_client = client.collections();

        let collection_schema_response = collection_client.retrieve().await.unwrap();
        println!("{:?}", collection_schema_response);

        assert_eq!(collection_schema_response.len(), 3);
    }
}

#[allow(dead_code)]
#[derive(Document, Serialize, Deserialize, Clone, Debug)]
#[typesense(default_sorting_field = "num_employees")]
#[typesense(collection_name = "companies")]
struct Company {
    id: String,
    company_name: String,
    num_employees: i32,
    #[typesense(facet)]
    country: String,
}
